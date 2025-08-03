import { Link } from 'react-router-dom';
import React, { useState } from 'react'; 
import DropdownMenu from "./Dropdownbox"
import "../styles/Navbar.css"
const Navbar = () => {
    const [isDropdownOpen, setIsDropdownOpen] = useState(false);
    return (
        <nav className= "navbar">
            <div className="logo"></div>
            <ul className= "nav-links">
                <li><Link to= "/">ElectrifAI</Link></li>
                <li><Link to= "/">Products</Link></li>
                <li onClick={() => setIsDropdownOpen(!isDropdownOpen)} style={{ cursor: 'pointer'}}><Link to= "/">Electrimy</Link></li>
                <li><Link to= "/">About us</Link></li>
                
                <div className='navbar-right'>
                <li><Link to= "/login">Log in</Link></li> 
                <button><Link to= "/register">Create Account</Link></button>
                </div>                
            </ul>
            {isDropdownOpen && <DropdownMenu />}
        </nav>
    );
};

export default Navbar;