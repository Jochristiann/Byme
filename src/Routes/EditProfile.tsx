import test from "@/assets/Profile/VanessaKirby.jpg"
import {Input} from "@/Components/Interactive/Input.tsx";
import {useState} from "react";
import {MdEdit} from "react-icons/md";
import {Button} from "@/Components/Interactive/Button.tsx";
import Dropdown from "@/Components/Interactive/Dropdown.tsx";


function EditProfile() {
    const [isPictureHovered, setIsPictureHovered] = useState(false)
    const [selectedGender, setSelectedGender] = useState("Prefer not to say")

    const options = [
        { label: "Male", value: "Male" },
        { label: "Female", value: "Female" },
        { label: "Prefer not to say", value: "Prefer not to say" },
    ];


    return (
        <div className={"min-h-screen w-full h-full p-10 flex flex-col gap-5 items-center"}>
            <div
                className={"relative"}
                onMouseEnter={()=>{setIsPictureHovered(true)}}
                onMouseLeave={()=>{setIsPictureHovered(false)}}
            >
                <img src={test} alt="Profile" className={"object-cover rounded-full size-32 lg:size-52 border-primary border-2"}/>
                {isPictureHovered && (
                    <div className="absolute inset-0 bg-black/40 rounded-full flex justify-center items-center size-32 lg:size-52">
                        <label className="flex flex-col items-center justify-center cursor-pointer text-white gap-1">
                            <MdEdit className="size-6" />
                            <p className="text-sm">Change Picture</p>
                            <Input type="file" className="hidden"/>
                        </label>
                    </div>
                )}
            </div>

            <div className={"w-1/2 flex flex-col gap-5 items-center justify-center"}>
                <div className={"w-full flex flex-col gap-0.5"}>
                    <h5>Username</h5>
                    <Input type="text" placeholder="Username"/>
                </div>

                <div className={"w-full flex flex-col gap-0.5"}>
                    <h5>Full Name</h5>
                    <Input type="text" placeholder="Full Name"/>
                </div>

                <div className={"w-full flex flex-col gap-0.5"}>
                    <h5>Gender</h5>
                    <Dropdown items={options}
                              selected={selectedGender}
                              onSelect={(val) => setSelectedGender(val)}
                    />
                </div>

                <div className={"w-full flex flex-col gap-0.5"}>
                    <h5>Email</h5>
                    <Input type="email" placeholder="Email"/>
                </div>

                <div className={"w-full flex flex-col gap-0.5"}>
                    <h5>Bio</h5>
                    <textarea
                        className={"h-20 px-5 py-2 rounded-xl border-2 border-primary resize-none"}
                        placeholder="Describe yourself"
                    />
                </div>

                <div className={"w-full flex flex-row gap-5 justify-end"}>
                    <Button>Cancel</Button>
                    <Button>Save</Button>
                </div>

            </div>
        </div>
    );
}

export default EditProfile;