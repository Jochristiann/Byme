import test from "@/assets/Profile/VanessaKirby.jpg"
import {Button} from "@/Components/Interactive/Button.tsx";
import PostCard from "@/Components/Core/PostCard.tsx";
import {MdVerified} from "react-icons/md";
// import {CgProfile} from "react-icons/cg";
function Profile() {


    function pronouncation (gender:string){
        if (gender === "Male"){
            return "He/Him"
        } else if (gender === "Female"){
            return "She/Her"
        }
        return ""
    }
    return (
        <div className={"min-h-screen w-full h-full p-10"}>
            <div className={"flex flex-col w-full gap-10"}>
                <div className={"flex flex-col lg:flex-row gap-2 lg:gap-5 items-center"}>
                    <div className={"flex flex-col lg:flex-row gap-5 w-full items-center justify-center lg:justify-start"}>
                        <img src={test} alt="Profile" className={"object-cover rounded-full size-32 lg:size-56 border-primary border-2"}/>
                        {/*<CgProfile size={48}/>*/}
                        <div className={"flex flex-col gap-2 items-center lg:items-start"}>
                            <div className={"flex flex-row gap-7 items-center justify-center lg:justify-start"}>
                                <div className={"flex flex-col gap-0.25 items-center"}>
                                    <h5 className={"font-semibold"}>100</h5>
                                    <p className={"text-sm"}>Posts</p>
                                </div>

                                <div className={"flex flex-col gap-0.25 items-center"}>
                                    <h5 className={"font-semibold"}>1,894,704</h5>
                                    <p className={"text-sm"}>Followers</p>
                                </div>

                                <div className={"flex flex-col gap-0.25 items-center"}>
                                    <h5 className={"font-semibold"}>50</h5>
                                    <p className={"text-sm"}>Followings</p>
                                </div>
                            </div>
                            <div className={"flex flex-col lg:flex-row gap-0.5 lg:gap-2 items-center justify-center lg:justify-start"}>
                                <div className={"flex flex-row gap-1 items-start"}>
                                    <h4 className={"font-bold"}>Vanessa Kirby</h4>
                                    <MdVerified className={"text-blue-700 size-6"}/>
                                </div>
                                <p className={"text-sm"}>{pronouncation("Female")}</p>
                            </div>
                            <p>UK Actress! </p>
                        </div>
                    </div>
                    <Button>Follow</Button>
                </div>

                <div>
                    <div className={"grid grid-cols-1 lg:grid-cols-3 gap-2"}>
                        {[...Array(100).keys()].map((_) => (
                            <PostCard/>
                        ))}
                    </div>
                </div>
            </div>
        </div>
    );
}

export default Profile;