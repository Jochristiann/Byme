import NavigationBar from "@/Components/Informative/NavigationBar.tsx";
import Footer from "@/Components/Informative/Footer.tsx";
import {Outlet, useLocation} from "react-router-dom";

function Mains() {
    const location = useLocation();
    const hideFooter = location.pathname === "/for-your-page";

    return (
        <div className={"w-screen min-h-screen h-full flex flex-row"}>
            <NavigationBar/>
            <div className={"flex flex-col pt-25 lg:pl-40 lg:pt-0 w-full"}>
                <div className={"flex-grow-1"}>
                    <Outlet/>
                </div>

                {!hideFooter && <Footer />}
            </div>
        </div>
    );
}

export default Mains;