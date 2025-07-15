import {title} from "@/FrontUtils/Library.ts";
import {Link} from "react-router-dom";
import {IoLogoInstagram} from "react-icons/io";
import {FaFacebookF, FaLinkedinIn} from "react-icons/fa";

function Footer() {
    return (
        <div className="bg-primary">
            <div className={"px-32 py-16"}>
                <div className={"flex flex-row justify-between items-center"}>
                    <div className={"flex flex-col gap-3"}>
                        <div>
                            <h4 className={"font-bold"}>{title}</h4>
                        </div>

                        <div className={"flex flex-row gap-15 text-sm"}>
                            <div className={"flex flex-col gap-3"}>
                                <h5 className={"font-bold"}>Overview</h5>
                                <div className={"flex flex-col gap-1 justify-center"}>
                                    <Link to={"/about-us"}>About Us</Link>
                                    <p>Privacy Policy</p>
                                    <p>Terms & Conditions</p>
                                </div>
                            </div>

                            <div className={"flex flex-col gap-3"}>
                                <h5 className={"font-bold"}>Career</h5>
                                <div className={"flex flex-col gap-1 justify-center"}>
                                    <Link to={"/our-team"}>Our Team</Link>
                                    <Link to={"/career"}>Career Path</Link>
                                    <p>Affiliates</p>
                                    <p>Testimonials</p>
                                </div>
                            </div>

                            <div className={"flex flex-col gap-3"}>
                                <h5 className={"font-bold"}>Follow Us</h5>
                                <div className={"flex flex-row gap-1 items-center"}>
                                    <IoLogoInstagram size={24} className={"p-0.5 bg-white rounded-full"}/>
                                    <FaLinkedinIn size={20} className={"p-0.5 bg-white rounded-full"}/>
                                    <FaFacebookF size={20} className={"p-0.5 bg-white rounded-full"}/>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div className={"flex flex-col gap-1 items-center justify-center text-sm"}>
                        <p>Copyright @2025 {title}. All right reserved.</p>
                    </div>
                </div>
            </div>
        </div>
    );
}

export default Footer;