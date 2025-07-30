import { Button } from "@/Components/Interactive/Button";
import {useNavigate} from "react-router-dom";
import {title} from "@/FrontUtils/Library.ts";
import banner1 from "@/assets/Home/Banner 1.jpg";
import {FaArrowRightLong} from "react-icons/fa6";


function Home() {

    const navigate = useNavigate();

    return (
        <div className={"min-h-screen w-full h-full flex flex-col"}>
            <div className={"flex flex-row justify-between items-center"}>
                <div className={"px-32 pb-20 flex flex-col gap-10 justify-center items-center md:items-start"}>
                    <div className={"flex flex-col gap-2 justify-center text-center md:text-start"}>
                        <h3>Welcome to <br/><h1 className={"font-bold"}>{title}</h1></h3>
                        <h5><b>{title}</b> is the social media platform to know the world through internet</h5>
                    </div>
                    <Button
                        onClick={()=>{navigate("/auth/register")}}
                        className={"hover:underline"}
                    >
                        Get Started
                        <FaArrowRightLong/>
                    </Button>
                </div>
                <img src={banner1} alt={"Banner 1"} className={"hidden md:block w-1/2 shadow-xl"}/>
            </div>

            <div className={"px-32 py-20 flex flex-col gap-10 justify-center items-center bg-primary text-center"}>
                <h1 className={"font-bold"}>Explore the World!</h1>
                <p>
                    We are providing you to share your experience, fun stories, and unforgettable memories.
                    You can meet other people from the other country.
                </p>
            </div>

            <div className={"px-32 py-20 flex flex-col gap-10 justify-center items-center"}>
                <div className={"flex flex-col gap-2 justify-center"}>

                </div>
            </div>
        </div>
    );
}

export default Home;