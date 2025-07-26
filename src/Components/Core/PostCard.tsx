import test from "@/assets/Home/Banner 1.jpg";
import {IoEyeSharp} from "react-icons/io5";
import {FaCommentDots, FaHeart} from "react-icons/fa";
import {useNavigate} from "react-router-dom";

function PostCard() {
    const navigate = useNavigate();


    return (
        <div className={"relative size-fit rounded-md overflow-hidden shadow-lg"} onClick={()=>{navigate("/post")}}>
            <img src={test} alt={"Video"} className={"object-cover w-full h-full"} />
            <div className="absolute bottom-0 w-full">
                <div className="bg-gradient-to-t from-black/70 to-transparent px-5 pt-8 pb-2.5 text-white text-sm">
                    <div className="flex flex-row justify-between gap-2">
                        <div className="flex flex-row gap-1 items-center">
                            <IoEyeSharp className={"size-4 lg:size-5"} />
                            <p>1,000,000</p>
                        </div>

                        <div className="flex flex-row gap-1 items-center">
                            <FaCommentDots className={"size-4 lg:size-5"} />
                            <p>1,000,000</p>
                        </div>


                        <div className="flex flex-row gap-1 items-center">
                            <FaHeart className={"size-4 lg:size-5"} />
                            <p>1,231,328</p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
}

export default PostCard;