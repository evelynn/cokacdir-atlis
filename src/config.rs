use std::fs;
use std::io;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// Panel-specific settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelSettings {
    #[serde(default)]
    pub start_path: Option<String>,
    #[serde(default = "default_sort_by")]
    pub sort_by: String,
    #[serde(default = "default_sort_order")]
    pub sort_order: String,
}

fn default_sort_by() -> String {
    "name".to_string()
}

fn default_sort_order() -> String {
    "asc".to_string()
}

impl Default for PanelSettings {
    fn default() -> Self {
        Self {
            start_path: None,
            sort_by: default_sort_by(),
            sort_order: default_sort_order(),
        }
    }
}

/// Theme settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeSettings {
    #[serde(default = "default_theme_name")]
    pub name: String,
}

fn default_theme_name() -> String {
    "light".to_string()
}

impl Default for ThemeSettings {
    fn default() -> Self {
        Self {
            name: default_theme_name(),
        }
    }
}

/// Application settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub left_panel: PanelSettings,
    #[serde(default)]
    pub right_panel: PanelSettings,
    #[serde(default = "default_active_panel")]
    pub active_panel: String,
    #[serde(default)]
    pub theme: ThemeSettings,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tar_path: Option<String>,
}

fn default_active_panel() -> String {
    "left".to_string()
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            left_panel: PanelSettings::default(),
            right_panel: PanelSettings::default(),
            active_panel: default_active_panel(),
            theme: ThemeSettings::default(),
            tar_path: None,
        }
    }
}

impl Settings {
    /// Returns the config directory path (~/.cokacdir)
    pub fn config_dir() -> Option<PathBuf> {
        dirs::home_dir().map(|h| h.join(".cokacdir"))
    }

    /// Returns the themes directory path (~/.cokacdir/themes)
    pub fn themes_dir() -> Option<PathBuf> {
        Self::config_dir().map(|d| d.join("themes"))
    }

    /// Returns the config file path (~/.cokacdir/settings.json)
    pub fn config_path() -> Option<PathBuf> {
        Self::config_dir().map(|d| d.join("settings.json"))
    }

    /// Ensures config directories and default files exist
    /// Called on app startup to initialize configuration
    pub fn ensure_config_exists() {
        // Create ~/.cokacdir/
        if let Some(config_dir) = Self::config_dir() {
            if !config_dir.exists() {
                if fs::create_dir_all(&config_dir).is_ok() {
                    // Set directory permissions to user-only on Unix
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        let perms = fs::Permissions::from_mode(0o700);
                        let _ = fs::set_permissions(&config_dir, perms);
                    }
                }
            }
        }

        // Create ~/.cokacdir/themes/
        if let Some(themes_dir) = Self::themes_dir() {
            if !themes_dir.exists() {
                let _ = fs::create_dir_all(&themes_dir);
            }

            // Create default light.json if not exists
            let light_theme_path = themes_dir.join("light.json");
            if !light_theme_path.exists() {
                let _ = fs::write(&light_theme_path, DEFAULT_LIGHT_THEME);
            }

            // Create default dark.json if not exists
            let dark_theme_path = themes_dir.join("dark.json");
            if !dark_theme_path.exists() {
                let _ = fs::write(&dark_theme_path, DEFAULT_DARK_THEME);
            }
        }

