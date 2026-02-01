use ratatui::style::{Color, Modifier, Style};
use supports_color::Stream;

// ═══════════════════════════════════════════════════════════════════════════════
// 아이콘 문자
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct ThemeChars {
    pub folder: char,
    pub file: char,
    pub folder_open: char,
    pub parent: char,
}

impl Default for ThemeChars {
    fn default() -> Self {
        Self {
            folder: ' ',
            file: ' ',
            folder_open: ' ',
            parent: ' ',
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// 기본 팔레트 (실제 색상값 정의)
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy)]
pub struct Palette {
    // 명도 기반 (배경/텍스트용)
    pub bg: Color,           // 기본 배경
    pub bg_alt: Color,       // 대체 배경 (헤더, 상태바)
    pub fg: Color,           // 기본 텍스트
    pub fg_dim: Color,       // 흐린 텍스트 (보조 정보)
    pub fg_strong: Color,    // 강조 텍스트 (디렉토리, 제목)
    pub fg_inverse: Color,   // 반전 텍스트 (선택된 항목)

    // 용도 기반 (강조색)
    pub accent: Color,       // 정보성 강조 (컬럼 헤더, 프롬프트)
    pub shortcut: Color,     // 단축키 표시
    pub positive: Color,     // 긍정/성공 (AI 응답, 체크, 진행바)
    pub highlight: Color,    // 강조/경고/에러 (통합)
}

// ═══════════════════════════════════════════════════════════════════════════════
// 상태 색상
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy)]
pub struct StateColors {
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,
}

// ═══════════════════════════════════════════════════════════════════════════════
// 패널 색상 (DualPanel 파일 목록)
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy)]
pub struct PanelColors {
    pub bg: Color,
    pub border: Color,
    pub border_active: Color,
    pub header_bg: Color,
    pub header_bg_active: Color,
    pub header_text: Color,
    pub file_text: Color,
    pub directory_text: Color,
    pub selected_bg: Color,
    pub selected_text: Color,
    pub marked_text: Color,
    pub size_text: Color,
    pub date_text: Color,
}

// ═══════════════════════════════════════════════════════════════════════════════
// 앱 헤더 색상
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy)]
pub struct HeaderColors {
    pub bg: Color,
    pub text: Color,
    pub title: Color,
}

// ═══════════════════════════════════════════════════════════════════════════════
// 상태 표시줄 색상
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy)]
pub struct StatusBarColors {
    pub bg: Color,
    pub text: Color,
    pub text_dim: Color,
}

// ═══════════════════════════════════════════════════════════════════════════════
// 함수 바 색상 (하단 단축키 표시줄)
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy)]
pub struct FunctionBarColors {
    pub bg: Color,
    pub key: Color,
    pub label: Color,
}

// ═══════════════════════════════════════════════════════════════════════════════
// 메시지 색상
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy)]
pub struct MessageColors {
    pub bg: Color,
    pub text: Color,
}

// ═══════════════════════════════════════════════════════════════════════════════
// 대화 상자 색상
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy)]
pub struct DialogColors {
    // === 다이얼로그 프레임 ===
    pub bg: Color,                          // 배경
    pub border: Color,                      // 테두리
    pub title: Color,                       // 제목

    // === 일반 텍스트 ===
    pub text: Color,                        // 일반 텍스트
    pub text_dim: Color,                    // 흐린 텍스트
    pub message_text: Color,                // 메시지 내용

    // === 입력 필드 ===
    pub input_text: Color,                  // 입력 텍스트
    pub input_cursor_fg: Color,             // 커서 전경색
    pub input_cursor_bg: Color,             // 커서 배경색
    pub input_prompt: Color,                // 프롬프트 ">"

    // === 버튼 (확인 다이얼로그) ===
    pub button_text: Color,                 // 일반 버튼 텍스트
    pub button_selected_bg: Color,          // 선택 버튼 배경
    pub button_selected_text: Color,        // 선택 버튼 텍스트

    // === 자동완성 목록 ===
    pub autocomplete_bg: Color,             // 목록 배경
    pub autocomplete_text: Color,           // 파일 텍스트
    pub autocomplete_directory_text: Color, // 디렉토리 텍스트
    pub autocomplete_selected_bg: Color,    // 선택 항목 배경
    pub autocomplete_selected_text: Color,  // 선택 항목 텍스트
    pub autocomplete_scroll_info: Color,    // 스크롤 정보 "[1/10]"
    pub preview_suffix_text: Color,         // 미리보기 접미사

    // === 도움말 라인 ===
    pub help_key_text: Color,               // 단축키 텍스트
    pub help_label_text: Color,             // 설명 텍스트

    // === 진행률 다이얼로그 ===
    pub progress_label_text: Color,         // "File:", "Total:" 레이블
    pub progress_value_text: Color,         // 파일명, 수치
    pub progress_bar_fill: Color,           // 진행바 채움
    pub progress_bar_empty: Color,          // 진행바 빈 부분
    pub progress_percent_text: Color,       // "45%"

    // === 충돌 다이얼로그 ===
    pub conflict_filename_text: Color,      // 강조된 파일명
    pub conflict_count_text: Color,         // "(1 of 3 conflicts)"
    pub conflict_shortcut_text: Color,      // 버튼 단축키 문자 (O, S, A, l)

    // === Tar 제외 확인 다이얼로그 ===
    pub tar_exclude_title: Color,           // 제목
    pub tar_exclude_border: Color,          // 테두리
    pub tar_exclude_bg: Color,              // 배경
    pub tar_exclude_message_text: Color,    // 메시지 텍스트
    pub tar_exclude_path_text: Color,       // 제외 경로 텍스트
    pub tar_exclude_scroll_info: Color,     // 스크롤 정보 "[1-5/10]"
    pub tar_exclude_button_text: Color,     // 버튼 텍스트
    pub tar_exclude_button_selected_bg: Color,   // 선택된 버튼 배경
    pub tar_exclude_button_selected_text: Color, // 선택된 버튼 텍스트
}

