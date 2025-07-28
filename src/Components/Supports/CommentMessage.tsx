import {CgProfile} from "react-icons/cg";
import {FaHeart} from "react-icons/fa";
import {useState} from "react";
import {formatLikeNumber} from "@/FrontUtils/Library.ts";

function CommentMessage() {

    const [isLoved, setIsLoved] = useState(false)

    function toggleLoved() {
        setIsLoved(!isLoved)
    }

    return (
        <div className={"w-full"}>
            <div className={"flex flex-row justify-between"}>
                <div className={"w-full flex flex-col gap-1"}>
                    <div className={"flex flex-row gap-1 items-center"}>
                        <CgProfile size={24}/>
                        <h5>Jose Christian</h5>
                    </div>
                    <div className={"w-full flex flex-row gap-1 items-start justify-between pl-8"}>
                        <p className={"w-11/12"}>mami kirbyyy!</p>
                        <div className={"flex flex-col gap-1 items-center justify-between"}>
                            <FaHeart size={20} className={isLoved ? "text-red-500" : "text-gray-500"} onClick={toggleLoved}/>
                            <p className={"text-xs text-gray-400"}>{formatLikeNumber(9121300)}</p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
}

export default CommentMessage;