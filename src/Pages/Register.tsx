import {IoLockClosed, IoMail} from "react-icons/io5";
import {Input} from "@/Components/Interactive/Input.tsx";
import {Button} from "@/Components/Interactive/Button.tsx";
import {Link, useNavigate} from "react-router-dom";
import {useState} from "react";
import {FaUser} from "react-icons/fa";
import NotificationPopup from "@/Components/Interactive/NotificationPopup.tsx";
import {registerHandler} from "@/FrontUtils/AuthenticationHandler.ts";
import {title} from "@/FrontUtils/Library.ts";


function Register() {
    const navigate = useNavigate();
    const [username, setUsername] = useState("")
    const [email, setEmail] = useState("")
    const [password, setPassword] = useState("")
    const [confirmPassword, setConfirmPassword] = useState("")
    const [errorMessage, setErrorMessage] = useState("")
    const [isError, setIsError] = useState<boolean>(false)

    function popupToggle(){
        setIsError(!isError)
    }

    function toFYP(){
        navigate("/for-your-page")
    }

    async function register(){
        if(!username || !email || !password || !confirmPassword){
            setErrorMessage("Please fill out all fields")
            setIsError(true)
            return
        }

        if(confirmPassword !== password){
            setErrorMessage("The password doesn't match")
            setIsError(true)
            return
        }

        try{
            await registerHandler(username, email, password)
            navigate("/home")
        }catch(error){
            setErrorMessage("Something went wrong. Please try again later.")
            setIsError(true)
        }

    }
    return (
        <div className={"w-screen h-screen bg-primary"}>
            <div className={"h-full flex justify-center items-center"}>
                <div className={"bg-white rounded-4xl flex flex-col items-center justify-center gap-10 shadow-md  p-10 border-1 border-gray-300"}>
                    <div className={"flex flex-col gap-1 justify-center items-center"}>
                        <h3 className={"text-center text-2xl font-bold"}>Sign Up to</h3>
                        <h3 className={"text-center text-4xl font-bold cursor-pointer"} onClick={toFYP}>{title}</h3>
                    </div>
                    <div className={"flex flex-col gap-2"}>
                        <div className={'relative flex items-center'}>
                            <FaUser className={"absolute left-4"}/>
                            <Input
                                serial={"auth"}
                                onChange={(e) => setUsername(e.target.value)}
                                value={username}
                                name="username"
                                type="text"
                                placeholder="Username"
                                required />
                        </div>

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
                        <div className={"flex flex-col gap-2"}>
                            <div className={'relative flex items-center'}>
                                <IoLockClosed className={"absolute left-4"}/>
                                <Input
                                    serial={"auth"}
                                    onChange={(e) => setPassword(e.target.value)}
                                    value={password}
                                    name="password"
                                    type="password"
                                    minLength={6}
                                    placeholder="Password"
                                    required />
                            </div>

                            <ul className={"pl-3 text-xs text-light"}>
                                <li>Password length must be more than 5 characters</li>
                                <li>Password must contain special character (.&*#)</li>
                            </ul>
                        </div>


                        <div className={'relative flex items-center'}>
                            <IoLockClosed className={"absolute left-4"}/>
                            <Input
                                serial={"auth"}
                                onChange={(e) => setConfirmPassword(e.target.value)}
                                value={confirmPassword}
                                name="confirmPassword"
                                type="password"
                                minLength={6}
                                placeholder="Confirm Password"
                                required />
                        </div>
                    </div>
                    <Button
                        onClick={register}
                        rounded={"max"}
                        size={"xl"}
                    >Sign Up</Button>
                    <p className={"text-center text-sm text-light"}>Already have an account? <Link className={"underline cursor-pointer"} to='/auth/login'>Sign in here</Link></p>
                </div>
            </div>
            {isError && (
                <NotificationPopup message={errorMessage} func={popupToggle}/>
            )}
        </div>
    );
}

export default Register;