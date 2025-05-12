import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { bytesToSize } from "@/lib/utils";
import { ArrowDown, ArrowUp, Check, Loader, Pause, X } from "lucide-react";
import { useQuery } from "@tanstack/react-query";
import { QBitV2Torrent } from "tuono/types";

export function QBittorrent() {
    const {
        data: torrents = [],
        isLoading,
        error,
    } = useQuery<QBitV2Torrent[]>({
        queryKey: ["qbittorrent-torrents"],
        queryFn: () => fetch("/api/qbittorrent/torrents").then((res) => res.json()),
        refetchInterval: 3 * 60 * 1000,
    });

    const activeTorrents = torrents.filter((x) => x.state !== "paused").length;
    const totalDownloadSpeed = torrents.reduce((sum, item) => sum + (item.dlspeed || 0), 0);
    const totalUploadSpeed = torrents.reduce((sum, item) => sum + (item.upspeed || 0), 0);

    if (isLoading || error) {
        return (
            <Card className="bg-zinc-900 border-zinc-800 h-full">
                <CardHeader>
                    <CardTitle className="text-lg font-medium text-white">qBittorrent</CardTitle>
                </CardHeader>
                <CardContent>
                    <div className="text-zinc-400">...</div>
                </CardContent>
            </Card>
        );
    }

    return (
        <Card className="bg-zinc-900 border-zinc-800 h-full">
            <CardHeader>
                <CardTitle className="text-lg font-medium text-white">qBittorrent</CardTitle>
            </CardHeader>
            <CardContent>
                <div className="overflow-x-auto">
                    <table className="w-full text-sm">
                        <thead className="sr-only sm:not-sr-only">
                            <tr className="text-zinc-400 border-b border-zinc-800">
                                <th className="text-left pb-2">Name</th>
                                <th className="text-right pb-2 hidden sm:table-cell">Size</th>
                                <th className="text-right pb-2 hidden sm:table-cell">Progress</th>
                                <th className="text-center pb-2">Status</th>
                                <th className="text-right pb-2 hidden sm:table-cell">Seeds/Peers</th>
                                <th className="text-right pb-2 hidden lg:table-cell">Speed</th>
                                <th className="text-right pb-2 hidden lg:table-cell">ETA</th>
                                <th className="text-right pb-2 hidden lg:table-cell">Ratio</th>
                            </tr>
                        </thead>
                        <tbody>
                            {torrents.map((torrent, index) => (
                                <tr key={index} className="border-b border-zinc-800 last:border-b-0">
                                    <td className="py-2 text-white">
                                        <div>{torrent.name}</div>
                                        <div className="text-zinc-400 text-xs sm:hidden">
                                            {bytesToSize(torrent.size)} | {(torrent.progress * 100).toFixed(1)}% | {torrent.num_seeds}/{torrent.num_leechs}
                                        </div>
                                    </td>
                                    <td className="py-2 text-right text-zinc-400 hidden sm:table-cell">{bytesToSize(torrent.size)}</td>
                                    <td className="py-2 text-right text-zinc-400 hidden sm:table-cell">{(torrent.progress * 100).toFixed(1)}%</td>
                                    <td className="py-2 text-center">
                                        {["uploading", "stalledUP", "forcedUP"].includes(torrent.state) && <ArrowUp className="w-4 h-4 text-green-500 inline" />}
                                        {["downloading", "metaDL", "stalledDL", "allocating", "forcedDL"].includes(torrent.state) && <ArrowDown className="w-4 h-4 text-blue-500 inline" />}
                                        {["pausedDL"].includes(torrent.state) && <Pause className="w-4 h-4 text-yellow-500 inline" />}
                                        {["pausedUP", "checkingUP"].includes(torrent.state) && <Check className="w-4 h-4 text-green-500 inline" />}
                                        {["queuedUP", "queuedDL", "checkingDL", "checkingResumeData", "moving"].includes(torrent.state) && <Loader className="w-4 h-4 text-gray-500 inline" />}
                                        {["error", "missingFiles", "unknown"].includes(torrent.state) && <X className="w-4 h-4 text-red-500 inline" />}
                                    </td>
                                    <td className="py-2 text-right text-zinc-400 hidden sm:table-cell">
                                        {torrent.num_seeds}/{torrent.num_leechs}
                                    </td>
                                    <td className="py-2 text-right text-zinc-400 hidden lg:table-cell">{bytesToSize(torrent.dlspeed || torrent.upspeed)}/s</td>
                                    <td className="py-2 text-right text-zinc-400 hidden lg:table-cell">{new Date(torrent.eta * 1000).toISOString().slice(11, 19)}</td>
                                    <td className="py-2 text-right text-zinc-400 hidden lg:table-cell">{torrent.ratio.toFixed(2)}</td>
                                </tr>
                            ))}
                        </tbody>
                    </table>
                </div>
                <div className="mt-4 flex flex-col sm:flex-row justify-between text-sm">
                    <span className="text-zinc-400">Active Torrents: {activeTorrents}</span>
                    <span className="text-zinc-400">DL Speed: {bytesToSize(totalDownloadSpeed)}/s</span>
                    <span className="text-zinc-400">UP Speed: {bytesToSize(totalUploadSpeed)}/s</span>
                </div>
            </CardContent>
        </Card>
    );
}
