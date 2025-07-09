import {StrictMode} from "react";
import {createRoot} from "react-dom/client";
import './styles.css'
import Login from "./AuthenticationPage/Login.tsx";
import Register from "./AuthenticationPage/Register.tsx";
import {createBrowserRouter, RouterProvider} from "react-router-dom";

const router = createBrowserRouter([
    {
        path: "/",
        element: <Login/>
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
    }
    ])

createRoot(document.getElementById('root')!).render(
    <StrictMode>
        <RouterProvider router={router} />
    </StrictMode>
)
