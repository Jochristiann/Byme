import {RxCross2} from "react-icons/rx";
import CommentMessage from "@/Components/Supports/CommentMessage.tsx";
import {Input} from "@/Components/Interactive/Input.tsx";

function ViewComment({closeComment}:{closeComment:()=>void}) {

    return (
        <div className={"h-full w-full bg-white shadow-lg"}>
            <div className={"flex flex-col gap-5 px-5 lg:p-5 w-full"}>
                <div className={"flex flex-col gap-5 sticky"}>
                    <div className={"flex flex-row justify-between items-center border-b-2 border-primary"}>
                        <h4 className={"font-bold"}>Comments</h4>
                        <RxCross2 className={"cursor-pointer"} size={16} onClick={closeComment}/>
                    </div>
                    <Input placeholder={"Add your comment..."}/>
                </div>
                <div className={"flex flex-col gap-2 h-full"}>
                    <div className={"flex flex-col gap-2 w-full px-5"}>
                        {[...Array(5).keys()].map((_) => (
                            <CommentMessage/>
                        ))}
                    </div>
                </div>
            </div>
        </div>
    );
}

export default ViewComment;