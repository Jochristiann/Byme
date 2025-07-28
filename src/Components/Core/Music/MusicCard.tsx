import testP from "@/assets/Profile/VanessaKirby.jpg"
import {AnimatePresence, motion} from "framer-motion";
import {FaPause, FaPlay} from "react-icons/fa";
import {useState} from "react";

function MusicCard() {

    const [isMusicPlayed, setIsMusicPlayed] = useState(true)
    function togglePlay() {
        setIsMusicPlayed(!isMusicPlayed);
    }

    return (
        <div className={"bg-primary rounded-xl p-2 flex flex-row gap-2 items-center justify-between"}>
            <div className={"flex flex-row gap-2 items-center justify-start"}>

                <img src={testP} alt = "Music Post" className={"rounded-xl size-16 object-cover"}/>
                <div className={"flex flex-col gap-1"}>
                    <p className={"font-bold"}>Fantastic Four Soundtrack</p>
                    <p className={"text-sm"}>Vanessa Kirby</p>
                </div>
            </div>
            <div className={"bg-white p-2 rounded-full flex items-center justify-center text-black"} onClick={togglePlay}>
                <AnimatePresence mode="wait">
                    {isMusicPlayed ? (
                        <motion.div
                            key="check"
                            initial={{ opacity: 0, scale: 0.8 }}
                            animate={{ opacity: 1, scale: 1 }}
                            exit={{ opacity: 0, scale: 0.8 }}
                            transition={{ duration: 0.2 }}
                        >
                            <FaPlay/>
                        </motion.div>
                    ) : (
                        <motion.div
                            key="plus"
                            initial={{ opacity: 0, scale: 0.8 }}
                            animate={{ opacity: 1, scale: 1 }}
                            exit={{ opacity: 0, scale: 0.8 }}
                            transition={{ duration: 0.2 }}
                        >
                            <FaPause />
                        </motion.div>
                    )}
                </AnimatePresence>
            </div>
        </div>
    );
}

export default MusicCard;