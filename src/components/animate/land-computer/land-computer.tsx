import Image from "next/image";


const LandComputer = () => {
    return (
        <div className="absolute w-[560px] h-[643px] bottom-0">
            <Image className="absolute bottom-0 bounce-platform" src="/big-land.png" alt="big land" width={560} height={523} />
            <Image className="absolute top-[40px]" src="/computer.png" alt="computer" width={540} height={512} />
            <Image className="absolute left-[190px] top-[190px] bounce" src="/block.png" alt="computer" width={55} height={30} />
            <Image className="absolute top-[60px] right-[80px] bounce" src="/block.png" alt="computer" width={75} height={40} />
            <Image className="absolute left-[208px] bounce" src="/block.png" alt="computer" width={97} height={82} />
        </div>
    );
};

export default LandComputer;
