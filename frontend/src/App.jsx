import react from "react"
import { BrowserRouter, Routes, Route, Navigation } from "react-router-dom"
import Login from "./pages/login"
import Register from "./pages/register"
import Home from "./pages/home"


function Logout(){
  localStorage.clear()
    return <Navigate to= "/login" />
}
function RegisterandLogout(){
  localStorage.clear()
  return <Register/>
}

function App() {

  return (
    <>

</>
  )
}

export default App
