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
import ForYourPage from "@/Routes/ForYourPage.tsx";
import Profile from "@/Routes/Profile.tsx";
import EditProfile from "@/Routes/EditProfile.tsx";
import ForgotPassword from "@/Pages/ForgotPassword.tsx";
import CreatePost from "./Routes/CreatePost";

createRoot(document.getElementById('root')!).render(
    <React.StrictMode>
        <Router>
            <Routes>
                <Route path="/" element={<Mains />}>
                    <Route index element={<Home />} />
                    <Route path="home" element={<Home />} />
                    <Route path="for-your-page" element={<ForYourPage />} />
                    <Route path="explore" element={<Explore />} />
                    <Route path="post" element={<PostDetail />} />
                    <Route path="create-post" element={<CreatePost />} />
                    <Route path="profile">
                        <Route path={":username"} element={<Profile />} />
                        <Route path="edit-profile" element={<EditProfile />} />
                    </Route>
                </Route>

                <Route path={"auth"}>
                    <Route path="login" element={<Login />} />
                    <Route path="register" element={<Register />} />
                    <Route path="forget-password" element={<ForgotPassword />}/>
                </Route>
            </Routes>
        </Router>
    </React.StrictMode>
)
