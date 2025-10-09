import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { bytesToSize } from "@/lib/utils";
import { useQuery } from "@tanstack/react-query";
import { DockwatchStatsResponse } from "tuono/types";

export function Docker() {
    const { data, isLoading, error } = useQuery<DockwatchStatsResponse>({
        queryKey: ["dockwatch-overview"],
        queryFn: () => fetch("/api/dockwatch/overview").then((res) => res.json()),
        refetchInterval: 3 * 60 * 1000,
    });

    if (!data) return null;

    return (
        <Card className="backdrop-filter backdrop-blur-lg bg-zinc-900/60 border border-white/10 shadow-lg h-full min-h-[200px] sm:col-span-1">
            <CardHeader>
                <CardTitle className="text-lg font-medium text-white">Docker</CardTitle>
            </CardHeader>
            <CardContent>
                <div className="grid grid-cols-2 gap-2 h-full">
                    <div className="backdrop-filter backdrop-blur-lg bg-zinc-900/60 border border-white/10 shadow-lg p-2 rounded-md flex flex-col justify-between">
                        <span className="text-zinc-400 text-xs">Containers</span>
                        <div className="text-white text-sm">{isLoading || error ? "..." : `${data?.response.status.running}/${data?.response.status.total}`}</div>
                    </div>
                    <div className="backdrop-filter backdrop-blur-lg bg-zinc-900/60 border border-white/10 shadow-lg p-2 rounded-md flex flex-col justify-between">
                        <span className="text-zinc-400 text-xs">Health</span>
                        <div className="text-white text-sm">{isLoading || error ? "..." : `${data?.response.health.healthy}/${(data?.response.health.unhealthy ?? 0) + (data?.response.health.unknown ?? 0)}`}</div>
                    </div>
                    <div className="backdrop-filter backdrop-blur-lg bg-zinc-900/60 border border-white/10 shadow-lg p-2 rounded-md flex flex-col justify-between">
                        <span className="text-zinc-400 text-xs">Updates</span>
                        <div className="text-white text-sm">{isLoading || error ? "..." : `${data?.response.updates.uptodate}/${data?.response.updates.outdated}`}</div>
                    </div>
                    <div className="backdrop-filter backdrop-blur-lg bg-zinc-900/60 border border-white/10 shadow-lg p-2 rounded-md flex flex-col justify-between">
                        <span className="text-zinc-400 text-xs">Network</span>
                        <div className="text-white text-sm">{isLoading || error ? "..." : bytesToSize(data?.response.usage.netIO ?? 0)}</div>
                    </div>
                </div>
            </CardContent>
        </Card>
    );
}
