import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { useQuery } from "@tanstack/react-query";
import { bytesToSize } from "@/lib/utils";
import { ProxmoxV2Data } from "tuono/types";

export function Proxmox() {
    const { data, isLoading, error } = useQuery<ProxmoxV2Data[]>({
        queryKey: ["proxmox-node"],
        queryFn: () => fetch("/api/proxmox/node").then((res) => res.json()),
        refetchInterval: 3 * 60 * 1000,
    });

    const nodeData = data?.[0];
    const runningLxcs = nodeData?.lxc.data.filter((vm) => vm.status === "running").length ?? 0;
    const totalLxcs = nodeData?.lxc.data.length ?? 0;
    const runningVms = nodeData?.qemu.data.filter((vm) => vm.status === "running").length ?? 0;
    const totalVms = nodeData?.qemu.data.length ?? 0;
    const cpuUsage = (nodeData?.status.data.cpu ?? 0) * 100;
    const memoryUsed = nodeData?.status.data.memory.used ?? 0;
    const memoryTotal = nodeData?.status.data.memory.total ?? 1;
    const memoryPercentage = Math.round((memoryUsed / memoryTotal) * 100);

    return (
        <Card className="bg-zinc-900 border-zinc-800 h-full min-h-[200px]">
            <CardHeader>
                <CardTitle className="text-lg font-medium text-white">Proxmox</CardTitle>
            </CardHeader>
            <CardContent>
                <div className="grid grid-cols-2 gap-2 h-full">
                    <div className="bg-zinc-800 p-2 rounded-md flex flex-col justify-between">
                        <span className="text-zinc-400 text-xs">VMs</span>
                        <div className="text-white text-sm">{isLoading || error ? "..." : `${runningVms}/${totalVms}`}</div>
                    </div>
                    <div className="bg-zinc-800 p-2 rounded-md flex flex-col justify-between">
                        <span className="text-zinc-400 text-xs">LXCs</span>
                        <div className="text-white text-sm">{isLoading || error ? "..." : `${runningLxcs}/${totalLxcs}`}</div>
                    </div>
                    <div className="bg-zinc-800 p-2 rounded-md flex flex-col justify-between">
                        <span className="text-zinc-400 text-xs">CPU</span>
                        <div className="text-white text-sm">{isLoading || error ? "..." : `${cpuUsage.toFixed(1)}%`}</div>
                    </div>
                    <div className="bg-zinc-800 p-2 rounded-md flex flex-col justify-between">
                        <span className="text-zinc-400 text-xs">MEM</span>
                        <div className="text-white text-sm whitespace-nowrap">{isLoading || error ? "..." : window.innerWidth < 1000 ? `${memoryPercentage}%` : `${bytesToSize(memoryUsed)} / ${bytesToSize(memoryTotal)} (${memoryPercentage}%)`}</div>
                    </div>
                </div>
            </CardContent>
        </Card>
    );
}