// ═══════════════════════════════════════════════════════════════════════════════
// 설정 다이얼로그 색상
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy)]
pub struct SettingsColors {
    pub bg: Color,              // 배경
    pub border: Color,          // 테두리
    pub title: Color,           // 제목
    pub label_text: Color,      // "Theme:" 라벨
    pub prompt: Color,          // ">" 프롬프트
    pub value_text: Color,      // 선택된 값 텍스트
    pub value_bg: Color,        // 선택된 값 배경
    pub help_key: Color,        // 단축키
    pub help_text: Color,       // 단축키 설명
}

// ═══════════════════════════════════════════════════════════════════════════════
// 파일 에디터 색상
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy)]
pub struct EditorColors {
    pub bg: Color,
    pub border: Color,
    pub header_bg: Color,
    pub header_text: Color,
    pub header_info: Color,
    pub line_number: Color,
    pub text: Color,
    pub cursor: Color,
    pub selection_bg: Color,
    pub selection_text: Color,
    pub match_bg: Color,
    pub match_current_bg: Color,
    pub bracket_match: Color,
    pub modified_mark: Color,
    pub footer_bg: Color,
    pub footer_key: Color,
    pub footer_text: Color,
    pub find_input_text: Color,
    pub find_option: Color,
    pub find_option_active: Color,
}

// ═══════════════════════════════════════════════════════════════════════════════
// 파일 뷰어 색상
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy)]
pub struct ViewerColors {
    pub bg: Color,
    pub border: Color,
    pub header_text: Color,
    pub line_number: Color,
    pub text: Color,
    pub search_input_text: Color,
    pub search_cursor_fg: Color,
    pub search_cursor_bg: Color,
    pub search_match_current_bg: Color,
    pub search_match_current_fg: Color,
    pub search_match_other_bg: Color,
    pub search_match_other_fg: Color,
    pub search_info: Color,
    pub hex_offset: Color,
    pub hex_bytes: Color,
    pub hex_ascii: Color,
    pub wrap_indicator: Color,
    pub footer_key: Color,
    pub footer_text: Color,
}

// ═══════════════════════════════════════════════════════════════════════════════
// 프로세스 관리자 색상
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy)]
pub struct ProcessManagerColors {
    pub bg: Color,
    pub border: Color,
    pub header_text: Color,
    pub column_header: Color,
    pub text: Color,
    pub selected_bg: Color,
    pub selected_text: Color,
    pub cpu_high: Color,
    pub mem_high: Color,
    pub confirm_text: Color,
    pub footer_key: Color,
    pub footer_text: Color,
}

// ═══════════════════════════════════════════════════════════════════════════════
// AI 화면 색상
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy)]
pub struct AIScreenColors {
    // === 배경 ===
    pub bg: Color,

    // === 히스토리 영역 ===
    pub history_border: Color,              // 히스토리 영역 테두리
    pub history_title: Color,               // 히스토리 제목 (경로 + 세션)
    pub history_placeholder: Color,         // 빈 상태 플레이스홀더
    pub history_scroll_info: Color,         // 스크롤 정보 "[1/10]"

    // === 메시지 프리픽스 (아이콘) ===
    pub user_prefix: Color,                 // "> " 사용자 메시지
    pub assistant_prefix: Color,            // "< " AI 응답
    pub error_prefix: Color,                // "! " 에러
    pub system_prefix: Color,               // "* " 시스템

    // === 메시지 내용 ===
    pub message_text: Color,                // 일반 메시지 텍스트

    // === 입력 영역 ===
    pub input_border: Color,                // 입력 영역 테두리
    pub input_prompt: Color,                // "> " 입력 프롬프트
    pub input_text: Color,                  // 입력 텍스트
    pub input_cursor: Color,                // 커서
    pub input_placeholder: Color,           // 플레이스홀더

    // === 처리 중 상태 ===
    pub processing_spinner: Color,          // 스피너
    pub processing_text: Color,             // "Processing..." 텍스트

    // === 에러 상태 ===
    pub error_text: Color,                  // "Claude CLI not available"

    // === 하단 도움말 ===
    pub footer_key: Color,                  // 단축키 텍스트
    pub footer_text: Color,                 // 설명 텍스트
}

// ═══════════════════════════════════════════════════════════════════════════════
// 시스템 정보 색상
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy)]
pub struct SystemInfoColors {
    pub bg: Color,
    pub border: Color,
    pub section_title: Color,
    pub label: Color,
    pub value: Color,
    pub bar_fill: Color,
    pub bar_empty: Color,
    pub disk_header: Color,
    pub disk_text: Color,
    pub selected_bg: Color,
    pub selected_text: Color,
    pub footer_key: Color,
    pub footer_text: Color,
}

// ═══════════════════════════════════════════════════════════════════════════════
// 검색 결과 색상
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy)]
pub struct SearchResultColors {
    pub bg: Color,
    pub border: Color,
    pub header_text: Color,
    pub column_header: Color,
    pub directory_text: Color,
    pub file_text: Color,
    pub selected_bg: Color,
    pub selected_text: Color,
    pub match_highlight: Color,
    pub path_text: Color,
    pub footer_key: Color,
    pub footer_text: Color,
}

// ═══════════════════════════════════════════════════════════════════════════════
// 이미지 뷰어 색상
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy)]
pub struct ImageViewerColors {
    // === 프레임 ===
    pub bg: Color,                    // 배경 (빈 영역)
    pub border: Color,                // 테두리
    pub title_text: Color,            // 제목 텍스트 (파일명, 해상도, 줌)

    // === 로딩 상태 ===
    pub loading_spinner: Color,       // 로딩 스피너
    pub loading_text: Color,          // "Loading image..." 텍스트

