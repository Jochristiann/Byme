import {apiUser} from "@/Miscellaneous/api.tsx"

export const followUser = async (username:string) => {
    try{
        return await apiUser.post("/follow-user",
            JSON.stringify(username), {
            headers: { "Content-Type": "application/json"}
        })
    }catch(error){
        throw error;
    }
}

export const getUserByUsernameHandler = async (username:string) => {
    try{
        return await apiUser.get("/get-user-by-username/"+username)
    }catch(error){
        throw error;
    }
}