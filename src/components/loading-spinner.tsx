import { cn } from "@/lib/utils";

export default function Spinner() {
    return (
        <>
            <div className="fixed inset-0 flex items-center justify-center bg-black">
                <div className={cn("w-12 h-12 rounded-full animate-spin", "border-y-2 border-solid border-white", "border-l-2 border-l-transparent", "border-r-2 border-r-transparent")}>
                    <span className="sr-only">...</span>
                </div>
            </div>
        </>
    );
}
