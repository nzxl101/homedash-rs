import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { useQuery } from "@tanstack/react-query";
import { GluetunV1VPNPublicIP } from "tuono/types";
import { Button } from "../ui/button";
import { RotateCcw } from "lucide-react";

export function Gluetun() {
    const { data, isLoading, error } = useQuery<GluetunV1VPNPublicIP>({
        queryKey: ["gluetun-public-ip"],
        queryFn: () => fetch("/api/gluetun/vpn_public_ip").then((res) => res.json()),
        refetchInterval: 3 * 60e3,
    });
    if (!data) return null;

    const reconnectVPN = async () => {
        const [outcome] = await Promise.all([fetch("/api/gluetun/vpn_reconnect").then((res) => res.json())]);
        return {
            outcome,
        };
    };

    return (
        <Card className="backdrop-filter backdrop-blur-lg bg-zinc-900/60 border border-white/10 shadow-lg h-full min-h-[200px] sm:col-span-1">
            <CardHeader>
                <CardTitle className="text-lg font-medium text-white">Gluetun VPN</CardTitle>
            </CardHeader>
            <CardContent>
                <div className="grid grid-cols-2 gap-2 h-full">
                    <div className="backdrop-filter backdrop-blur-lg bg-zinc-900/60 border border-white/10 shadow-lg p-2 rounded-md flex flex-col justify-between">
                        <span className="text-zinc-400 text-xs">IP</span>
                        <div className="text-white text-sm">{isLoading || error ? "..." : (data?.public_ip ?? "127.0.0.1")}</div>
                    </div>
                    <div className="backdrop-filter backdrop-blur-lg bg-zinc-900/60 border border-white/10 shadow-lg p-2 rounded-md flex flex-col justify-between">
                        <span className="text-zinc-400 text-xs">Location</span>
                        <div className="text-white text-sm">{isLoading || error ? "..." : `${data?.city}, ${data?.country}`}</div>
                    </div>
                    <div className="backdrop-filter backdrop-blur-lg bg-zinc-900/60 border border-white/10 shadow-lg p-2 rounded-md flex flex-col justify-between">
                        <span className="text-zinc-400 text-xs">Provider</span>
                        <div className="text-white text-sm">{isLoading || error ? "..." : (data?.organization ?? "Unknown")}</div>
                    </div>
                    <div className="backdrop-filter backdrop-blur-lg bg-zinc-900/60 border border-white/10 shadow-lg p-2 rounded-md flex flex-col justify-between">
                        <Button onClick={() => reconnectVPN()} className="backdrop-filter backdrop-blur-lg bg-zinc-900/60 border border-white/10 shadow-lg rounded-md flex items-center justify-center text-xs font-medium relative transition-all duration-300 ease-in-out transform hover:scale-101 text-zinc-100 p-0 h-full cursor-pointer">
                            <RotateCcw />
                        </Button>
                    </div>
                </div>
            </CardContent>
        </Card>
    );
}
