import React from "react";
import { Swiper, SwiperSlide } from "swiper/react";
import { Autoplay, Pagination } from "swiper/modules";

import "swiper/css";
import "swiper/css/pagination";

const Herosection = () => {
    const slides = [
                "Welcome to Electrify!",
        "Empower your business with our solutions.",
        "Join us and innovate today."

    ]
    return (
        <div className ="hero-section">
            <div className="overlay" />
            <Swiper
                modules = {[Autoplay, Pagination]}
                autoplay = {{ delay: 3000, disbaleOnInteraction:false}}
                pagination= {{ clickable: true}}
                loop= {true}
                className="hero-swiper"
        >
            {slides.map((text, index) => (
                <SwiperSlide key={index}>
                  <div className="hero-content">
                    <h1>{text}</h1>
                  </div>
                  </SwiperSlide>
            ))}
                  </Swiper>
    </div>
    )

}

export default Herosection;