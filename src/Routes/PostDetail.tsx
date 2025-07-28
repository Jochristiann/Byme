import PostContent from "@/Components/Core/Post/PostContent.tsx";
import ViewComment from "@/Components/Core/Post/ViewComment.tsx";
import {useState} from "react";
import {TbArrowBackUp} from "react-icons/tb";
import {useNavigate} from "react-router-dom";

function PostDetail() {

    const navigate = useNavigate();
    const [viewComments, setViewComments] = useState(true)

    function handleViewComments() {
        setViewComments(!viewComments)
    }

    function backTrack(){
        navigate(-1)
    }

    return (
        <div className={"w-full h-full"}>
            <div className={"flex flex-col gap-2"}>
                <div className={"relative "}>
                    <TbArrowBackUp onClick={backTrack} size={40} className={"absolute cursor-pointer top-2 left-1 text-white"} />
                    <PostContent/>
                </div>
                {viewComments && (
                    <ViewComment closeComment={handleViewComments} />
                )}
            </div>
        </div>
    );
}

export default PostDetail;