        // Create default settings.json if not exists
        if let Some(config_path) = Self::config_path() {
            if !config_path.exists() {
                let default_settings = Self::default();
                let _ = default_settings.save();
            }
        }
    }

    /// Loads settings from the config file, returns default if not found or invalid
    pub fn load() -> Self {
        Self::load_with_error().unwrap_or_default()
    }

    /// Loads settings from the config file with error information
    /// Returns Ok(settings) on success, Err(error_message) on failure
    pub fn load_with_error() -> Result<Self, String> {
        // Ensure config directories and files exist
        Self::ensure_config_exists();

        let config_path = Self::config_path()
            .ok_or_else(|| "Could not determine config path".to_string())?;

        let content = fs::read_to_string(&config_path)
            .map_err(|e| format!("Failed to read settings file: {}", e))?;

        serde_json::from_str(&content)
            .map_err(|e| format!("Invalid JSON in settings.json: {}", e))
    }

    /// Saves settings to the config file using atomic write pattern
    pub fn save(&self) -> io::Result<()> {
        let Some(config_dir) = Self::config_dir() else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Could not determine config directory",
            ));
        };

        // Create config directory if it doesn't exist
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)?;
            // Set directory permissions to user-only on Unix
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let perms = fs::Permissions::from_mode(0o700);
                let _ = fs::set_permissions(&config_dir, perms);
            }
        }

        let config_path = config_dir.join("settings.json");
        let temp_path = config_dir.join("settings.json.tmp");
        let content = serde_json::to_string_pretty(self)?;

        // Atomic write: write to temp file first, then rename
        fs::write(&temp_path, &content)?;
        fs::rename(&temp_path, &config_path)?;

        Ok(())
    }

    /// Returns a valid start path for the left panel
    /// Falls back to current directory, then home directory, then root
    pub fn left_start_path(&self) -> PathBuf {
        self.resolve_path(&self.left_panel.start_path, || {
            std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"))
        })
    }

    /// Returns a valid start path for the right panel
    /// Falls back to home directory, then root
    pub fn right_start_path(&self) -> PathBuf {
        self.resolve_path(&self.right_panel.start_path, || {
            dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"))
        })
    }

    /// Resolves a path setting to a valid directory
    /// Security: Only accepts absolute paths and canonicalizes to resolve symlinks
    fn resolve_path<F>(&self, path_opt: &Option<String>, fallback: F) -> PathBuf
    where
        F: FnOnce() -> PathBuf,
    {
        if let Some(path_str) = path_opt {
            let path = PathBuf::from(path_str);

            // Security: Reject relative paths to prevent path traversal
            if !path.is_absolute() {
                return fallback();
            }

            // Canonicalize to resolve symlinks and verify the path exists
            if let Ok(canonical) = path.canonicalize() {
                if canonical.is_dir() {
                    return canonical;
                }
            }

            // If canonicalize fails, try parent directories
            let mut current = path;
            while let Some(parent) = current.parent() {
                if let Ok(canonical_parent) = parent.canonicalize() {
                    if canonical_parent.is_dir() {
                        return canonical_parent;
                    }
                }
                if parent == current {
                    break;
                }
                current = parent.to_path_buf();
            }
        }
        fallback()
    }
}

