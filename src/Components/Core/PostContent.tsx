import video from "@/assets/Videos/The Fantastic Four First Steps Official Teaser Only in Theaters July 25 - Marvel Entertainment (1080p, h264).mp4";
import {IoEyeSharp} from "react-icons/io5";
import {FaHeart} from "react-icons/fa";
import {CgProfile} from "react-icons/cg";
import {Button} from "@/Components/Interactive/Button.tsx";
import {useRef, useState} from "react";

function PostContent() {
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
        <div className={"h-full w-full overflow-hidden"}>
            <div className={"flex flex-col gap-5"}>
                <div className={"flex justify-center relative lg:static overflow-y-scroll"}>
                    <video src={video} autoPlay={true} loop className={"object-cover mb-10 lg:mb-0"} controls={true}/>
                    <div className="lg:hidden absolute bottom-0 w-full">
                        <div className="px-5 pt-8 pb-2.5 text-sm">
                            <div className="flex flex-row justify-between gap-2">
                                <div className="flex flex-row gap-1 items-center">
                                    <IoEyeSharp size={14} />
                                    <p>1,000,000</p>
                                </div>
                                <div className="flex flex-row gap-1 items-center">
                                    <FaHeart size={10} />
                                    <p>1,231,328</p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
                <div className={"flex flex-col gap-5 p-5"}>
                    <div className={"flex flex-row justify-between items-center"}>
                        <div className={"flex flex-row gap-2 items-center"}>
                            <CgProfile size={48}/>
                            <div className={"flex flex-col gap-0.5"}>
                                <h5>Marvel Entertainment</h5>
                                <p className={"text-sm"}>7,981,991 followers</p>
                            </div>
                        </div>
                        <Button>Follow</Button>
                    </div>
                    <div>
                        <p>
                            The Fantastic Four will arrive in the cinemas soon!!!!
                        </p>
                    </div>
                </div>
            </div>
        </div>
    );
}

export default PostContent;