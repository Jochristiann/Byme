import NavigationBar from "@/Components/Informative/NavigationBar.tsx";
import Footer from "@/Components/Informative/Footer.tsx";
import {Outlet} from "react-router-dom";

function Mains() {
    return (
        <div className={"w-screen min-h-screen h-full flex flex-row"}>
            <NavigationBar/>
            <div className={"flex flex-col pt-25 lg:pl-40 lg:pt-0 w-full"}>
                <div className={"flex-grow-1"}>
                    <Outlet/>
                </div>
                <Footer/>
            </div>
        </div>
    );
}

export default Mains;