import {CgProfile} from "react-icons/cg";
import {FaHeart} from "react-icons/fa";
import {useState} from "react";

function CommentMessage() {

    const [isLoved, setIsLoved] = useState(true)

    function toggleLoved() {
        setIsLoved(!isLoved)
    }

    return (
        <div className={"w-full"}>
            <div className={"flex flex-row justify-between"}>
                <div className={"w-full flex flex-col gap-2"}>
                    <div className={"flex flex-row gap-1 items-center"}>
                        <CgProfile size={18}/>
                        <h5>gmaerkevin</h5>
                    </div>

                    <div className={"w-full flex flex-row gap-1 items-center justify-between pl-5"}>
                        <p className={"w-11/12"}>EXCITED!</p>
                        <FaHeart size={14} className={isLoved ? "text-red-500" : "text-gray-500"} onClick={toggleLoved}/>
                    </div>
                </div>
            </div>
        </div>
    );
}

export default CommentMessage;