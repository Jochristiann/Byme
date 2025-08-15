import {useState} from "react";
import {motion, AnimatePresence} from "framer-motion";
import {FaCheck, FaCommentDots,
    FaHeart, FaPlus
} from "react-icons/fa";
import {useNavigate} from "react-router-dom";
import ViewComment from "@/Components/Core/Post/ViewComment.tsx";
import {formatLikeNumber} from "@/FrontUtils/Library.ts";
import VideoDisplay from "@/Components/Core/Video/VideoDisplay.tsx";

interface VideoCardProps {
    user:User|null,
    src: string;
    profileImg: string;
    isActive: boolean;
}

const FYPCard = ({ user, src, profileImg, isActive }: VideoCardProps) => {
    const navigate = useNavigate();

    const [viewComment, setViewComment] = useState(false);
    const [isLoved, setIsLoved] = useState(false)
    const [isFollowed, setIsFollowed] = useState(false);

    const toggleLoved = () => setIsLoved(!isLoved);
    const toggleFollow = () => setIsFollowed(!isFollowed);
    const toggleComment = () => setViewComment(!viewComment);


    return (
        <div className={"relative w-full h-screen"}>
            <VideoDisplay src={src} isActive={isActive} />

            <div className="absolute rounded-tl-4xl right-0 bottom-1 p-5 pb-10 text-white bg-gradient-to-tl from-black/50 to-transparent flex flex-col gap-7 justify-center">
                {user && (
                    <div className="relative w-full flex items-center justify-center">
                        <img src={profileImg} alt="Profile"
                             className="object-cover rounded-full size-16 lg:size-24 border-primary border-2"
                             onClick={() => navigate("/profile/" + user.username)}/>
                        <div className="absolute -bottom-2 bg-white p-1 rounded-full size-4 lg:size-6"
                             onClick={toggleFollow}>
                            <AnimatePresence mode="wait">
                                {isFollowed ? (
                                    <motion.div key="check" initial={{opacity: 0, scale: 0.8}}
                                                animate={{opacity: 1, scale: 1}} exit={{opacity: 0, scale: 0.8}}
                                                transition={{duration: 0.2}}>
                                        <FaCheck className="text-green-700"/>
                                    </motion.div>
                                ) : (
                                    <motion.div key="plus" initial={{opacity: 0, scale: 0.8}}
                                                animate={{opacity: 1, scale: 1}} exit={{opacity: 0, scale: 0.8}}
                                                transition={{duration: 0.2}}>
                                        <FaPlus className="text-black"/>
                                    </motion.div>
                                )}
                            </AnimatePresence>
                        </div>
                    </div>
                )}


                <div className="flex flex-col gap-1 items-center z-50">
                    <FaHeart className={`cursor-pointer size-5 lg:size-7 ${isLoved ? 'text-red-500' : 'text-white'}`}
                             onClick={toggleLoved}/>
                    <p className="text-sm lg:text-md">{formatLikeNumber(1239213)}</p>
                </div>
                <div className="flex flex-col gap-1 items-center">
                    <FaCommentDots className="cursor-pointer size-5 lg:size-7" onClick={toggleComment}/>
                    <p className="text-sm lg:text-md">{formatLikeNumber(911721)}</p>
                </div>
            </div>

            {viewComment && (
                <div className="absolute inset-0 flex flex-col lg:flex-row">
                    <div className="lg:w-2/3 w-full h-1/2 lg:h-full pointer-events-none"/>
                    <div className="lg:w-1/3 w-full h-1/2 lg:h-full overflow-y-auto z-50">
                        <ViewComment closeComment={toggleComment}/>
                    </div>
                </div>
            )}
        </div>
    );
};

export default FYPCard;
