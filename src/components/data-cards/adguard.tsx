import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { formatNumber } from "@/lib/utils";
import { useQuery } from "@tanstack/react-query";

export function AdGuard() {
    const { data, isLoading, error } = useQuery({
        queryKey: ["adguard-stats"],
        queryFn: () => fetch("/api/adguard/stats").then((res) => res.json()),
        refetchInterval: 3 * 60e3,
    });

    return (
        <Card className="bg-zinc-900 border-zinc-800 h-full min-h-[200px]">
            <CardHeader>
                <CardTitle className="text-lg font-medium text-white">AdGuard Home</CardTitle>
            </CardHeader>
            <CardContent>
                <div className="grid grid-cols-2 gap-2 h-full">
                    <div className="bg-zinc-800 p-2 rounded-md flex flex-col justify-between">
                        <span className="text-zinc-400 text-xs">Queries</span>
                        <div className="text-white text-sm">{isLoading || error ? "..." : formatNumber(data?.num_dns_queries ?? 0)}</div>
                    </div>
                    <div className="bg-zinc-800 p-2 rounded-md flex flex-col justify-between">
                        <span className="text-zinc-400 text-xs">Blocked</span>
                        <div className="text-white text-sm">{isLoading || error ? "..." : formatNumber(data?.num_blocked_filtering ?? 0)}</div>
                    </div>
                    <div className="bg-zinc-800 p-2 rounded-md flex flex-col justify-between">
                        <span className="text-zinc-400 text-xs">Filtered</span>
                        <div className="text-white text-sm">{isLoading || error ? "..." : data?.num_replaced_safebrowsing + data?.num_replaced_safesearch + data?.num_replaced_parental}</div>
                    </div>
                    <div className="bg-zinc-800 p-2 rounded-md flex flex-col justify-between">
                        <span className="text-zinc-400 text-xs">Latency</span>
                        <div className="text-white text-sm">{isLoading || error ? "..." : (data?.avg_processing_time * 1000).toFixed(2)}ms</div>
                    </div>
                </div>
            </CardContent>
        </Card>
    );
}
