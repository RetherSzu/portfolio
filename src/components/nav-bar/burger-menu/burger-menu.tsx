import { useEffect, useState } from "react";
//
import BurgerItem from "./burger-item";
import { configNavBar } from "../config-nav-bar";

const BurgerMenu = () => {

    const [isOpen, setIsOpen] = useState(false);

    const handleClick = () => setIsOpen(!isOpen);

    // Close the menu when the window is resized
    useEffect(() => {
        // Check if window is not undefined to avoid issues during SSR
        if (typeof window !== "undefined") {
            const handleResize = () => setIsOpen(false);
            window.addEventListener("resize", handleResize);

            // Cleanup event listener on component unmount
            return () => window.removeEventListener("resize", handleResize);
        }
    }, []);

    return (
        <>
            <nav className={`${isOpen ? "absolute" : "hidden"} w-full left-0 top-[96px] bg-[#070707] py-8`}>
                <ul className="flex flex-col justify-between gap-8">
                    {configNavBar.map((item, index) => (
                        <BurgerItem key={index} href={item.path} text={item.label} />
                    ))}
                </ul>
            </nav>
            <button
                onClick={handleClick}
                className="z-10 md:hidden w-10 h-10 flex justify-center items-center hover:bg-[#607B967C] rounded-full p-2"
            >
                {isOpen ? (
                    <svg width="20" height="19" viewBox="0 0 20 19" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <rect x="1" y="16.9706" width="24" height="2" transform="rotate(-45 1 16.9706)"
                              fill="#D9D9D9" />
                        <rect x="2" width="24" height="2" transform="rotate(45 2 0)" fill="#D9D9D9" />
                    </svg>
                ) : (
                    <svg width="24" height="9" viewBox="0 0 24 9" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <rect width="24" height="2" fill="#FFF" />
                        <rect x="9" y="7" width="15" height="2" fill="#FFF" />
                    </svg>
                )}
            </button>
        </>
    );
};

export default BurgerMenu;
