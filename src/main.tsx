import {StrictMode} from "react";
import {createRoot} from "react-dom/client";
import './styles.css'
import Login from "./AuthenticationPage/Login.tsx";
import Register from "./AuthenticationPage/Register.tsx";
import {createBrowserRouter, RouterProvider} from "react-router-dom";
import Home from "@/GeneralPage/Home.tsx";

const router = createBrowserRouter([
    {
        path: "/",
        element: <Home/>
    },
    {
        path: "/auth",
        children: [{
            path: "login",
            element: <Login/>
        },
        {
            path: "register",
            element: <Register/>
        }],
    },
    {
        path: "/home",
        element: <Home/>
    }
    ])

createRoot(document.getElementById('root')!).render(
    <StrictMode>
        <RouterProvider router={router} />
    </StrictMode>
)
