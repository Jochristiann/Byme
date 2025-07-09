import {Input} from "../Components/Interactive/Input.tsx";
import {Button} from "@/Components/Interactive/Button.tsx";
import {useState} from "react";
import {FcGoogle} from "react-icons/fc";
import {IoLockClosed, IoMail} from "react-icons/io5";
import {Link} from "react-router-dom";

function Login() {
    const [username, setUsername] = useState("")
    const [password, setPassword] = useState("")
    async function login(){

    }

    async function loginByGoogle(){

    }
    return (
        <div className={"w-screen h-screen bg-primary text-word"}>
            <div className={"h-full flex justify-center items-center"}>
                <div className={"bg-white rounded-4xl flex flex-col gap-10 shadow-md  p-10 border-1 border-gray-300"}>
                    <div className={"flex flex-col gap-1 justify-center items-center"}>
                        <h1 className={"text-center text-2xl font-bold"}>Sign In to</h1>
                        <h1 className={"text-center text-4xl font-bold"}>ByMe</h1>
                    </div>
                    <div className={"flex flex-col gap-2"}>
                        <div className={'relative flex items-center'}>
                            <IoMail className={"absolute left-4"}/>
                            <Input
                                serial={"auth"}
                                onChange={(e) => setUsername(e.target.value)}
                                value={username}
                                name="username"
                                type="text"
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
        </div>
    );
}

export default Login;