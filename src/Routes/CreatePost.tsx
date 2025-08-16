import {MdEdit} from "react-icons/md";
import {useState} from "react";
import test from "@/assets/Profile/VanessaKirby.jpg"
import {Button} from "../Components/Interactive/Button";

function CreatePost() {
    const [isPictureHovered, setIsPictureHovered] = useState(false);
    const [caption, setCaption] = useState("");
    const [selectedFiles, setSelectedFiles] = useState<File[]>([]);
    const [previewUrls, setPreviewUrls] = useState<string[]>([]);

    const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const files = e.target.files;
        if (!files) return;

        const fileArray = Array.from(files);
        console.log("Selected:", fileArray);

        if (fileArray.some((f) => f.type.startsWith("video/"))) {
            const videoFile = fileArray.find((f) => f.type.startsWith("video/"))!;
            setSelectedFiles([videoFile]);
            setPreviewUrls([URL.createObjectURL(videoFile)]);
        } else {
            setSelectedFiles(fileArray);
            setPreviewUrls(fileArray.map((file) => URL.createObjectURL(file)));
        }

        e.target.value = "";
    };

    return (
        <div className="min-h-screen w-full h-full p-10 flex flex-col gap-5 items-center">
            <h2 className="font-bold">Create Your Own Story</h2>
            <div
                className="relative"
                onMouseEnter={() => setIsPictureHovered(true)}
                onMouseLeave={() => setIsPictureHovered(false)}
            >
                {previewUrls.length > 0 ? (
                    selectedFiles[0]?.type.startsWith("video/") ? (
                        <video
                            src={previewUrls[0]}
                            controls
                            className="object-cover rounded-xl w-128 h-96 border-primary border-2"
                        />
                    ) : (
                        <div className="grid grid-cols-2 gap-2 w-128 h-96 overflow-auto">
                            {previewUrls.map((url, idx) => (
                                <img
                                    key={idx}
                                    src={url}
                                    alt={`Preview-${idx}`}
                                    className="object-cover rounded-xl border border-primary w-full h-48"
                                />
                            ))}
                        </div>
                    )
                ) : (
                    <div className="w-128 h-96 flex items-center justify-center border-2 border-primary rounded-xl bg-gray-100 text-gray-500">
                        No file chosen
                    </div>
                )}

                {isPictureHovered && (
                    <div className="absolute inset-0 bg-black/40 rounded-xl flex justify-center items-center ">
                        <label className="flex flex-col items-center justify-center cursor-pointer text-white gap-1">
                            <MdEdit className="size-6" />
                            <p className="text-sm">Upload Image(s) / Video</p>
                            <input
                                type="file"
                                accept="image/*,video/*"
                                multiple
                                className="hidden"
                                onChange={handleFileChange}
                            />
                        </label>
                    </div>
                )}
            </div>

            <div className="w-1/2 flex flex-col gap-5 items-center justify-center">
                <div className="w-full flex flex-col gap-0.5">
                    <h5>Caption</h5>
                    <textarea
                        className="min-h-50 px-5 py-2 rounded-xl border-2 border-primary resize-none"
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