    // === 에러 상태 ===
    pub error_text: Color,            // 에러 메시지
    pub hint_text: Color,             // "Press ESC to close" 힌트

    // === 하단 도움말 ===
    pub footer_key: Color,            // 단축키 (PgUp, +, -, r, Esc)
    pub footer_text: Color,           // 설명 (Prev/Next, Zoom, Pan)
    pub footer_separator: Color,      // 구분자 (/)
}

// ═══════════════════════════════════════════════════════════════════════════════
// 파일 정보 색상
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy)]
pub struct FileInfoColors {
    // === 다이얼로그 프레임 ===
    pub bg: Color,
    pub border: Color,
    pub title: Color,

    // === 정보 표시 ===
    pub label: Color,               // 라벨 (Name, Path, Type 등)
    pub value: Color,               // 기본 값
    pub value_name: Color,          // 파일/폴더 이름
    pub value_path: Color,          // 경로
    pub value_type: Color,          // 파일 타입
    pub value_size: Color,          // 크기 (숫자)
    pub value_permission: Color,    // 권한
    pub value_owner: Color,         // 소유자/그룹
    pub value_date: Color,          // 날짜/시간

    // === 상태 표시 ===
    pub calculating_spinner: Color, // 계산 중 스피너
    pub calculating_text: Color,    // "Calculating..." 텍스트
    pub error_text: Color,          // 에러 메시지

    // === 하단 도움말 ===
    pub hint_text: Color,           // 도움말 텍스트
}

// ═══════════════════════════════════════════════════════════════════════════════
// 도움말 화면 색상
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy)]
pub struct HelpColors {
    // === 다이얼로그 프레임 ===
    pub bg: Color,
    pub border: Color,
    pub title: Color,

    // === 섹션 ===
    pub section_title: Color,       // 섹션 제목 (Navigation, Tools 등)
    pub section_decorator: Color,   // 섹션 데코레이터 ("──")

    // === 단축키 목록 ===
    pub key: Color,                 // 단축키 텍스트
    pub key_highlight: Color,       // 강조 단축키 (첫 글자)
    pub description: Color,         // 설명 텍스트

    // === 하단 도움말 ===
    pub hint_text: Color,           // "Press any key to close"
}

// ═══════════════════════════════════════════════════════════════════════════════
// 고급 검색 색상
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy)]
pub struct AdvancedSearchColors {
    pub bg: Color,
    pub border: Color,
    pub title: Color,
    pub label: Color,
    pub input_text: Color,
    pub input_cursor: Color,
    pub checkbox_checked: Color,
    pub checkbox_unchecked: Color,
    pub button_text: Color,
    pub button_selected_bg: Color,
    pub button_selected_text: Color,
    pub footer_key: Color,
    pub footer_text: Color,
}

// ═══════════════════════════════════════════════════════════════════════════════
// 메인 Theme 구조체
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Clone)]
#[allow(dead_code)]
pub struct Theme {
    // 기본 팔레트
    pub palette: Palette,

    // 상태 색상
    pub state: StateColors,

    // UI 컴포넌트별 색상
    pub panel: PanelColors,
    pub header: HeaderColors,
    pub status_bar: StatusBarColors,
    pub function_bar: FunctionBarColors,
    pub message: MessageColors,
    pub dialog: DialogColors,
    pub settings: SettingsColors,
    pub editor: EditorColors,
    pub viewer: ViewerColors,
    pub process_manager: ProcessManagerColors,
    pub ai_screen: AIScreenColors,
    pub system_info: SystemInfoColors,
    pub search_result: SearchResultColors,
    pub image_viewer: ImageViewerColors,
    pub file_info: FileInfoColors,
    pub help: HelpColors,
    pub advanced_search: AdvancedSearchColors,

    // 아이콘 문자
    pub chars: ThemeChars,

    // ═══ 하위 호환성을 위한 기존 필드 (deprecated) ═══
    pub bg: Color,
    pub bg_panel: Color,
    pub bg_selected: Color,
    pub bg_header: Color,
    pub bg_header_active: Color,
    pub bg_status_bar: Color,
    pub text: Color,
    pub text_dim: Color,
    pub text_bold: Color,
    pub text_selected: Color,
    pub text_header: Color,
    pub text_header_active: Color,
    pub text_directory: Color,
    pub border: Color,
    pub border_active: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,
    pub shortcut_key: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self::light()
    }
}

impl Theme {
    /// Load theme by name from ~/.cokacdir/themes/{name}.json
    /// Falls back to built-in theme if file not found
    pub fn load(name: &str) -> Self {
        // Try to load from JSON file first
        if let Some(theme) = super::theme_loader::load_theme(name) {
            return theme;
        }
        // Fall back to built-in themes
        match name {
            "light" => Self::light(),
            "dark" => Self::dark(),
            _ => Self::light(),
        }
    }

    /// Check if terminal supports true color (24-bit RGB)
    #[allow(dead_code)]
    fn supports_true_color() -> bool {
        if let Some(support) = supports_color::on(Stream::Stdout) {
            support.has_16m
        } else {
            false
        }
    }

    /// Light theme (default)
    pub fn light() -> Self {
        Self::light_256()
    }

