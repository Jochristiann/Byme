import test from "@/assets/Videos/The Fantastic Four First Steps Official Teaser Only in Theaters July 25 - Marvel Entertainment (1080p, h264).mp4";
import {AnimatePresence, motion} from "framer-motion";
import testP from "@/assets/Profile/VanessaKirby.jpg"
import {useEffect, useRef, useState} from "react";
import {
    FaArrowLeft, FaArrowRight,
    FaCheck,
    FaCommentDots,
    FaHeart,
    FaPause,
    FaPlay,
    FaPlus,
    FaVolumeMute,
    FaVolumeUp
} from "react-icons/fa";
import ViewComment from "@/Components/Core/Post/ViewComment.tsx";
import {useNavigate} from "react-router-dom";
import {formatLikeNumber} from "@/FrontUtils/Library.ts";

function ForYourPage() {
    const navigate = useNavigate();
    const videos = useRef<HTMLVideoElement>(null);
    const [isPlaying, setIsPlaying] = useState(true);
    const [isMuted, setIsMuted] = useState(false);
    const [progress, setProgress] = useState(0);
    const [isHovered, setIsHovered] = useState(false);
    const [viewComment, setViewComment] = useState(false);
    const [isLoved, setIsLoved] = useState(false)
    const [isFollowed, setIsFollowed] = useState(false);

    useEffect(() => {
        // const checkVideos = () => {
        //     const video = videos.current;
        //     if (video) {
        //        setIsPlaying(video.paused);
        //     }
        // }
        //
        // checkVideos();

    },[])

    function toggleLoved() {
        setIsLoved(!isLoved)
    }
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

    function toggleFollow() {
        setIsFollowed(!isFollowed);
    }

    return (
        <div className={"flex flex-row w-full"}>
            <div className="relative w-full h-full">
                <div className="relative w-full h-full"
                     onMouseEnter={() => setIsHovered(true)}
                     onMouseLeave={() => setIsHovered(false)}
                >
                    <video
                        ref={videos}
                        src={test}
                        autoPlay
                        loop
                        muted={isMuted}
                        className="w-full h-screen bg-black object-contain lg:object-cover"
                        onClick={togglePlay}
                        onTimeUpdate={handleTimeUpdate}

                    />

                    {isHovered && (
                        <div
                            className="absolute bg-black/60 inset-0 "
                            onClick={togglePlay}
                        >
                            <motion.div
                                initial={{ opacity: 0, scale: 0.8 }}
                                animate={{ opacity: 1, scale: 1 }}
                                exit={{ opacity: 0, scale: 0.8 }}
                                transition={{ duration: 0.2 }}
                                className={"w-full h-full flex flex-row justify-evenly items-center"}
                            >
                                <FaArrowLeft className="text-white text-2xl"/>
                                <div className="p-4 rounded-full cursor-pointer hover:scale-110 transition">
                                    {isPlaying ? (
                                        <FaPlay className="text-white text-3xl" />
                                    ) : (
                                        <FaPause className="text-white text-3xl" />
                                    )}
                                </div>

                                <FaArrowRight className="text-white text-2xl"/>
                            </motion.div>
                        </div>
                    )}

                    {(!isPlaying || isHovered) &&(
                        <motion.div
                            initial={{ opacity: 0, scale: 0.8 }}
                            animate={{ opacity: 1, scale: 1 }}
                            exit={{ opacity: 0, scale: 0.8 }}
                            transition={{ duration: 0.5 }}
                            onMouseEnter={() => setIsHovered(true)}
                            className={`absolute w-full bottom-5 flex items-center gap-4 px-4 py-2 text-white z-10` }>
                            <button onClick={toggleMute}>
                                {isMuted ? <FaVolumeMute size={24}/> : <FaVolumeUp size={24} />}
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
                <div className={"absolute rounded-tl-4xl right-0 bottom-1 p-5 pb-10 text-white bg-gradient-to-tl from-black/50 to-transparent flex flex-col gap-7 justify-center"}>
                    <div className={"relative w-full flex items-center justify-center"}>
                        <img src={testP} alt="Profile" className={"object-cover rounded-full size-16 lg:size-24 border-primary border-2"} onClick={()=>{navigate("/profile")}}/>
                        <div className={"absolute -bottom-2 bg-white p-1 rounded-full size-4 lg:size-6"} onClick={toggleFollow}>
                            <AnimatePresence mode="wait">
                                {isFollowed ? (
                                    <motion.div
                                        key="check"
                                        initial={{ opacity: 0, scale: 0.8 }}
                                        animate={{ opacity: 1, scale: 1 }}
                                        exit={{ opacity: 0, scale: 0.8 }}
                                        transition={{ duration: 0.2 }}
                                    >
                                        <FaCheck className="text-green-700"/>
                                    </motion.div>
                                ) : (
                                    <motion.div
                                        key="plus"
                                        initial={{ opacity: 0, scale: 0.8 }}
                                        animate={{ opacity: 1, scale: 1 }}
                                        exit={{ opacity: 0, scale: 0.8 }}
                                        transition={{ duration: 0.2 }}
                                    >
                                        <FaPlus className="text-black" />
                                    </motion.div>
                                )}
                            </AnimatePresence>
                        </div>
                    </div>
                    <div className="flex flex-col gap-1 items-center z-50">
                        <FaHeart
                            className={`cursor-pointer size-5 lg:size-7 ${isLoved ? 'text-red-500' : 'text-white'}`}
                            onClick={toggleLoved}
                        />
                        <p className={"text-sm lg:text-md"}>{formatLikeNumber(1239213)}</p>
                    </div>
                    <div className="flex flex-col gap-1 items-center">
                        <FaCommentDots className={"cursor-pointer size-5 lg:size-7"} onClick={toggleComment}/>
                        <p className={"text-sm lg:text-md"}>{formatLikeNumber(911721)}</p>
                    </div>
                </div>
            </div>

            {viewComment && (
                <div className={"w-1/3"}>
                    <ViewComment closeComment={toggleComment}/>
                </div>
            )}

        </div>
    );
}

export default ForYourPage;
