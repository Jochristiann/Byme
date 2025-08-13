import {MdError} from "react-icons/md";
import {Button} from "@/Components/Interactive/Button.tsx";
import {FaCircleCheck} from "react-icons/fa6";

function NotificationPopup({type = 1, message, func}: {type?:number,message: string, func:() => void}) {

    return (
        <div className={"fixed inset-0 z-50 bg-black/50 flex justify-center items-center"}>
            <div className={"bg-white rounded-xl flex flex-col gap-5 justify-center items-center p-10"}>
                <div className={`flex flex-col gap-2 justify-center items-center ${type == 1 ? 'text-red-500' : 'text-green-500'}`}>
                    {type == 1 ? (
                        <>
                            <MdError size={64} color={"red"}/>
                            <h3>Error</h3>
                        </>
                    ):(
                        <>
                            <FaCircleCheck size={64} color={"green"}/>
                            <h3>Success</h3>
                        </>
                    )}
                </div>
                <p>{message}</p>
                <Button onClick={func}>Close</Button>
            </div>
        </div>
    );
}

export default NotificationPopup;