    fn light_256() -> Self {
        // 기본 팔레트 정의
        let palette = Palette {
            // 명도 기반
            bg: Color::Indexed(255),             // 기본 배경
            bg_alt: Color::Indexed(254),         // 대체 배경
            fg: Color::Indexed(243),             // 기본 텍스트
            fg_dim: Color::Indexed(251),         // 흐린 텍스트
            fg_strong: Color::Indexed(238),      // 강조 텍스트
            fg_inverse: Color::Indexed(231),     // 반전 텍스트

            // 용도 기반
            accent: Color::Indexed(21),          // 정보성 강조
            shortcut: Color::Indexed(74),        // 단축키
            positive: Color::Indexed(34),        // 긍정/성공
            highlight: Color::Indexed(198),      // 강조/경고/에러
        };

        // 상태 색상
        let state = StateColors {
            success: Color::Indexed(34),
            warning: Color::Indexed(198),
            error: Color::Indexed(198),
            info: Color::Indexed(21),
        };

        // 패널 색상
        let panel = PanelColors {
            bg: Color::Indexed(255),
            border: Color::Indexed(251),
            border_active: Color::Indexed(238),
            header_bg: Color::Indexed(254),
            header_bg_active: Color::Indexed(253),
            header_text: Color::Indexed(249),
            file_text: Color::Indexed(243),
            directory_text: Color::Indexed(67),
            selected_bg: Color::Indexed(67),
            selected_text: Color::Indexed(231),
            marked_text: Color::Indexed(198),
            size_text: Color::Indexed(251),
            date_text: Color::Indexed(251),
        };

        // 앱 헤더
        let header = HeaderColors {
            bg: Color::Indexed(255),
            text: Color::Indexed(243),
            title: Color::Indexed(238),
        };

        // 상태 표시줄
        let status_bar = StatusBarColors {
            bg: Color::Indexed(253),
            text: Color::Indexed(249),
            text_dim: Color::Indexed(251),
        };

        // 함수 바
        let function_bar = FunctionBarColors {
            bg: Color::Indexed(255),
            key: Color::Indexed(243),
            label: Color::Indexed(251),
        };

        // 메시지
        let message = MessageColors {
            bg: Color::Indexed(255),
            text: Color::Indexed(198),
        };

        // 대화 상자
        let dialog = DialogColors {
            // === 다이얼로그 프레임 ===
            bg: Color::Indexed(255),
            border: Color::Indexed(238),
            title: Color::Indexed(238),

            // === 일반 텍스트 ===
            text: Color::Indexed(243),
            text_dim: Color::Indexed(251),
            message_text: Color::Indexed(243),

            // === 입력 필드 ===
            input_text: Color::Indexed(243),
            input_cursor_fg: Color::Indexed(255),
            input_cursor_bg: Color::Indexed(238),
            input_prompt: Color::Indexed(74),       // 단축키 색상 (editor.footer_key)

            // === 버튼 ===
            button_text: Color::Indexed(251),
            button_selected_bg: Color::Indexed(67),
            button_selected_text: Color::Indexed(231),

            // === 자동완성 ===
            autocomplete_bg: Color::Indexed(255),
            autocomplete_text: Color::Indexed(243),
            autocomplete_directory_text: Color::Indexed(67),
            autocomplete_selected_bg: Color::Indexed(67),
            autocomplete_selected_text: Color::Indexed(231),
            autocomplete_scroll_info: Color::Indexed(251),
            preview_suffix_text: Color::Indexed(251),

            // === 도움말 ===
            help_key_text: Color::Indexed(74),      // 단축키 색상 (editor.footer_key)
            help_label_text: Color::Indexed(251),

            // === 진행률 ===
            progress_label_text: Color::Indexed(251),
            progress_value_text: Color::Indexed(243),
            progress_bar_fill: Color::Indexed(67),  // 선택 배경색 (panel.selected_bg)
            progress_bar_empty: Color::Indexed(251),
            progress_percent_text: Color::Indexed(243),

            // === 충돌 ===
            conflict_filename_text: Color::Indexed(198),  // 강조된 파일명
            conflict_count_text: Color::Indexed(251),     // 진행 정보
            conflict_shortcut_text: Color::Indexed(117),  // 버튼 단축키 (O, S, A, l)

            // === Tar 제외 확인 ===
            tar_exclude_title: Color::Indexed(238),       // 제목 (dialog.title과 동일)
            tar_exclude_border: Color::Indexed(238),      // 테두리 (dialog.border와 동일)
            tar_exclude_bg: Color::Indexed(255),          // 배경 (dialog.bg와 동일)
            tar_exclude_message_text: Color::Indexed(243), // 메시지 텍스트 (dialog.message_text와 동일)
            tar_exclude_path_text: Color::Indexed(208),   // 제외 경로 (주황색)
            tar_exclude_scroll_info: Color::Indexed(251), // 스크롤 정보
            tar_exclude_button_text: Color::Indexed(251), // 버튼 텍스트 (dialog.button_text와 동일)
            tar_exclude_button_selected_bg: Color::Indexed(67),   // 선택 버튼 배경
            tar_exclude_button_selected_text: Color::Indexed(231), // 선택 버튼 텍스트
        };

        // 설정 다이얼로그
        let settings = SettingsColors {
            bg: Color::Indexed(255),
            border: Color::Indexed(238),
            title: Color::Indexed(238),
            label_text: Color::Indexed(243),
            prompt: Color::Indexed(74),
            value_text: Color::Indexed(231),
            value_bg: Color::Indexed(67),
            help_key: Color::Indexed(74),
            help_text: Color::Indexed(251),
        };

        // 에디터
        let editor = EditorColors {
            bg: Color::Indexed(255),
            border: Color::Indexed(238),
            header_bg: Color::Indexed(253),
            header_text: Color::Indexed(249),
            header_info: Color::Indexed(251),
            line_number: Color::Indexed(251),
            text: Color::Indexed(243),
            cursor: Color::Indexed(238),
            selection_bg: Color::Indexed(67),
            selection_text: Color::Indexed(231),
            match_bg: Color::Indexed(198),
            match_current_bg: Color::Indexed(208),
            bracket_match: Color::Indexed(74),
            modified_mark: Color::Indexed(198),
            footer_bg: Color::Indexed(253),
            footer_key: Color::Indexed(74),
            footer_text: Color::Indexed(251),
            find_input_text: Color::Indexed(243),
            find_option: Color::Indexed(251),
            find_option_active: Color::Indexed(74),
        };

        // 뷰어
        let viewer = ViewerColors {
            bg: Color::Indexed(255),
            border: Color::Indexed(238),
            header_text: Color::Indexed(249),
            line_number: Color::Indexed(251),
            text: Color::Indexed(243),
            search_input_text: Color::Indexed(67),
            search_cursor_fg: Color::Indexed(255),
            search_cursor_bg: Color::Indexed(67),
            search_match_current_bg: Color::Indexed(67),
            search_match_current_fg: Color::Indexed(255),
            search_match_other_bg: Color::Indexed(243),
            search_match_other_fg: Color::Indexed(255),
            search_info: Color::Indexed(251),
            hex_offset: Color::Indexed(251),
            hex_bytes: Color::Indexed(243),
            hex_ascii: Color::Indexed(238),
            wrap_indicator: Color::Indexed(248),
            footer_key: Color::Indexed(74),
            footer_text: Color::Indexed(251),
        };

        // 프로세스 관리자
        let process_manager = ProcessManagerColors {
            bg: Color::Indexed(255),
            border: Color::Indexed(238),
            header_text: Color::Indexed(249),
            column_header: Color::Indexed(21),
            text: Color::Indexed(243),
            selected_bg: Color::Indexed(67),
            selected_text: Color::Indexed(231),
            cpu_high: Color::Indexed(198),
            mem_high: Color::Indexed(198),
            confirm_text: Color::Indexed(198),
            footer_key: Color::Indexed(74),
            footer_text: Color::Indexed(251),
        };

        // AI 화면 (Panel/Viewer/Editor 색상만 사용)
        let ai_screen = AIScreenColors {
            // === 배경 ===
            bg: Color::Indexed(255),                    // 흰색 배경 (editor.bg)

            // === 히스토리 영역 ===
            history_border: Color::Indexed(238),        // 테두리 (editor.border)
            history_title: Color::Indexed(238),         // 제목 (editor.border)
            history_placeholder: Color::Indexed(251),   // 플레이스홀더 (editor.footer_text)
            history_scroll_info: Color::Indexed(251),   // 스크롤 정보 (editor.footer_text)

            // === 메시지 프리픽스 ===
            user_prefix: Color::Indexed(67),            // 사용자 ">" (panel.directory_text)
            assistant_prefix: Color::Indexed(74),       // AI "<" (editor.footer_key)
            error_prefix: Color::Indexed(198),          // 에러 "!" (panel.marked_text)
            system_prefix: Color::Indexed(251),         // 시스템 "*" (editor.footer_text)

            // === 메시지 내용 ===
            message_text: Color::Indexed(243),          // 메시지 텍스트 (editor.text)

            // === 입력 영역 ===
            input_border: Color::Indexed(238),          // 입력 테두리 (editor.border)
            input_prompt: Color::Indexed(74),           // 입력 ">" (editor.footer_key)
            input_text: Color::Indexed(243),            // 입력 텍스트 (editor.text)
            input_cursor: Color::Indexed(238),          // 커서 (editor.cursor)
            input_placeholder: Color::Indexed(251),     // 플레이스홀더 (editor.footer_text)

            // === 처리 중 상태 ===
            processing_spinner: Color::Indexed(74),     // 스피너 (editor.footer_key)
            processing_text: Color::Indexed(251),       // 처리 중 텍스트 (editor.footer_text)

            // === 에러 상태 ===
            error_text: Color::Indexed(198),            // 에러 텍스트 (panel.marked_text)

            // === 하단 도움말 ===
            footer_key: Color::Indexed(74),             // 단축키 (editor.footer_key)
            footer_text: Color::Indexed(251),           // 설명 (editor.footer_text)
        };

        // 시스템 정보
        let system_info = SystemInfoColors {
            bg: Color::Indexed(255),
            border: Color::Indexed(238),
            section_title: Color::Indexed(34),
            label: Color::Indexed(243),
            value: Color::Indexed(243),
            bar_fill: Color::Indexed(34),
            bar_empty: Color::Indexed(251),
            disk_header: Color::Indexed(21),
            disk_text: Color::Indexed(243),
            selected_bg: Color::Indexed(67),
            selected_text: Color::Indexed(231),
            footer_key: Color::Indexed(74),
            footer_text: Color::Indexed(251),
        };

        // 검색 결과
        let search_result = SearchResultColors {
            bg: Color::Indexed(255),
            border: Color::Indexed(238),
            header_text: Color::Indexed(249),
            column_header: Color::Indexed(21),
            directory_text: Color::Indexed(238),
            file_text: Color::Indexed(243),
            selected_bg: Color::Indexed(67),
            selected_text: Color::Indexed(231),
            match_highlight: Color::Indexed(198),
            path_text: Color::Indexed(251),
            footer_key: Color::Indexed(74),
            footer_text: Color::Indexed(251),
        };

        // 이미지 뷰어
        let image_viewer = ImageViewerColors {
            // === 프레임 ===
            bg: Color::Indexed(255),              // 배경
            border: Color::Indexed(238),          // 테두리
            title_text: Color::Indexed(238),      // 제목 텍스트

            // === 로딩 상태 ===
            loading_spinner: Color::Indexed(74),  // 스피너 (shortcut 색상)
            loading_text: Color::Indexed(251),    // 로딩 텍스트

            // === 에러 상태 ===
            error_text: Color::Indexed(198),      // 에러 (highlight 색상)
            hint_text: Color::Indexed(251),       // 힌트 텍스트

            // === 하단 도움말 ===
            footer_key: Color::Indexed(74),       // 단축키 (shortcut 색상)
            footer_text: Color::Indexed(251),     // 설명
            footer_separator: Color::Indexed(251), // 구분자
        };

        // 파일 정보
        let file_info = FileInfoColors {
            // === 다이얼로그 프레임 ===
            bg: Color::Indexed(255),
            border: Color::Indexed(238),
            title: Color::Indexed(238),

            // === 정보 표시 ===
            label: Color::Indexed(251),
            value: Color::Indexed(243),
            value_name: Color::Indexed(67),         // 파일명은 폴더색 (파란)
            value_path: Color::Indexed(243),
            value_type: Color::Indexed(243),
            value_size: Color::Indexed(67),         // 크기는 숫자 강조 (파란)
            value_permission: Color::Indexed(243),
            value_owner: Color::Indexed(243),
            value_date: Color::Indexed(243),

            // === 상태 표시 ===
            calculating_spinner: Color::Indexed(74),
            calculating_text: Color::Indexed(74),
            error_text: Color::Indexed(198),

            // === 하단 도움말 ===
            hint_text: Color::Indexed(251),
        };

        // 도움말
        let help = HelpColors {
            // === 다이얼로그 프레임 ===
            bg: Color::Indexed(255),
            border: Color::Indexed(238),
            title: Color::Indexed(238),

            // === 섹션 ===
            section_title: Color::Indexed(67),      // 섹션 제목 (파란)
            section_decorator: Color::Indexed(251), // 섹션 데코레이터 ("──")

            // === 단축키 목록 ===
            key: Color::Indexed(74),                // 단축키 텍스트 (청록)
            key_highlight: Color::Indexed(74),      // 강조 단축키 (청록)
            description: Color::Indexed(243),       // 설명 텍스트

            // === 하단 도움말 ===
            hint_text: Color::Indexed(251),
        };

        // 고급 검색
        let advanced_search = AdvancedSearchColors {
            bg: Color::Indexed(255),
            border: Color::Indexed(238),
            title: Color::Indexed(238),
            label: Color::Indexed(243),
            input_text: Color::Indexed(243),
            input_cursor: Color::Indexed(238),
            checkbox_checked: Color::Indexed(34),
            checkbox_unchecked: Color::Indexed(251),
            button_text: Color::Indexed(251),
            button_selected_bg: Color::Indexed(67),
            button_selected_text: Color::Indexed(231),
            footer_key: Color::Indexed(74),
            footer_text: Color::Indexed(251),
        };

        Self {
            palette,
            state,
            panel,
            header,
            status_bar,
            function_bar,
            message,
            dialog,
            settings,
            editor,
            viewer,
            process_manager,
            ai_screen,
            system_info,
            search_result,
            image_viewer,
            file_info,
            help,
            advanced_search,
            chars: ThemeChars::default(),

            // 하위 호환성 필드
            bg: Color::Indexed(255),
            bg_panel: Color::Indexed(255),
            bg_selected: Color::Indexed(67),
            bg_header: Color::Indexed(254),
            bg_header_active: Color::Indexed(253),
            bg_status_bar: Color::Indexed(253),
            text: Color::Indexed(243),
            text_dim: Color::Indexed(251),
            text_bold: Color::Indexed(243),
            text_selected: Color::Indexed(231),
            text_header: Color::Indexed(249),
            text_header_active: Color::Indexed(242),
            text_directory: Color::Indexed(67),
            border: Color::Indexed(251),
            border_active: Color::Indexed(238),
            success: Color::Indexed(34),
            warning: Color::Indexed(198),
            error: Color::Indexed(198),
            info: Color::Indexed(21),
            shortcut_key: Color::Indexed(249),
        }
    }

