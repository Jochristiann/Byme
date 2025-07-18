import PostContent from "@/Components/Core/PostContent.tsx";
import ViewComment from "@/Components/Core/ViewComment.tsx";
import {useState} from "react";

function PostDetail() {

    const [viewComments, setViewComments] = useState(true)

    function handleViewComments() {
        setViewComments(!viewComments)
    }

    return (
        <div className={"w-full"}>
            <div className={"flex flex-row"}>
                <PostContent/>
                {viewComments && (
                    <ViewComment closeComment={handleViewComments} />
                )}
            </div>
        </div>
    );
}

export default PostDetail;