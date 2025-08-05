import React from "react";
import "../styles/Trusted.css"

const Trusted = () => {
    return (
        <section className ="trusted-section">
            <h2 className = "trusted-text"> Trusted by top companies around the globe </h2>
            <div className="trusted-images">   
               <img src="https://upload.wikimedia.org/wikipedia/commons/2/2f/Google_2015_logo.svg" alt="Google Logo" className="h-10" />
               <img src="https://upload.wikimedia.org/wikipedia/commons/2/2f/Google_2015_logo.svg" alt="Google Logo" className="h-10" />
               <img src="https://upload.wikimedia.org/wikipedia/commons/2/2f/Google_2015_logo.svg" alt="Google Logo" className="h-10" />
               <img src="https://upload.wikimedia.org/wikipedia/commons/2/2f/Google_2015_logo.svg" alt="Google Logo" className="h-10" />
               </div>                               
        </section>
    );
};

export default Trusted;