import {useEffect, useRef, useState} from 'react';
import {AnimatePresence, motion} from "framer-motion";
import {FaArrowLeft, FaArrowRight, FaPause, FaPlay, FaVolumeMute, FaVolumeUp} from "react-icons/fa";

interface VideoCardProps {
    src: string;
    isActive: boolean;
}

function VideoDisplay({src, isActive}:VideoCardProps) {

    const [isPlaying, setIsPlaying] = useState(true);
    const [isMuted, setIsMuted] = useState(false);
    const [progress, setProgress] = useState(0);
    const [isHovered, setIsHovered] = useState(false);
    const videos = useRef<HTMLVideoElement>(null);
    const inactivityTimer = useRef<NodeJS.Timeout | null>(null);

    useEffect(() => {
        const video = videos.current;
        if (!video) return;

        if (isActive) {
            video.currentTime = 0;
            video.play().catch(() => {});
            setIsPlaying(true);
        } else {
            video.pause();
            video.currentTime = 0;
            setIsPlaying(false);
        }
    }, [isActive]);

    const videoAreaRef = useRef<HTMLDivElement>(null);
    useEffect(() => {
        const area = videoAreaRef.current;
        if (!area) return;

        area.addEventListener("mousemove", handleMouseMove);
        return () => {
            area.removeEventListener("mousemove", handleMouseMove);
            if (inactivityTimer.current) clearTimeout(inactivityTimer.current);
        };
    }, []);


    const resetInactivityTimer = () => {
        setIsHovered(true);
        if (inactivityTimer.current) clearTimeout(inactivityTimer.current);

        inactivityTimer.current = setTimeout(() => {
            setIsHovered(false);
        }, 2000);
    };

    const handleMouseMove = () => {
        resetInactivityTimer();
    };

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
        <div
            ref={videoAreaRef}
            className="relative w-full h-full"
            onMouseEnter={() => setIsHovered(true)}
            onMouseLeave={() => setIsHovered(false)}
        >
            <video
                ref={videos}
                src={src}
                autoPlay
                loop
                muted={isMuted}
                className="w-full h-screen bg-black object-contain lg:object-cover"
                onClick={togglePlay}
                onTimeUpdate={handleTimeUpdate}
            />

            <AnimatePresence>
                <motion.div
                    className="absolute inset-0 bg-black/60"
                    initial={{ opacity: 0 }}
                    animate={{ opacity: isHovered ? 1 : 0 }}
                    transition={{ duration: 0.4, ease: "easeInOut" }}
                    onClick={togglePlay}
                />

                {isHovered && (
                    <motion.div
                        key="overlay-icons"
                        initial={{ opacity: 0, scale: 0.8 }}
                        animate={{ opacity: 1, scale: 1 }}
                        exit={{ opacity: 0, scale: 0.8 }}
                        transition={{ duration: 0.2 }}
                        className="absolute inset-0 flex flex-row justify-evenly items-center pointer-events-none"
                    >
                        <FaArrowLeft className="text-white text-2xl pointer-events-auto" />
                        <div className="p-4 rounded-full cursor-pointer hover:scale-110 transition pointer-events-auto">
                            {isPlaying ? (
                                <FaPause className="text-white text-3xl" onClick={togglePlay}/>
                            ) : (
                                <FaPlay className="text-white text-3xl" onClick={togglePlay}/>
                            )}
                        </div>
                        <FaArrowRight className="text-white text-2xl pointer-events-auto"/>
                    </motion.div>
                )}
            </AnimatePresence>

            <AnimatePresence>
                {(!isPlaying || isHovered) && (
                    <motion.div
                        key="controls"
                        initial={{ opacity: 0, y: 20 }}
                        animate={{ opacity: 1, y: 0 }}
                        exit={{ opacity: 0, y: 20 }}
                        transition={{ duration: 0.3 }}
                        onMouseEnter={() => setIsHovered(true)}
                        className="absolute w-full bottom-5 flex items-center gap-4 px-4 py-2 text-white z-10"
                    >
                        <button onClick={toggleMute}>
                            {isMuted ? <FaVolumeMute size={24}/> : <FaVolumeUp size={24} />}
                        </button>
                    </motion.div>
                )}
            </AnimatePresence>

            <div className="absolute bottom-0 left-0 w-full">
                <div className="h-1 bg-white/30 cursor-pointer" onClick={handleSeek}>
                    <div className="h-1 bg-white transition-all duration-200" style={{ width: `${progress}%` }} />
                </div>
            </div>
        </div>
    );
}

export default VideoDisplay;