    /// Dark theme
    pub fn dark() -> Self {
        Self::dark_256()
    }

    fn dark_256() -> Self {
        // 기본 팔레트 정의 (어두운 배경)
        let palette = Palette {
            bg: Color::Indexed(235),             // 어두운 배경
            bg_alt: Color::Indexed(236),         // 대체 배경
            fg: Color::Indexed(252),             // 밝은 텍스트
            fg_dim: Color::Indexed(245),         // 흐린 텍스트
            fg_strong: Color::Indexed(255),      // 강조 텍스트
            fg_inverse: Color::Indexed(235),     // 반전 텍스트
            accent: Color::Indexed(81),          // 정보성 강조
            shortcut: Color::Indexed(117),       // 단축키
            positive: Color::Indexed(114),       // 긍정/성공
            highlight: Color::Indexed(204),      // 강조/경고/에러
        };

        let state = StateColors {
            success: Color::Indexed(114),
            warning: Color::Indexed(204),
            error: Color::Indexed(204),
            info: Color::Indexed(81),
        };

        let panel = PanelColors {
            bg: Color::Indexed(235),
            border: Color::Indexed(245),
            border_active: Color::Indexed(252),
            header_bg: Color::Indexed(236),
            header_bg_active: Color::Indexed(237),
            header_text: Color::Indexed(250),
            file_text: Color::Indexed(252),
            directory_text: Color::Indexed(117),
            selected_bg: Color::Indexed(117),
            selected_text: Color::Indexed(235),
            marked_text: Color::Indexed(204),
            size_text: Color::Indexed(245),
            date_text: Color::Indexed(245),
        };

        let header = HeaderColors {
            bg: Color::Indexed(235),
            text: Color::Indexed(252),
            title: Color::Indexed(255),
        };

        let status_bar = StatusBarColors {
            bg: Color::Indexed(237),
            text: Color::Indexed(250),
            text_dim: Color::Indexed(245),
        };

        let function_bar = FunctionBarColors {
            bg: Color::Indexed(235),
            key: Color::Indexed(252),
            label: Color::Indexed(245),
        };

        let message = MessageColors {
            bg: Color::Indexed(235),
            text: Color::Indexed(204),
        };

        let dialog = DialogColors {
            bg: Color::Indexed(236),
            border: Color::Indexed(252),
            title: Color::Indexed(255),
            text: Color::Indexed(252),
            text_dim: Color::Indexed(245),
            message_text: Color::Indexed(252),
            input_text: Color::Indexed(252),
            input_cursor_fg: Color::Indexed(235),
            input_cursor_bg: Color::Indexed(252),
            input_prompt: Color::Indexed(117),
            button_text: Color::Indexed(245),
            button_selected_bg: Color::Indexed(117),
            button_selected_text: Color::Indexed(235),
            autocomplete_bg: Color::Indexed(236),
            autocomplete_text: Color::Indexed(252),
            autocomplete_directory_text: Color::Indexed(117),
            autocomplete_selected_bg: Color::Indexed(117),
            autocomplete_selected_text: Color::Indexed(235),
            autocomplete_scroll_info: Color::Indexed(245),
            preview_suffix_text: Color::Indexed(245),
            help_key_text: Color::Indexed(117),
            help_label_text: Color::Indexed(245),
            progress_label_text: Color::Indexed(245),
            progress_value_text: Color::Indexed(252),
            progress_bar_fill: Color::Indexed(117),
            progress_bar_empty: Color::Indexed(245),
            progress_percent_text: Color::Indexed(252),
            conflict_filename_text: Color::Indexed(204),  // 강조된 파일명
            conflict_count_text: Color::Indexed(245),     // 진행 정보
            conflict_shortcut_text: Color::Indexed(33),   // 버튼 단축키 (O, S, A, l)

            // === Tar 제외 확인 ===
            tar_exclude_title: Color::Indexed(255),       // 제목 (dialog.title과 동일)
            tar_exclude_border: Color::Indexed(252),      // 테두리 (dialog.border와 동일)
            tar_exclude_bg: Color::Indexed(236),          // 배경 (dialog.bg와 동일)
            tar_exclude_message_text: Color::Indexed(252), // 메시지 텍스트 (dialog.message_text와 동일)
            tar_exclude_path_text: Color::Indexed(166),   // 제외 경로 (주황색)
            tar_exclude_scroll_info: Color::Indexed(245), // 스크롤 정보
            tar_exclude_button_text: Color::Indexed(245), // 버튼 텍스트 (dialog.button_text와 동일)
            tar_exclude_button_selected_bg: Color::Indexed(117),  // 선택 버튼 배경
            tar_exclude_button_selected_text: Color::Indexed(235), // 선택 버튼 텍스트
        };

        let settings = SettingsColors {
            bg: Color::Indexed(236),
            border: Color::Indexed(252),
            title: Color::Indexed(255),
            label_text: Color::Indexed(252),
            prompt: Color::Indexed(117),
            value_text: Color::Indexed(235),
            value_bg: Color::Indexed(117),
            help_key: Color::Indexed(117),
            help_text: Color::Indexed(245),
        };

        let editor = EditorColors {
            bg: Color::Indexed(235),
            border: Color::Indexed(252),
            header_bg: Color::Indexed(237),
            header_text: Color::Indexed(250),
            header_info: Color::Indexed(245),
            line_number: Color::Indexed(245),
            text: Color::Indexed(252),
            cursor: Color::Indexed(252),
            selection_bg: Color::Indexed(117),
            selection_text: Color::Indexed(231),
            match_bg: Color::Indexed(204),
            match_current_bg: Color::Indexed(208),
            bracket_match: Color::Indexed(117),
            modified_mark: Color::Indexed(204),
            footer_bg: Color::Indexed(237),
            footer_key: Color::Indexed(117),
            footer_text: Color::Indexed(245),
            find_input_text: Color::Indexed(117),
            find_option: Color::Indexed(245),
            find_option_active: Color::Indexed(117),
        };

        let viewer = ViewerColors {
            bg: Color::Indexed(235),
            border: Color::Indexed(252),
            header_text: Color::Indexed(250),
            line_number: Color::Indexed(245),
            text: Color::Indexed(252),
            search_input_text: Color::Indexed(117),
            search_cursor_fg: Color::Indexed(235),
            search_cursor_bg: Color::Indexed(117),
            search_match_current_bg: Color::Indexed(117),
            search_match_current_fg: Color::Indexed(235),
            search_match_other_bg: Color::Indexed(245),
            search_match_other_fg: Color::Indexed(235),
            search_info: Color::Indexed(245),
            hex_offset: Color::Indexed(245),
            hex_bytes: Color::Indexed(252),
            hex_ascii: Color::Indexed(255),
            wrap_indicator: Color::Indexed(240),
            footer_key: Color::Indexed(117),
            footer_text: Color::Indexed(245),
        };

        let process_manager = ProcessManagerColors {
            bg: Color::Indexed(235),
            border: Color::Indexed(252),
            header_text: Color::Indexed(250),
            column_header: Color::Indexed(81),
            text: Color::Indexed(252),
            selected_bg: Color::Indexed(117),
            selected_text: Color::Indexed(235),
            cpu_high: Color::Indexed(204),
            mem_high: Color::Indexed(204),
            confirm_text: Color::Indexed(204),
            footer_key: Color::Indexed(117),
            footer_text: Color::Indexed(245),
        };

        let ai_screen = AIScreenColors {
            bg: Color::Indexed(235),
            history_border: Color::Indexed(252),
            history_title: Color::Indexed(255),
            history_placeholder: Color::Indexed(245),
            history_scroll_info: Color::Indexed(245),
            user_prefix: Color::Indexed(117),
            assistant_prefix: Color::Indexed(117),
            error_prefix: Color::Indexed(204),
            system_prefix: Color::Indexed(245),
            message_text: Color::Indexed(252),
            input_border: Color::Indexed(252),
            input_prompt: Color::Indexed(117),
            input_text: Color::Indexed(252),
            input_cursor: Color::Indexed(252),
            input_placeholder: Color::Indexed(245),
            processing_spinner: Color::Indexed(117),
            processing_text: Color::Indexed(245),
            error_text: Color::Indexed(204),
            footer_key: Color::Indexed(117),
            footer_text: Color::Indexed(245),
        };

        let system_info = SystemInfoColors {
            bg: Color::Indexed(235),
            border: Color::Indexed(252),
            section_title: Color::Indexed(114),
            label: Color::Indexed(252),
            value: Color::Indexed(252),
            bar_fill: Color::Indexed(114),
            bar_empty: Color::Indexed(245),
            disk_header: Color::Indexed(81),
            disk_text: Color::Indexed(252),
            selected_bg: Color::Indexed(117),
            selected_text: Color::Indexed(235),
            footer_key: Color::Indexed(117),
            footer_text: Color::Indexed(245),
        };

        let search_result = SearchResultColors {
            bg: Color::Indexed(235),
            border: Color::Indexed(252),
            header_text: Color::Indexed(250),
            column_header: Color::Indexed(81),
            directory_text: Color::Indexed(255),
            file_text: Color::Indexed(252),
            selected_bg: Color::Indexed(117),
            selected_text: Color::Indexed(235),
            match_highlight: Color::Indexed(204),
            path_text: Color::Indexed(245),
            footer_key: Color::Indexed(117),
            footer_text: Color::Indexed(245),
        };

        let image_viewer = ImageViewerColors {
            bg: Color::Indexed(235),
            border: Color::Indexed(252),
            title_text: Color::Indexed(255),
            loading_spinner: Color::Indexed(117),
            loading_text: Color::Indexed(245),
            error_text: Color::Indexed(204),
            hint_text: Color::Indexed(245),
            footer_key: Color::Indexed(117),
            footer_text: Color::Indexed(245),
            footer_separator: Color::Indexed(245),
        };

        let file_info = FileInfoColors {
            bg: Color::Indexed(235),
            border: Color::Indexed(252),
            title: Color::Indexed(255),
            label: Color::Indexed(245),
            value: Color::Indexed(252),
            value_name: Color::Indexed(117),
            value_path: Color::Indexed(252),
            value_type: Color::Indexed(252),
            value_size: Color::Indexed(117),
            value_permission: Color::Indexed(252),
            value_owner: Color::Indexed(252),
            value_date: Color::Indexed(252),
            calculating_spinner: Color::Indexed(117),
            calculating_text: Color::Indexed(117),
            error_text: Color::Indexed(204),
            hint_text: Color::Indexed(245),
        };

        let help = HelpColors {
            bg: Color::Indexed(235),
            border: Color::Indexed(252),
            title: Color::Indexed(255),
            section_title: Color::Indexed(117),
            section_decorator: Color::Indexed(245),
            key: Color::Indexed(117),
            key_highlight: Color::Indexed(117),
            description: Color::Indexed(252),
            hint_text: Color::Indexed(245),
        };

        let advanced_search = AdvancedSearchColors {
            bg: Color::Indexed(235),
            border: Color::Indexed(252),
            title: Color::Indexed(255),
            label: Color::Indexed(252),
            input_text: Color::Indexed(252),
            input_cursor: Color::Indexed(252),
            checkbox_checked: Color::Indexed(114),
            checkbox_unchecked: Color::Indexed(245),
            button_text: Color::Indexed(245),
            button_selected_bg: Color::Indexed(117),
            button_selected_text: Color::Indexed(235),
            footer_key: Color::Indexed(117),
            footer_text: Color::Indexed(245),
        };

        Self {
            palette,
            state,
            panel,
            header,
            status_bar,
            function_bar,
            message,
            dialog,
            settings,
            editor,
            viewer,
            process_manager,
            ai_screen,
            system_info,
            search_result,
            image_viewer,
            file_info,
            help,
            advanced_search,
            chars: ThemeChars::default(),

            // 하위 호환성 필드
            bg: Color::Indexed(235),
            bg_panel: Color::Indexed(235),
            bg_selected: Color::Indexed(117),
            bg_header: Color::Indexed(236),
            bg_header_active: Color::Indexed(237),
            bg_status_bar: Color::Indexed(237),
            text: Color::Indexed(252),
            text_dim: Color::Indexed(245),
            text_bold: Color::Indexed(252),
            text_selected: Color::Indexed(235),
            text_header: Color::Indexed(250),
            text_header_active: Color::Indexed(255),
            text_directory: Color::Indexed(117),
            border: Color::Indexed(245),
            border_active: Color::Indexed(252),
            success: Color::Indexed(114),
            warning: Color::Indexed(204),
            error: Color::Indexed(204),
            info: Color::Indexed(81),
            shortcut_key: Color::Indexed(250),
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // 하위 호환성을 위한 스타일 메서드 (기존 코드에서 사용)
    // ═══════════════════════════════════════════════════════════════════════════

    pub fn normal_style(&self) -> Style {
        Style::default().fg(self.text)
    }

    pub fn dim_style(&self) -> Style {
        Style::default().fg(self.text_dim)
    }

    pub fn selected_style(&self) -> Style {
        Style::default()
            .fg(self.text_selected)
            .bg(self.bg_selected)
    }

    pub fn directory_style(&self) -> Style {
        Style::default()
            .fg(self.text_directory)
            .add_modifier(Modifier::BOLD)
    }

    pub fn header_style(&self) -> Style {
        Style::default()
            .fg(self.text_header)
            .add_modifier(Modifier::BOLD)
    }

    pub fn border_style(&self, active: bool) -> Style {
        if active {
            Style::default().fg(self.border_active)
        } else {
            Style::default().fg(self.border)
        }
    }

    pub fn warning_style(&self) -> Style {
        Style::default()
            .fg(self.warning)
            .add_modifier(Modifier::BOLD)
    }

    pub fn error_style(&self) -> Style {
        Style::default().fg(self.error)
    }

    pub fn success_style(&self) -> Style {
        Style::default().fg(self.success)
    }

    pub fn marked_style(&self) -> Style {
        Style::default().fg(self.warning)
    }

    pub fn status_bar_style(&self) -> Style {
        Style::default()
            .fg(self.text_header)
            .bg(self.bg_status_bar)
    }

    pub fn info_style(&self) -> Style {
        Style::default().fg(self.info)
    }
}
