import logo from "../../assets/Logo.png"
import {Link, useNavigate} from "react-router-dom";
import {Button} from "@/Components/Interactive/Button.tsx";
function NavigationBar() {

    const navigate = useNavigate()
    const pathLink = [
        {path: "/home", name: "Home"},
        {path: "/for-your-page", name: "FYP"},
        {path: "/home", name: "Explore"},
    ]

    function toLogin(){
        navigate("/auth/login")
    }

    return (
        <div className={"w-full h-28 bg-primary py-10 px-5 flex items-center fixed"}>
            <div className={"w-full flex flex-row justify-between items-center"}>
                <div className={"flex items-center justify-center"}>
                    <img src={logo} alt={"Logo"} className={"size-48"}/>
                    <div className={"flex flex-row gap-4 items-center font-bold"}>
                        {pathLink.map((link) => (
                            <Link to={link.path} key={link.path}>{link.name}</Link>
                        ))}
                    </div>
                </div>
                <div className={"flex items-center justify-center"}>
                    <Button onClick={toLogin}>Sign In</Button>
                    {/*<img alt={"Logo"} className={"size-48"}/>*/}
                    {/*<div className={"flex flex-row gap-4 items-center"}>User</div>*/}
                </div>
            </div>
        </div>
    );
}

export default NavigationBar;