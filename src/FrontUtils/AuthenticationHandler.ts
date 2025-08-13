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

export const changePasswordHandler = async (password:string) => {
    try{
        return await apiAuth.post("/change-password",
            JSON.stringify(password), {
            headers: { "Content-Type": "application/json"}
        })
    }catch(error){
        throw error;
    }
}

export const forgetPasswordHandler = async (email:string) => {
    try{
        return await apiAuth.post("/forget-password",
            JSON.stringify(email),{
            headers: { "Content-Type": "application/json" }
        })
    }catch(error){
        throw error;
    }
}