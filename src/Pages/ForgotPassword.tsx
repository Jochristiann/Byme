import {Button} from "@/Components/Interactive/Button.tsx";
import {Input} from "@/Components/Interactive/Input.tsx";
import {IoMail} from "react-icons/io5";
import {useState} from "react";
import NotificationPopup from "@/Components/Interactive/NotificationPopup.tsx";
import {forgetPasswordHandler} from "@/FrontUtils/AuthenticationHandler.ts";
import {useNavigate} from "react-router-dom";
import {TbArrowBackUp} from "react-icons/tb";

function ForgotPassword() {
    const [email, setEmail] = useState("")
    const navigate = useNavigate()
    const [notifMessage, setNotifMessage] = useState("")
    const [isNotif, setIsNotif] = useState<boolean>(false)
    const [type, setType] = useState(1)
    function popupToggle(){
        setIsNotif(!isNotif)
        if(type == 2) {
            navigate("/auth/login")
        }
    }

    async function sendEmail(){
        if(!email){
            setNotifMessage("Please fill out all fields")
            setIsNotif(true)
            return
        }

        try{
            let response = await forgetPasswordHandler(email)
            setNotifMessage(response.data)
            setType(2)
            setIsNotif(true)
        }catch(err:any){
            const temp = "Something went wrong. Please try again later."
            const backendMessage = err.data || temp
            setNotifMessage(backendMessage)
            setType(1)
            setIsNotif(true)
        }
    }

    function backTrack(){
        navigate(-1)
    }

    return (
        <div className={"w-screen h-screen bg-primary"}>
            <div className={"h-full flex justify-center items-center"}>
                <div className={"relative bg-white rounded-4xl flex flex-col gap-10 shadow-md items-center justify-center p-20 border-1 border-gray-300"}>
                    <TbArrowBackUp onClick={backTrack} size={40} className={"absolute cursor-pointer top-5 left-5"} />
                    <h3 className={"text-center text-2xl font-bold"}>Forget Password</h3>
                    <div className={"flex flex-col gap-1 justify-center items-center"}>
                        <div className={'relative flex items-center'}>
                            <IoMail className={"absolute left-4"}/>
                            <Input
                                serial={"auth"}
                                onChange={(e) => setEmail(e.target.value)}
                                value={email}
                                name="email"
                                type="email"
                                placeholder="Email"
                                required />
                        </div>
                    </div>
                    <Button onClick={sendEmail}>Send to Email</Button>
                </div>
            </div>

            {isNotif && (
                <NotificationPopup type={type} message={notifMessage} func={popupToggle}/>
            )}
        </div>
    );
}

export default ForgotPassword;