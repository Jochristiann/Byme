import {Button} from "@/Components/Interactive/Button.tsx";
import {IoMail} from "react-icons/io5";
import {Input} from "@/Components/Interactive/Input.tsx";
import {useState} from "react";
import {changePasswordHandler} from "@/FrontUtils/AuthenticationHandler.ts";
import NotificationPopup from "@/Components/Interactive/NotificationPopup.tsx";


function ChangePassword() {
    const [password, setPassword] = useState("")
    const [confirmPassword, setConfirmPassword] = useState("")

    const [notifMessage, setNotifMessage] = useState("")
    const [isNotif, setIsNotif] = useState<boolean>(false)
    const [type, setType] = useState(1)
    function popupToggle(){
        setIsNotif(!isNotif)
    }

    async function changePassword() {
        if(!password || !confirmPassword){
            setNotifMessage("Please fill out all fields")
            setIsNotif(true)
            return
        }

        if (password !== confirmPassword){}

        try{
            let response = await changePasswordHandler(password)
            console.log(response)
            setType(2)
        }catch(err:any){
            console.log(err)
            const temp = "Something went wrong. Please try again later."
            const backendMessage = err.data?.message || temp
            setNotifMessage(backendMessage)
            setIsNotif(true)
        }
    }

    return (
        <div className={"w-screen h-screen bg-primary"}>
            <div className={"h-full flex justify-center items-center"}>
                <div className={"bg-white rounded-4xl flex flex-col gap-10 shadow-md items-center justify-center p-10 border-1 border-gray-300"}>
                    <h3 className={"text-center text-2xl font-bold"}>Forget Password</h3>
                    <div className={"flex flex-col gap-1 justify-center items-center"}>
                        <div className={'relative flex items-center'}>
                            <IoMail className={"absolute left-4"}/>
                            <Input
                                serial={"auth"}
                                onChange={(e) => setPassword(e.target.value)}
                                value={password}
                                name="NewPassword"
                                type="password"
                                placeholder="New Password"
                                required />
                        </div>

                        <div className={'relative flex items-center'}>
                            <IoMail className={"absolute left-4"}/>
                            <Input
                                serial={"auth"}
                                onChange={(e) => setConfirmPassword(e.target.value)}
                                value={confirmPassword}
                                name="ConfirmPassword"
                                type="password"
                                placeholder="Confirm Password"
                                required />
                        </div>
                    </div>
                    <Button onClick={changePassword}>Send to Email</Button>
                </div>
            </div>

            {isNotif && (
                <NotificationPopup type={type} message={notifMessage} func={popupToggle}/>
            )}
        </div>
    );
}

export default ChangePassword;