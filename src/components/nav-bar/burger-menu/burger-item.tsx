import React from "react";
import Link from "next/link";
import { usePathname } from "next/navigation";

type NavItemProps = {
    href: string;
    text: string;
    onClick: VoidFunction;
}

const BurgerItem: React.FC<NavItemProps> = React.memo(({ href, text, onClick }) => {
    const pathname = usePathname();
    const isActive = pathname === href;

    return (
        <li className="flex-1 px-10 h-20 flex items-center" onClick={onClick}>
            <Link
                href={href}
                className={`font-fira flex justify-center hover:text-[#607B96] items-center lowercase font-medium group text-[32px] text-[#607B967F] ${isActive && "!text-[#FFF]"}`}
            >
                <p className={`text-[#FFFFFF7F] font-medium text-[32px] group-hover:text-[#C778DD] ${isActive && "!text-[#C778DD]"}`}>#</p>
                _{text}
            </Link>
        </li>
    );
});

export default BurgerItem;
