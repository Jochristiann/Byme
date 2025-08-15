import {Input} from "../Components/Interactive/Input.tsx";
import {Button} from "@/Components/Interactive/Button.tsx";
import {useState} from "react";
import {FcGoogle} from "react-icons/fc";
import {IoLockClosed, IoMail} from "react-icons/io5";
import {Link, useNavigate} from "react-router-dom";
import NotificationPopup from "@/Components/Interactive/NotificationPopup.tsx";
import {title} from "@/FrontUtils/Library.ts";
import {loginHandler} from "../FrontUtils/AuthenticationHandler";
import {currentUser} from "../FrontUtils/StateManagement";

function Login() {
    const navigate = useNavigate();

    const setUser = currentUser((state) => state.setUser);
    const [email, setEmail] = useState("")
    const [password, setPassword] = useState("")
    const [errorMessage, setErrorMessage] = useState("")
    const [isError, setIsError] = useState<boolean>(false)

    function popupToggle(){
        setIsError(!isError)
    }

    async function login(){
        if(!email || !password){
            setErrorMessage("Please fill out all fields")
            setIsError(true)
            return
        }

        try{
            let response = await loginHandler(email, password)

            setUser({
                ...response.data.user,
                token:response.data.token
            });

            navigate("/home")
        }catch(err:any){
            const temp = "Something went wrong. Please try again later."
            const backendMessage = err.data?.message || temp
            setErrorMessage(backendMessage)
            setIsError(true)
        }
    }

    function toFYP(){
        navigate("/for-your-page")
    }

    async function loginByGoogle(){
        try{
            let response = await loginHandler(email, password)
            console.log(response)
            navigate("/home")
        }catch(err:any){
            const temp = "Something went wrong. Please try again later."
            const backendMessage = err.data?.message || temp
            setErrorMessage(backendMessage)
            setIsError(true)
        }
    }
    return (
        <div className={"w-screen h-screen bg-primary"}>
            <div className={"h-full flex justify-center items-center"}>
                <div className={"bg-white rounded-4xl flex flex-col gap-10 shadow-md  items-center justify-center p-10 border-1 border-gray-300"}>
                    <div className={"flex flex-col gap-1 justify-center items-center"}>
                        <h3 className={"text-center text-2xl font-bold"}>Sign In to</h3>
                        <h3 className={"text-center text-4xl font-bold cursor-pointer"} onClick={toFYP}>{title}</h3>
                    </div>
                    <div className={"flex flex-col gap-2"}>
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
                        <div className={"flex flex-col gap-1"}>
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
                            <div className={"w-full flex flex-row gap-1 text-xs text-center justify-center"}>
                                <p>Forget your password?</p>
                                <Link to={"/auth/forget-password"}>Click here</Link>
                            </div>
                        </div>

                    </div>
                    <div className={"flex flex-col gap-1 justify-center items-center"}>
                        <Button
                            onClick={login}
                            rounded={"max"}
                            size={"xl"}
                        >Sign In</Button>
                        <p>or</p>
                        <Button
                            onClick={loginByGoogle}
                            rounded={"max"}
                            size={"xl"}
                        >
                            <FcGoogle/>
                            Sign In with Google
                        </Button>
                    </div>
                    <p className={"text-center text-sm text-light"}>Don't have any account? <Link className={"underline cursor-pointer"} to='/auth/register'>Sign up here</Link></p>
                </div>
            </div>
            {isError && (
                <NotificationPopup message={errorMessage} func={popupToggle}/>
            )}
        </div>
    );
}

export default Login;