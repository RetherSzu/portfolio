import Link from "next/link";
import LandComputer from "@/components/animate/land-computer/land-computer";

const NotFound = () => {
    return (
        <div className="h-full flex flex-col justify-center items-center gap-4">
            <h1 className="font-fira text-white text-xl">Not found – 404!</h1>
            <Link href="/" className="text-white font-fira">Go back to Home</Link>
            <LandComputer />
        </div>
    );
};

export default NotFound;
