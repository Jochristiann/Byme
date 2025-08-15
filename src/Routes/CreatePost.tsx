import {MdEdit} from "react-icons/md";
import {Input} from "../Components/Interactive/Input";
import {useState} from "react";
import test from "@/assets/Profile/VanessaKirby.jpg"
import {Button} from "../Components/Interactive/Button";

function CreatePost() {

    const [isPictureHovered, setIsPictureHovered] = useState(false)
    const [caption, setCaption] = useState("")

    return (
        <div className={"min-h-screen w-full h-full p-10 flex flex-col gap-5 items-center"}>
            <h2 className={"font-bold"}>Create Your Own Story</h2>
            <div
                className={"relative"}
                onMouseEnter={() => {
                    setIsPictureHovered(true)
                }}
                onMouseLeave={() => {
                    setIsPictureHovered(false)
                }}
            >
                <img src={test} alt="Profile"
                     className={"object-cover rounded-xl w-128 h-96 border-primary border-2"}/>
                {isPictureHovered && (
                    <div
                        className="absolute inset-0 bg-black/40 rounded-xl flex justify-center items-center ">
                        <label className="flex flex-col items-center justify-center cursor-pointer text-white gap-1">
                            <MdEdit className="size-6"/>
                            <p className="text-sm">Upload Videos</p>
                            <Input type="file" className="hidden"/>
                        </label>
                    </div>
                )}
            </div>
            <div className={"w-1/2 flex flex-col gap-5 items-center justify-center"}>
                <div className={"w-full flex flex-col gap-0.5"}>
                    <h5>Caption</h5>
                    <textarea
                        className={"min-h-50 px-5 py-2 rounded-xl border-2 border-primary resize-none"}
                        placeholder="Share your experience..."
                        value={caption}
                        onChange={(e) => setCaption(e.target.value)}
                    />
                </div>
                <Button>Post!</Button>
            </div>

        </div>
    );
}

export default CreatePost;