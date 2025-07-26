import test from "@/assets/Videos/The Fantastic Four First Steps Official Teaser Only in Theaters July 25 - Marvel Entertainment (1080p, h264).mp4";
import { motion } from "framer-motion";
import testP from "@/assets/Profile/VanessaKirby.jpg"
import {useRef, useState} from "react";
import {FaCommentDots, FaHeart, FaPause, FaPlay, FaVolumeMute, FaVolumeUp} from "react-icons/fa";
import ViewComment from "@/Components/Core/ViewComment.tsx";

function ForYourPage() {
    const videos = useRef<HTMLVideoElement>(null);
    const [isPlaying, setIsPlaying] = useState(true);
    const [isMuted, setIsMuted] = useState(false);
    const [progress, setProgress] = useState(0);
    const [isHovered, setIsHovered] = useState(false);
    const [viewComment, setViewComment] = useState(false)


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

    function toggleComment() {
        setViewComment(!viewComment);
    }

    return (
        <div className={"flex flex-row w-full"}>
            <div className="relative w-full h-full"
                 onMouseEnter={() => setIsHovered(true)}
                 onMouseLeave={() => setIsHovered(false)}>
                <video
                    ref={videos}
                    src={test}
                    autoPlay
                    loop
                    muted={isMuted}
                    className="w-full h-screen object-cover"
                    onClick={togglePlay}

                    onTimeUpdate={handleTimeUpdate}
                />

                {isHovered && (
                    <div
                        className="absolute bg-black/60 inset-0 flex justify-center items-center"
                        onClick={togglePlay}
                    >
                        <div className="p-4 rounded-full cursor-pointer hover:scale-110 transition">
                            {isPlaying ? (
                                <FaPlay className="text-white text-3xl" />
                            ) : (
                                <FaPause className="text-white text-3xl" />
                                )}

                        </div>
                    </div>
                )}

                {(!isPlaying || isHovered) &&(
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

                <div className={"absolute rounded-tl-xl right-0 bottom-1 p-2 pb-10 text-white bg-gradient-to-tl from-black/50 to-transparent flex flex-col gap-7 justify-center"}>
                    <div>
                        <img src={testP} alt="Profile" className={"object-cover rounded-full size-16 lg:size-24 border-primary border-2"}/>
                    </div>
                    <div className="flex flex-col gap-1 items-center">
                        <FaHeart className={"size-4 lg:size-5"} />
                        <p>1,231,328</p>
                    </div>
                    <div className="flex flex-col gap-1 items-center">
                        <FaCommentDots className={"cursor-pointer size-4 lg:size-5"} onClick={toggleComment}/>
                        <p>1,000,000</p>
                    </div>

                </div>
            </div>
            {viewComment && (
                <ViewComment closeComment={toggleComment}/>
            )}

        </div>
    );
}

export default ForYourPage;
