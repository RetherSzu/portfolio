"use client";
//
import React from "react";
import Link from "next/link";
// components
import NavItem from "@/components/nav-bar/nav-item";
import BurgerMenu from "@/components/nav-bar/burger-menu/burger-menu";
//
import { configNavBar } from "./config-nav-bar";

// -------------------------------------------------

export default function NavBar() {
    return (
        <nav className="flex w-full items-center justify-between border-[#1E2D3D] border-b-2 h-[80px]">
            <Link href="/">
                <p className="font-fira text-[#607B96] font-medium text-center text-2xl">&lt; 0xR3v /&gt;</p>
            </Link>
            <BurgerMenu />
            <ul className="hidden md:flex flex-row justify-around">
                {configNavBar.map((item, index) => (
                    <NavItem key={index} href={item.path} text={item.label} />
                ))}
            </ul>
        </nav>
    );
}
