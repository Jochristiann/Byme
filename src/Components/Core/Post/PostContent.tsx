import {CgProfile} from "react-icons/cg";
import {Button} from "@/Components/Interactive/Button.tsx";
import test
    from "@/assets/Videos/The Fantastic Four First Steps Official Teaser Only in Theaters July 25 - Marvel Entertainment (1080p, h264).mp4";
import {useNavigate} from "react-router-dom";
import VideoDisplay from "@/Components/Core/Video/VideoDisplay.tsx";
import {formatLikeNumber} from "@/FrontUtils/Library.ts";

function PostContent() {
    const navigate = useNavigate();

    function toProfile (){
        navigate("/profile");
    }

    return (
        <div className={"h-full w-full overflow-hidden"}>
            <div className={"flex flex-col gap-5"}>
                <VideoDisplay src={test} isActive={true}/>

                <div className={"flex flex-col gap-5 p-5"}>
                    <div className={"flex flex-row justify-between items-center"}>
                        <div className={"flex flex-row gap-2 items-center cursor-pointer"} onClick={toProfile}>
                            <CgProfile size={48} />
                            <div className={"flex flex-col gap-0.5"}>
                                <h5 className={"font-bold"}>Marvel Entertainment</h5>
                                <p className={"text-sm"}>{formatLikeNumber(7281103)} followers</p>
                            </div>
                        </div>
                        <Button>Follow</Button>
                    </div>

                    <p>
                        The Fantastic Four will arrive in the cinemas soon!!!!
                    </p>
                </div>
            </div>
        </div>
    );
}

export default PostContent;