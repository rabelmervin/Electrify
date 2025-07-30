import React from 'react';
import Navbar from '../components/Navbar';
import Herosection from '../components/Herosection';
const Home = () => {
    return (
        <div>
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