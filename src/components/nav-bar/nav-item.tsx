import React from "react";
import Link from "next/link";
import { usePathname } from "next/navigation";

type NavItemProps = {
    href: string;
    text: string;
}

const NavItem: React.FC<NavItemProps> = React.memo(({ href, text }) => {
    const pathname = usePathname();
    const isActive = pathname === href;

    return (
        <li className="flex-1 border-r-2 border-[#1E2D3D] px-10 h-20 flex items-center">
            <Link
                href={href}
                className={`font-fira flex justify-center hover:text-[#607B96] items-center lowercase font-[450] group text-lg text-[#607B967F] ${isActive && "!text-[#FFF]"}`}
            >
                <p className={`font-fira text-[#FFFFFF7F] text-lg group-hover:text-[#C778DD] ${isActive && "!text-[#C778DD]"}`}>#</p>
                _{text}
            </Link>
        </li>
    );
});

export default NavItem;
