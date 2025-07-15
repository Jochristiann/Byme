import NavigationBar from "@/Components/Informative/NavigationBar.tsx";
import { Button } from "@/Components/Interactive/Button";
import {useNavigate} from "react-router-dom";
import {title} from "@/FrontUtils/Library.ts";
import Footer from "@/Components/Informative/Footer.tsx";

function Home() {

    const navigate = useNavigate();

    return (
        <div className={"w-screen h-screen"}>
            <NavigationBar/>
            <div className={"flex flex-col gap-5"}>
                <div className={"pt-40 px-32 pb-20 flex flex-row justify-between items-center py-5"}>
                    <div className={"flex flex-col gap-10 justify-center"}>
                        <div className={"flex flex-col gap-2 justify-center"}>
                            <h3>Welcome to <b>{title}</b></h3>
                            <h5><b>{title}</b> is the social media platform to know the world through internet</h5>
                        </div>
                        <Button
                            onClick={()=>{navigate("/auth/register")}}
                        >Get Started!</Button>
                    </div>
                    <img alt={"Banner 1"}/>
                </div>

                <div className={"px-32 py-20 flex flex-col gap-10 justify-center items-center bg-primary"}>
                    <h3 className={"font-bold"}>Explore the World!</h3>
                    <p>
                        We are providing you to share your experience, fun stories, and unforgettable memories.
                        You can meet other people from the other country.
                    </p>
                </div>

                <div className={"px-32 py-20 flex flex-col gap-10 justify-center items-center"}>

                </div>
            </div>
            <Footer/>
        </div>
    );
}

export default Home;