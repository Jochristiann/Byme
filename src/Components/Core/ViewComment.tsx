import {RxCross2} from "react-icons/rx";
import CommentMessage from "@/Components/Supports/CommentMessage.tsx";
import {Input} from "@/Components/Interactive/Input.tsx";

function ViewComment({closeComment}:{closeComment:()=>void}) {

    return (
        <div className={"h-screen w-1/2 bg-white overflow-hidden shadow-lg"}>
            <div className={"flex flex-col gap-5 p-5 w-full h-screen overflow-y-auto"}>
                <div className={"flex flex-row justify-between items-center border-b-2 border-primary"}>
                    <h4 className={"font-bold"}>Comments</h4>
                    <RxCross2 size={16} onClick={closeComment}/>
                </div>
                <div className={"flex flex-col gap-2"}>
                    <Input placeholder={"Add your comment..."}/>
                    <div className={"flex flex-col gap-2 w-full"}>
                        {[...Array(20).keys()].map((_) => (
                            <CommentMessage/>
                        ))}
                    </div>
                </div>
            </div>
        </div>
    );
}

export default ViewComment;