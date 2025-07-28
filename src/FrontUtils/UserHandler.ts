import {apiUser} from "@/Miscellaneous/api.tsx"

export const followUser = async (followedId:string) => {
    try{
        return await apiUser.post("/follow-user", {followedId})
    }catch(error){
        throw error;
    }
}