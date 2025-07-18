import logo from "../../assets/Logo.png"
import {Link, useNavigate} from "react-router-dom";
import {Button} from "@/Components/Interactive/Button.tsx";
import {IoClose} from "react-icons/io5";
import {RxHamburgerMenu} from "react-icons/rx";
import { motion } from "framer-motion";
import {useState} from "react";

function NavigationBar() {

    const navigate = useNavigate()
    const user = null

    const [isOpen, setIsOpen] = useState(false);
    const pathLink = [
        {path: "/home", name: "Home"},
        {path: "/for-your-page", name: "FYP"},
        {path: "/explore", name: "Explore"},
    ]

    function toLogin(){
        navigate("/auth/login")
    }

    function togglePopup() {
        setIsOpen(!isOpen);
    }

    return (
        <div className={"w-full h-25 lg:w-50 lg:h-full bg-secondary flex fixed lg:border-r-2 lg:border-black shadow-xl"}>
            <div className={"w-full py-10 px-4 flex flex-row lg:flex-col justify-between items-center"}>
                <button className={"flex lg:hidden flex-col gap-3 items-center cursor-pointer"} onClick={togglePopup}>
                    <motion.div
                        initial={{ rotate: 0 }}
                        animate={{ rotate: isOpen ? 90 : 0 }}
                        transition={{ duration: 0.2 }}
                        className="z-50"
                    >
                        {isOpen ? <IoClose size={28} /> : <RxHamburgerMenu size={28} />}
                    </motion.div>
                </button>
                <div className={"flex flex-col items-center justify-center"}>
                    <img src={logo} alt={"Logo"} className={"size-24 p-4"}/>
                    <nav className={"hidden lg:flex flex-col gap-4 items-center font-bold"}>
                        {pathLink.map((link) => (
                            <Link to={link.path} key={link.path}>{link.name}</Link>
                        ))}
                    </nav>
                </div>
                <div className={"flex items-center justify-center"}>
                    <Button onClick={toLogin}>Sign In</Button>
                    {/*<img alt={"Logo"} className={"size-48"}/>*/}
                    {/*<div className={"flex flex-row gap-4 items-center"}>User</div>*/}
                </div>
            </div>

            {isOpen && (
                <div className={"lg:hidden fixed w-full bg-black/50 inset-0"}>
                    <motion.div
                        initial={{ x: "-100%" }}
                        animate={{ x: 0 }}
                        exit={{ x: "-100%" }}
                        transition={{ type: "spring", stiffness: 300, damping: 30 }}
                        className={`left-0 top-0 pt-20 z-30 flex flex-col gap-6 h-full w-3/4 max-w-xs shadow-xl bg-primary`}
                    >
                        {/*{user && ( // Use user from context*/}
                        {/*    <div className="flex flex-col items-center mb-4 border-b border-white/20 pb-4">*/}
                        {/*        <img src={profileImageUrl} alt="Profile" className="h-20 w-20 rounded-full object-cover border-2 border-white mb-2" />*/}
                        {/*        <p className="font-semibold text-lg truncate max-w-[calc(100%-2rem)]">{user.email}</p>*/}
                        {/*        <Link to="/profile" className="text-sm hover:underline mt-1" onClick={togglePopup}>View Profile</Link>*/}
                        {/*    </div>*/}
                        {/*)}*/}
                        <nav className={"flex flex-col gap-4 items-center justify-center w-full text-lg"}>
                            {pathLink.map(link => (
                                <Link key={link.path} to={link.path} className="px-8 pb-8 hover:bg-gray-200 w-full" onClick={togglePopup}>{link.name}</Link>
                            ))}
                        </nav>
                        <div className="mt-auto border-t border-white/20 pt-4 flex justify-center items-center">
                            {user ? (
                                <button
                                    onClick={() => { togglePopup(); }}
                                    className="w-full text-left px-8 py-8 text-lg cursor-pointer hover:text-gray-200"
                                >
                                    Logout
                                </button>
                            ) : (
                                <Link to="/auth/login" className="px-8 py-8 text-lg hover:bg-gray-200 w-full" onClick={togglePopup}>Login</Link>
                            )}
                        </div>
                    </motion.div>
                </div>

            )}
        </div>
    );
}

export default NavigationBar;