import test from "@/assets/Videos/The Fantastic Four First Steps Official Teaser Only in Theaters July 25 - Marvel Entertainment (1080p, h264).mp4";
import { motion } from "framer-motion";
import {useRef, useState} from "react";
import { FaPlay, FaVolumeMute, FaVolumeUp} from "react-icons/fa";

function ForYourPage() {
    const videos = useRef<HTMLVideoElement>(null);
    const [isPlaying, setIsPlaying] = useState(false);
    const [isMuted, setIsMuted] = useState(false);
    const [progress, setProgress] = useState(0);
    const [isHovered, setIsHovered] = useState(false);


    const togglePlay = () => {
        const video = videos.current;
        if (video) {
            if (video.paused) {
                video.play();
                setIsPlaying(true);
            } else {
                video.pause();
                setIsPlaying(false);
            }
        }
    };

    const toggleMute = () => {
        const video = videos.current;
        if (!video) return;
        video.muted = !video.muted;
        setIsMuted(video.muted);
    };

    const handleTimeUpdate = () => {
        const video = videos.current;
        if (!video) return;
        const percent = (video.currentTime / video.duration) * 100;
        setProgress(percent || 0);
    };

    const handleSeek = (e: React.MouseEvent<HTMLDivElement, MouseEvent>) => {
        const video = videos.current;
        if (!video) return;

        const rect = (e.target as HTMLDivElement).getBoundingClientRect();
        const clickX = e.clientX - rect.left;
        const width = rect.width;
        video.currentTime = (clickX / width) * video.duration;
    };

    return (
        <div className={"flex flex-col gap-5 w-full"}>
            <div className="relative w-full">
                <video
                    ref={videos}
                    src={test}
                    autoPlay
                    loop
                    muted={isMuted}
                    className="w-full h-screen object-cover"
                    onClick={togglePlay}
                    onMouseEnter={() => setIsHovered(true)}
                    onMouseLeave={() => setIsHovered(false)}
                    onTimeUpdate={handleTimeUpdate}
                />

                {!isPlaying && (
                    <div
                        className="absolute bg-black/60 inset-0 flex justify-center items-center"
                        onClick={togglePlay}
                    >
                        <div className="p-4 rounded-full cursor-pointer hover:scale-110 transition">
                            <FaPlay className="text-white text-3xl" />
                        </div>
                    </div>
                )}

                {(!isPlaying ||isHovered) &&(
                    <motion.div
                        initial={{ opacity: 0, y: -1, scale: 0.95 }}
                        animate={{ opacity: 1, y: 0, scale: 1 }}
                        exit={{ opacity: 0, y: -1, scale: 0.95 }}
                        transition={{ duration: 0.3, ease: "easeInOut" }}
                        onMouseEnter={() => setIsHovered(true)}
                        className={`absolute w-full bottom-1 flex items-center gap-4 px-4 py-2 text-white z-10` }>
                        <button onClick={toggleMute}>
                            {isMuted ? <FaVolumeMute size={24} /> : <FaVolumeUp size={24} />}
                        </button>
                    </motion.div>
                )}

                <div className="absolute bottom-0 left-0 w-full">
                    <div className="h-1 bg-white/30 cursor-pointer" onClick={handleSeek}>
                        <div
                            className="h-1 bg-white transition-all duration-200"
                            style={{ width: `${progress}%` }}
                        />
                    </div>
                </div>
            </div>
        </div>
    );
}

export default ForYourPage;
