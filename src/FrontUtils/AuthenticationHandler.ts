import {apiAuth} from "@/Miscellaneous/api.tsx";


export const registerHandler = async (username:string, email:string, password:string) => {
    try{
        return await apiAuth.post("/register", {username, email, password})
    }catch(error){
        throw error;
    }
}

export const loginHandler = async (email:string, password:string) => {
    try{
        return await apiAuth.post("/login", {email, password})
    }catch(error){
        throw error;
    }
}