/// Default light theme JSON content
const DEFAULT_LIGHT_THEME: &str = r#"{
  "name": "light",
  "palette": {
    "bg": 255, "bg_alt": 254, "fg": 243, "fg_dim": 251,
    "fg_strong": 238, "fg_inverse": 231, "accent": 21,
    "shortcut": 74, "positive": 34, "highlight": 198
  },
  "state": { "success": 34, "warning": 198, "error": 198, "info": 21 },
  "panel": {
    "bg": 255, "border": 251, "border_active": 238,
    "header_bg": 254, "header_bg_active": 253, "header_text": 249,
    "file_text": 243, "directory_text": 67, "selected_bg": 67,
    "selected_text": 231, "marked_text": 198, "size_text": 251, "date_text": 251
  },
  "header": { "bg": 255, "text": 243, "title": 238 },
  "status_bar": { "bg": 253, "text": 249, "text_dim": 251 },
  "function_bar": { "bg": 255, "key": 243, "label": 251 },
  "message": { "bg": 255, "text": 198 },
  "dialog": {
    "bg": 255, "border": 238, "title": 238, "text": 243, "text_dim": 251,
    "message_text": 243, "input_text": 243, "input_cursor_fg": 255,
    "input_cursor_bg": 238, "input_prompt": 74, "button_text": 251,
    "button_selected_bg": 67, "button_selected_text": 231,
    "autocomplete_bg": 255, "autocomplete_text": 243,
    "autocomplete_directory_text": 67, "autocomplete_selected_bg": 67,
    "autocomplete_selected_text": 231, "autocomplete_scroll_info": 251,
    "preview_suffix_text": 251, "help_key_text": 74, "help_label_text": 251,
    "progress_label_text": 251, "progress_value_text": 243,
    "progress_bar_fill": 67, "progress_bar_empty": 251,
    "progress_percent_text": 243, "conflict_filename_text": 198,
    "conflict_count_text": 251, "conflict_shortcut_text": 74,
    "tar_exclude_title": 238, "tar_exclude_border": 238, "tar_exclude_bg": 255,
    "tar_exclude_message_text": 243, "tar_exclude_path_text": 208,
    "tar_exclude_scroll_info": 251, "tar_exclude_button_text": 251,
    "tar_exclude_button_selected_bg": 67, "tar_exclude_button_selected_text": 231
  },
  "settings": {
    "bg": 255, "border": 238, "title": 238, "label_text": 243,
    "prompt": 74, "value_text": 231, "value_bg": 67,
    "help_key": 74, "help_text": 251
  },
  "editor": {
    "bg": 255, "border": 238, "header_bg": 253, "header_text": 249,
    "header_info": 251, "line_number": 251, "text": 243, "cursor": 238,
    "selection_bg": 67, "selection_text": 231, "match_bg": 198,
    "match_current_bg": 208, "bracket_match": 74, "modified_mark": 198,
    "footer_bg": 253, "footer_key": 74, "footer_text": 251,
    "find_input_text": 243, "find_option": 251, "find_option_active": 74
  },
  "viewer": {
    "bg": 255, "border": 238, "header_text": 249, "line_number": 251,
    "text": 243, "search_input_text": 67, "search_cursor_fg": 255,
    "search_cursor_bg": 67, "search_match_current_bg": 67,
    "search_match_current_fg": 255, "search_match_other_bg": 243,
    "search_match_other_fg": 255, "search_info": 251, "hex_offset": 251,
    "hex_bytes": 243, "hex_ascii": 238, "wrap_indicator": 248,
    "footer_key": 74, "footer_text": 251
  },
  "process_manager": {
    "bg": 255, "border": 238, "header_text": 249, "column_header": 21,
    "text": 243, "selected_bg": 67, "selected_text": 231, "cpu_high": 198,
    "mem_high": 198, "confirm_text": 198, "footer_key": 74, "footer_text": 251
  },
  "ai_screen": {
    "bg": 255, "history_border": 238, "history_title": 238,
    "history_placeholder": 251, "history_scroll_info": 251, "user_prefix": 67,
    "assistant_prefix": 74, "error_prefix": 198, "system_prefix": 251,
    "message_text": 243, "input_border": 238, "input_prompt": 74,
    "input_text": 243, "input_cursor": 238, "input_placeholder": 251,
    "processing_spinner": 74, "processing_text": 251, "error_text": 198,
    "footer_key": 74, "footer_text": 251
  },
  "system_info": {
    "bg": 255, "border": 238, "section_title": 34, "label": 243, "value": 243,
    "bar_fill": 34, "bar_empty": 251, "disk_header": 21, "disk_text": 243,
    "selected_bg": 67, "selected_text": 231, "footer_key": 74, "footer_text": 251
  },
  "search_result": {
    "bg": 255, "border": 238, "header_text": 249, "column_header": 21,
    "directory_text": 238, "file_text": 243, "selected_bg": 67,
    "selected_text": 231, "match_highlight": 198, "path_text": 251,
    "footer_key": 74, "footer_text": 251
  },
  "image_viewer": {
    "bg": 255, "border": 238, "title_text": 238,
    "loading_spinner": 74, "loading_text": 251,
    "error_text": 198, "hint_text": 251,
    "footer_key": 74, "footer_text": 251, "footer_separator": 251
  },
  "file_info": {
    "bg": 255, "border": 238, "title": 238, "label": 251, "value": 243,
    "value_name": 67, "value_path": 243, "value_type": 243, "value_size": 67,
    "value_permission": 243, "value_owner": 243, "value_date": 243,
    "calculating_spinner": 74, "calculating_text": 74, "error_text": 198,
    "hint_text": 251
  },
  "help": {
    "bg": 255, "border": 238, "title": 238, "section_title": 67,
    "section_decorator": 251, "key": 74, "key_highlight": 74,
    "description": 243, "hint_text": 251
  },
  "advanced_search": {
    "bg": 255, "border": 238, "title": 238, "label": 243, "input_text": 243,
    "input_cursor": 238, "checkbox_checked": 34, "checkbox_unchecked": 251,
    "button_text": 251, "button_selected_bg": 67, "button_selected_text": 231,
    "footer_key": 74, "footer_text": 251
  },
  "legacy": {
    "bg": 255, "bg_panel": 255, "bg_selected": 67, "bg_header": 254,
    "bg_header_active": 253, "bg_status_bar": 253, "text": 243, "text_dim": 251,
    "text_bold": 243, "text_selected": 231, "text_header": 249,
    "text_header_active": 242, "text_directory": 67, "border": 251,
    "border_active": 238, "success": 34, "warning": 198, "error": 198,
    "info": 21, "shortcut_key": 249
  }
}
"#;

