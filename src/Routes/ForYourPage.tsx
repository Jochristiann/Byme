import { useEffect, useRef, useState } from "react";
import VideoCard from "@/Components/Core/Video/VideoCard.tsx";
import test from "@/assets/Videos/The Fantastic Four First Steps Official Teaser Only in Theaters July 25 - Marvel Entertainment (1080p, h264).mp4";
import testP from "@/assets/Profile/VanessaKirby.jpg";

function ForYourPage() {
    const [activeIndex, setActiveIndex] = useState(0);
    const videoRefs = useRef<HTMLDivElement[]>([]);

    const videoList = [
        { src: test, profile: testP },
        { src: test, profile: testP },
        { src: test, profile: testP },
    ];

    useEffect(() => {
        const observer = new IntersectionObserver(
            (entries) => {
                entries.forEach((entry, i) => {
                    if (entry.isIntersecting) {
                        setActiveIndex(i);
                    }
                });
            },
            { threshold: 0.75 }
        );

        videoRefs.current.forEach((el) => el && observer.observe(el));

        return () => {
            videoRefs.current.forEach((el) => el && observer.unobserve(el));
        };
    }, []);

    return (
        <div className="h-screen overflow-y-scroll snap-y snap-mandatory">
            {videoList.map((video, idx) => (
                <div
                    key={idx}
                    className="snap-start h-screen w-full"
                    ref={(el) => (videoRefs.current[idx] = el!)}
                >
                    <VideoCard
                        src={video.src}
                        profileImg={video.profile}
                        isActive={activeIndex === idx}
                    />
                </div>
            ))}
        </div>
    );
}

export default ForYourPage;
