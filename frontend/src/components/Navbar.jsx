import { Link } from 'react-router-dom';
import React, { useState } from 'react'; 
import DropdownMenu from "./Dropdownbox"

const Navbar = () => {
    const [isDropdownOpen, setIsDropdownOpen] = useState(false);
    return (
        <nav className= "navbar">
            <ul className= "nav-links">
                <li><Link to= "/">ElectrifAI</Link></li>
                <li><Link to= "/">Products</Link></li>
                <li onClick={() => setIsDropdownOpen(!isDropdownOpen)} style={{ cursor: 'pointer'}}><Link to= "/">Electrimy</Link></li>
                <li><Link to= "/">About us</Link></li>
                <li><Link to= "/login">Log in</Link></li> 
                <li><Link to= "/register">Create Account</Link></li>                 
            </ul>
            {isDropdownOpen && <DropdownMenu />}
        </nav>
    );
};

export default Navbar;