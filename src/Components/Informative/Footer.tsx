import {title} from "@/FrontUtils/Library.ts";
import {Link} from "react-router-dom";
import {IoLogoInstagram} from "react-icons/io";
import {FaFacebookF, FaLinkedinIn} from "react-icons/fa";
import logo from "@/assets/Logo.png"

function Footer() {
    return (
        <div className="bg-primary">
            <div className={"px-8 md:px-32 py-12"}>
                <div className={"flex flex-col md:flex-row gap-5 md:gap-0 justify-center md:justify-between items-center"}>
                    <div className={"flex flex-col gap-3 justify-center md:justify-start"}>

                        <div className={"flex flex-col md:flex-row justify-center md:justify-start items-center"}>
                            <img src={logo} alt={"Logo"} className={"size-24 p-4"}/>
                            <h4 className={"font-bold"}>{title}</h4>
                        </div>

                        <div className={"flex flex-col md:flex-row gap-5 md:gap-15 text-sm text-center md:text-left"}>
                            <div className={"flex flex-col gap-3"}>
                                <h5 className={"font-bold"}>Overview</h5>
                                <div className={"flex flex-col gap-1 justify-center"}>
                                    <Link to={"/about-us"}>About Us</Link>
                                    <p>Privacy Policy</p>
                                    <p>Terms & Conditions</p>
                                </div>
                            </div>

                            <div className={"flex flex-col gap-3"}>
                                <h5 className={"font-bold"}>Careers</h5>
                                <div className={"flex flex-col gap-1 justify-center"}>
                                    <Link to={"/our-team"}>Our Team</Link>
                                    <Link to={"/career"}>Opportunity</Link>
                                    <p>Affiliates</p>
                                    <p>Testimonials</p>
                                </div>
                            </div>

                            <div className={"flex flex-col gap-3"}>
                                <h5 className={"font-bold"}>Follow Us</h5>
                                <div className={"flex flex-row gap-1 items-center justify-center md:justify-start"}>
                                    <IoLogoInstagram size={24} className={"p-0.5 bg-white rounded-full"}/>
                                    <FaLinkedinIn size={20} className={"p-0.5 bg-white rounded-full"}/>
                                    <FaFacebookF size={20} className={"p-0.5 bg-white rounded-full"}/>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div className={"flex flex-col gap-1 items-end justify-end text-sm"}>
                        <p>Copyright @2025 {title}.</p>
                        <p>All right reserved.</p>
                    </div>
                </div>
            </div>
        </div>
    );
}

export default Footer;