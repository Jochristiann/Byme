import {createRoot} from "react-dom/client";
import './styles.css'
import Login from "@/Pages/Login.tsx";
import Register from "@/Pages/Register.tsx";
import {BrowserRouter as Router, Routes, Route} from "react-router-dom";
import Home from "@/Routes/Home.tsx";
import React from "react";
import Mains from "@/Pages/Mains.tsx";
import Explore from "@/Routes/Explore.tsx";
import PostDetail from "@/Routes/PostDetail.tsx";

createRoot(document.getElementById('root')!).render(
    <React.StrictMode>
        <Router>
            <Routes>
                <Route path="/" element={<Mains/>}>s
                    <Route index element={<Home/>}/>
                    <Route path="/home" element={<Home/>}/>
                    <Route path="/for-your-page" element={<Explore/>}/>
                    <Route path="/explore" element={<Explore/>}/>
                    <Route path="/post" element={<PostDetail/>}/>
                </Route>
                <Route path="/auth/login" element={<Login/>}/>
                <Route path="/auth/register" element={<Register/>}/>
            </Routes>
        </Router>
    </React.StrictMode>
)
