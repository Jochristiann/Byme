import {Input} from "@/Components/Interactive/Input.tsx";
import PostCard from "@/Components/Core/PostCard.tsx";


function Explore() {
    return (
        <div className={"flex flex-col p-10 gap-5"}>
            <Input serial={"search"} color={"cream"}/>
            <div className={"flex flex-col p-10 gap-5"}>
                <div>

                </div>

                <div className={"flex flex-col py-2 gap-5"}>
                    <h3 className={"font-bold"}>Related Posts</h3>
                    <div className={"grid grid-cols-5 gap-1"}>
                        {[...Array(10).keys()].map((_) => (
                            <PostCard/>
                        ))}
                    </div>
                </div>
            </div>
        </div>
    );
}

export default Explore;