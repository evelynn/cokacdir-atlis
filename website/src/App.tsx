import { Routes, Route } from 'react-router-dom'
import LandingPage from './pages/LandingPage'
import TutorialPage from './components/tutorial/TutorialPage'
import EC2Page from './components/ec2/EC2Page'

function App() {
  return (
    <Routes>
      <Route path="/" element={<LandingPage />} />
      <Route path="/tutorial" element={<TutorialPage />} />
      <Route path="/ec2" element={<EC2Page />} />
    </Routes>
  )
}

export default App