/// Default dark theme JSON content
const DEFAULT_DARK_THEME: &str = r#"{
  "name": "dark",
  "palette": {
    "bg": 236, "bg_alt": 238, "fg": 253, "fg_dim": 245,
    "fg_strong": 255, "fg_inverse": 236, "accent": 117,
    "shortcut": 81, "positive": 84, "highlight": 212
  },
  "state": { "success": 84, "warning": 214, "error": 204, "info": 117 },
  "panel": {
    "bg": 236, "border": 241, "border_active": 253,
    "header_bg": 238, "header_bg_active": 240, "header_text": 250,
    "file_text": 253, "directory_text": 141, "selected_bg": 61,
    "selected_text": 255, "marked_text": 212, "size_text": 245, "date_text": 245
  },
  "header": { "bg": 236, "text": 253, "title": 255 },
  "status_bar": { "bg": 238, "text": 250, "text_dim": 245 },
  "function_bar": { "bg": 236, "key": 253, "label": 245 },
  "message": { "bg": 236, "text": 212 },
  "dialog": {
    "bg": 238, "border": 253, "title": 255, "text": 253, "text_dim": 245,
    "message_text": 253, "input_text": 253, "input_cursor_fg": 236,
    "input_cursor_bg": 253, "input_prompt": 81, "button_text": 245,
    "button_selected_bg": 61, "button_selected_text": 255,
    "autocomplete_bg": 238, "autocomplete_text": 253,
    "autocomplete_directory_text": 141, "autocomplete_selected_bg": 61,
    "autocomplete_selected_text": 255, "autocomplete_scroll_info": 245,
    "preview_suffix_text": 245, "help_key_text": 81, "help_label_text": 245,
    "progress_label_text": 245, "progress_value_text": 253,
    "progress_bar_fill": 61, "progress_bar_empty": 241,
    "progress_percent_text": 253, "conflict_filename_text": 212,
    "conflict_count_text": 245, "conflict_shortcut_text": 81,
    "tar_exclude_title": 255, "tar_exclude_border": 253, "tar_exclude_bg": 238,
    "tar_exclude_message_text": 253, "tar_exclude_path_text": 214,
    "tar_exclude_scroll_info": 245, "tar_exclude_button_text": 245,
    "tar_exclude_button_selected_bg": 61, "tar_exclude_button_selected_text": 255
  },
  "settings": {
    "bg": 238, "border": 253, "title": 255, "label_text": 253,
    "prompt": 81, "value_text": 255, "value_bg": 61,
    "help_key": 81, "help_text": 245
  },
  "editor": {
    "bg": 236, "border": 253, "header_bg": 238, "header_text": 250,
    "header_info": 245, "line_number": 241, "text": 253, "cursor": 255,
    "selection_bg": 61, "selection_text": 255, "match_bg": 212,
    "match_current_bg": 214, "bracket_match": 81, "modified_mark": 212,
    "footer_bg": 238, "footer_key": 81, "footer_text": 245,
    "find_input_text": 253, "find_option": 245, "find_option_active": 81
  },
  "viewer": {
    "bg": 236, "border": 253, "header_text": 250, "line_number": 241,
    "text": 253, "search_input_text": 141, "search_cursor_fg": 236,
    "search_cursor_bg": 141, "search_match_current_bg": 61,
    "search_match_current_fg": 255, "search_match_other_bg": 239,
    "search_match_other_fg": 253, "search_info": 245, "hex_offset": 241,
    "hex_bytes": 253, "hex_ascii": 250, "wrap_indicator": 241,
    "footer_key": 81, "footer_text": 245
  },
  "process_manager": {
    "bg": 236, "border": 253, "header_text": 250, "column_header": 117,
    "text": 253, "selected_bg": 61, "selected_text": 255, "cpu_high": 204,
    "mem_high": 204, "confirm_text": 204, "footer_key": 81, "footer_text": 245
  },
  "ai_screen": {
    "bg": 236, "history_border": 253, "history_title": 255,
    "history_placeholder": 245, "history_scroll_info": 245, "user_prefix": 141,
    "assistant_prefix": 81, "error_prefix": 204, "system_prefix": 245,
    "message_text": 253, "input_border": 253, "input_prompt": 81,
    "input_text": 253, "input_cursor": 255, "input_placeholder": 245,
    "processing_spinner": 81, "processing_text": 245, "error_text": 204,
    "footer_key": 81, "footer_text": 245
  },
  "system_info": {
    "bg": 236, "border": 253, "section_title": 84, "label": 253, "value": 253,
    "bar_fill": 84, "bar_empty": 241, "disk_header": 117, "disk_text": 253,
    "selected_bg": 61, "selected_text": 255, "footer_key": 81, "footer_text": 245
  },
  "search_result": {
    "bg": 236, "border": 253, "header_text": 250, "column_header": 117,
    "directory_text": 255, "file_text": 253, "selected_bg": 61,
    "selected_text": 255, "match_highlight": 212, "path_text": 245,
    "footer_key": 81, "footer_text": 245
  },
  "image_viewer": {
    "bg": 236, "border": 253, "title_text": 255,
    "loading_spinner": 81, "loading_text": 245,
    "error_text": 204, "hint_text": 245,
    "footer_key": 81, "footer_text": 245, "footer_separator": 241
  },
  "file_info": {
    "bg": 238, "border": 253, "title": 255, "label": 245, "value": 253,
    "value_name": 141, "value_path": 253, "value_type": 253, "value_size": 141,
    "value_permission": 253, "value_owner": 253, "value_date": 253,
    "calculating_spinner": 81, "calculating_text": 81, "error_text": 204,
    "hint_text": 245
  },
  "help": {
    "bg": 238, "border": 253, "title": 255, "section_title": 141,
    "section_decorator": 241, "key": 81, "key_highlight": 81,
    "description": 253, "hint_text": 245
  },
  "advanced_search": {
    "bg": 238, "border": 253, "title": 255, "label": 253, "input_text": 253,
    "input_cursor": 255, "checkbox_checked": 84, "checkbox_unchecked": 241,
    "button_text": 245, "button_selected_bg": 61, "button_selected_text": 255,
    "footer_key": 81, "footer_text": 245
  },
  "legacy": {
    "bg": 236, "bg_panel": 236, "bg_selected": 61, "bg_header": 238,
    "bg_header_active": 240, "bg_status_bar": 238, "text": 253, "text_dim": 245,
    "text_bold": 255, "text_selected": 255, "text_header": 250,
    "text_header_active": 253, "text_directory": 141, "border": 241,
    "border_active": 253, "success": 84, "warning": 214, "error": 204,
    "info": 117, "shortcut_key": 250
  }
}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let settings = Settings::default();
        assert_eq!(settings.active_panel, "left");
        assert_eq!(settings.left_panel.sort_by, "name");
        assert_eq!(settings.left_panel.sort_order, "asc");
        assert_eq!(settings.theme.name, "light");
    }

    #[test]
    fn test_parse_partial_json() {
        let json = r#"{"left_panel":{"start_path":"/tmp"}}"#;
        let settings: Settings = serde_json::from_str(json).unwrap();
        assert_eq!(settings.left_panel.start_path, Some("/tmp".to_string()));
        assert_eq!(settings.left_panel.sort_by, "name"); // default
        assert_eq!(settings.right_panel.sort_by, "name"); // default
    }

    #[test]
    fn test_ensure_config_exists() {
        // This test verifies that ensure_config_exists creates required files
        Settings::ensure_config_exists();

        // Check that themes directory exists
        if let Some(themes_dir) = Settings::themes_dir() {
            assert!(themes_dir.exists(), "themes directory should exist");
            let light_theme = themes_dir.join("light.json");
            assert!(light_theme.exists(), "light.json should exist");
        }
    }
}
