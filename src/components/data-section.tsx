import { useState } from "react";
import { ChevronDown, ChevronUp } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Starr } from "./data-cards/starr";
import { Overseerr } from "./data-cards/overseerr";
import { Docker } from "./data-cards/docker";
import { Tautulli } from "./data-cards/tautulli";
import { Proxmox } from "./data-cards/proxmox";
import { AdGuard } from "./data-cards/adguard";
import { QBittorrent } from "./data-cards/qbittorrent";

export function ExpandableDataSection() {
    const [isExpanded, setIsExpanded] = useState(false);

    return (
        <div className="mt-8">
            <Button onClick={() => setIsExpanded(!isExpanded)} variant="ghost" className="w-full flex items-center justify-center py-2 text-zinc-400 hover:text-white hover:bg-zinc-800 transition-colors duration-200">
                <span className="mr-2">Click here for more data</span>
                {isExpanded ? <ChevronUp className="w-4 h-4" /> : <ChevronDown className="w-4 h-4" />}
            </Button>
            {isExpanded && (
                <div className="grid grid-cols-1 sm:grid-cols-3 gap-4 mt-6">
                    <Docker />
                    <AdGuard />
                    <Proxmox />
                    <Starr />
                    <Tautulli />
                    <Overseerr />
                    <QBittorrent />
                </div>
            )}
        </div>
    );
}
