import video from "@/assets/Videos/MISS DIOR, THE NEW FRAGRANCE - Christian Dior (720p, h264).mp4"
import {IoEyeSharp} from "react-icons/io5";
import {FaHeart} from "react-icons/fa";
import {CgProfile} from "react-icons/cg";
import {Button} from "@/Components/Interactive/Button.tsx";

function PostContent() {
    return (
        <div className={"min-h-screen h-full w-full p-5"}>
            <div className={"flex flex-col gap-5"}>
                <div className={"relative lg:static rounded-xl overflow-hidden"}>
                    <video src={video} autoPlay={true} loop className={"w-full object-cover rounded-xl mb-10 lg:mb-0"} controls={true}/>
                    <div className="lg:hidden absolute bottom-0 w-full">
                        <div className="bg-gradient-to-t from-black/70 to-transparent px-5 pt-8 pb-2.5 text-white text-sm">
                            <div className="flex flex-row justify-between gap-2">
                                <div className="flex flex-row gap-1 items-center">
                                    <IoEyeSharp size={14} />
                                    <p>1,000,000</p>
                                </div>
                                <div className="flex flex-row gap-1 items-center">
                                    <FaHeart size={10} />
                                    <p>1,231,328</p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
                <div className={"flex flex-col gap-5"}>
                    <div className={"flex flex-row justify-between items-center"}>
                        <div className={"flex flex-row gap-2 items-center"}>
                            <CgProfile size={48}/>
                            <div className={"flex flex-col gap-0.5"}>
                                <h5>Christian Dior</h5>
                                <p className={"text-sm"}>7,981,991 followers</p>
                            </div>
                        </div>
                        <Button>Follow</Button>
                    </div>
                    <div>
                        <p>
                            A new breath of love and optimism.
                            Natalie Portman calls out to wake up to the beauty of the world and always... To love.
                        </p>
                    </div>
                </div>
            </div>
        </div>
    );
}

export default PostContent;