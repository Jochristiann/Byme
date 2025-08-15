import {apiAuth} from "@/Miscellaneous/api.tsx";


export const registerHandler = async (name:string, email:string, password:string) => {
    try{
        return await apiAuth.post("/auth/register", {name, email, password})
    }catch(error){
        throw error;
    }
}

export const loginHandler = async (email:string, password:string) => {
    try{
        return await apiAuth.post("/auth/login", {email, password})
    }catch(error){
        throw error;
    }
}

export const changePasswordHandler = async (password:string) => {
    try{
        return await apiAuth.post("/auth/change-password",
            JSON.stringify(password), {
            headers: { "Content-Type": "application/json"}
        })
    }catch(error){
        throw error;
    }
}

export const forgetPasswordHandler = async (email:string) => {
    try{
        return await apiAuth.post("/auth/send-email",
            JSON.stringify(email),{
            headers: { "Content-Type": "application/json" }
        })
    }catch(error){
        throw error;
    }
}