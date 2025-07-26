import {Input} from "@/Components/Interactive/Input.tsx";
import PostCard from "@/Components/Core/PostCard.tsx";
import {useState} from "react";


function Explore() {

    const [search, setSearch] = useState("")

    const title = search == "" ? "Today's Trend" : "Related Posts"

    return (
        <div className={"flex flex-col p-10 gap-4"}>
            <Input serial={"search"} color={"cream"} placeholder={"What's in your mind?"} onChange={(e)=>{setSearch(e.target.value)}} />
            <div className={"flex flex-col py-2 gap-5"}>
                <h3 className={"font-bold"}>{title}</h3>
                <div className={"grid grid-cols-1 lg:grid-cols-3 gap-2"}>
                    {[...Array(20).keys()].map((_) => (
                        <PostCard/>
                    ))}
                </div>
            </div>
        </div>
    );
}

export default Explore;