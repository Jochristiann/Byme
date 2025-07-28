import {Input} from "@/Components/Interactive/Input.tsx";
import PostCard from "@/Components/Core/Post/PostCard.tsx";
import {useState} from "react";
import MusicCard from "@/Components/Core/Music/MusicCard.tsx";
import banner2 from "@/assets/Home/Banner 2.jpeg";

function Explore() {

    const [search, setSearch] = useState("")

    const title = search == "" ? "Today's Trend" : "Related Posts"

    return (
        <div className={"flex flex-col p-10 gap-4"}>
            <Input serial={"search"} color={"cream"} placeholder={"What's in your mind?"} onChange={(e)=>{setSearch(e.target.value)}} />

            <img src={banner2} alt={"Banner"} className={"hidden md:block w-full h-36 shadow-xl object-cover rounded-xl"}/>

            <div className={"flex flex-col py-2 gap-5"}>
                <h3 className={"font-bold"}>{title}</h3>
                <div className={"grid grid-cols-1 lg:grid-cols-3 gap-2"}>
                    {[...Array(3).keys()].map((_) => (
                        <PostCard/>
                    ))}
                </div>
            </div>

            {title == "Today's Trend" && (
                <>
                    <div className={"flex flex-col py-2 gap-5"}>
                        <h4 className={"font-bold"}>Based on Your Preferences</h4>
                        <div className={"grid grid-cols-1 lg:grid-cols-4 gap-2"}>
                            {[...Array(8).keys()].map((_) => (
                                <PostCard/>
                            ))}
                        </div>
                    </div>

                    <div className={"flex flex-col py-2 gap-5"}>
                        <h4 className={"font-bold"}>Popular Musics</h4>
                        <div className={"grid grid-cols-1 lg:grid-cols-4 gap-2"}>
                            {[...Array(8).keys()].map((_) => (
                                <MusicCard/>
                            ))}
                        </div>
                    </div>
                </>
            )}

        </div>
    );
}

export default Explore;