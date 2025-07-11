import {Input} from "../Components/Interactive/Input.tsx";
import {Button} from "@/Components/Interactive/Button.tsx";
import {useState} from "react";
import {FcGoogle} from "react-icons/fc";
import {IoLockClosed, IoMail} from "react-icons/io5";
import {Link, useNavigate} from "react-router-dom";
import ErrorPopup from "@/Components/Interactive/ErrorPopup.tsx";

function Login() {
    const navigate = useNavigate();
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
            navigate("/home")
        }catch(err){
            setErrorMessage("Something went wrong. Please try again later.")
            setIsError(true)
        }
    }

    async function loginByGoogle(){
        try{
            navigate("/home")
        }catch(err){

        }
    }
    return (
        <div className={"w-screen h-screen bg-primary"}>
            <div className={"h-full flex justify-center items-center"}>
                <div className={"bg-white rounded-4xl flex flex-col gap-10 shadow-md  p-10 border-1 border-gray-300"}>
                    <div className={"flex flex-col gap-1 justify-center items-center"}>
                        <h2 className={"text-center text-2xl font-bold"}>Sign In to</h2>
                        <h2 className={"text-center text-4xl font-bold"}>ByMe</h2>
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
                <ErrorPopup message={errorMessage} func={popupToggle}/>
            )}
        </div>
    );
}

export default Login;