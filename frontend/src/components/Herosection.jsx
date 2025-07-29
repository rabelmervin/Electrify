import react from "react";
import { Swiper, SwiperSlide } from "swiper/react";
import { Autoplay, Pagination} from "swiper/modeules";
import "swiper/css"
import "swiper/css/pagination"

const Herosection = () => {
    const slides = [

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
                  </Swiper>
    </div>
    )

}