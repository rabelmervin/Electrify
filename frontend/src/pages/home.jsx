import React from 'react';
import Navbar from '../components/Navbar';
import Herosection from '../components/Herosection';
import Trusted from '../components/Trusted';         
import ValueProps from '../components/Value'; 
import "../styles/Home.css"
const Home = () => {
    return (
        <div className="home-container">
            <Navbar />
                <main>
                    <Herosection />
                    <Trusted />
                    <ValueProps />
                </main>
        </div>
    )
}

export default Home;