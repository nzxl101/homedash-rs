import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { formatNumber } from "@/lib/utils";
import { useQuery } from "@tanstack/react-query";
import { PlayCircle, Film, Download, Search } from "lucide-react";
import type { ProwlarrV1IndexerStats, RadarrV3Movies, RadarrV3WantedMissing, SonarrV3Series, SonarrV3WantedMissing } from "tuono/types";

interface StarrData {
    sonarr: {
        series: SonarrV3Series[];
        wantedMissing: SonarrV3WantedMissing;
    };
    radarr: {
        movies: RadarrV3Movies[];
        wantedMissing: RadarrV3WantedMissing;
    };
    prowlarr: ProwlarrV1IndexerStats;
}

function DataCard({ icon: Icon, label, value }: { icon: React.ElementType; label: string; value: string | number }) {
    return (
        <div className="bg-zinc-800 p-2 2xl:p-3 rounded-md flex items-center justify-between">
            <div className="flex items-center">
                <Icon className="w-4 h-4 2xl:w-5 2xl:h-5 text-zinc-400 flex-shrink-0" />
                <span className="hidden 2xl:inline text-zinc-400 text-sm ml-2">{label}</span>
            </div>
            <span className="text-white text-xs 2xl:text-sm font-medium ml-2">{value}</span>
        </div>
    );
}

export function Starr() {
    const { data, isLoading, error } = useQuery<StarrData>({
        queryKey: ["starr-data"],
        queryFn: async (): Promise<StarrData> => {
            const [series, sonarrWanted, movies, radarrWanted, prowlarrStats] = await Promise.all([fetch("/api/sonarr/series").then((res) => res.json()), fetch("/api/sonarr/wanted_missing").then((res) => res.json()), fetch("/api/radarr/movies").then((res) => res.json()), fetch("/api/radarr/wanted_missing").then((res) => res.json()), fetch("/api/prowlarr/stats").then((res) => res.json())]);

            return {
                sonarr: {
                    series: series || [],
                    wantedMissing: {
                        totalRecords: sonarrWanted?.totalRecords || 0,
                    },
                },
                radarr: {
                    movies: movies || [],
                    wantedMissing: {
                        totalRecords: radarrWanted?.totalRecords || 0,
                    },
                },
                prowlarr: prowlarrStats,
            };
        },
        refetchInterval: 3 * 60 * 1000,
    });

    if (!data) return null;

    return (
        <Card className="bg-zinc-900 border-zinc-800 h-full sm:col-span-1">
            <CardHeader>
                <CardTitle className="text-base 2xl:text-lg font-medium text-white">⭐ Apps</CardTitle>
            </CardHeader>
            <CardContent>
                <div className="grid grid-cols-2 2xl:grid-cols-3 gap-4">
                    <div className="space-y-2">
                        <h3 className="text-zinc-400 text-xs 2xl:text-sm font-medium mb-2">Sonarr</h3>
                        <DataCard icon={PlayCircle} label="Wanted" value={isLoading || error ? "..." : (data?.sonarr.wantedMissing.totalRecords ?? 0)} />
                        <DataCard icon={Film} label="Series" value={isLoading || error ? "..." : (data?.sonarr.series.length ?? 0)} />
                    </div>
                    <div className="space-y-2">
                        <h3 className="text-zinc-400 text-xs 2xl:text-sm font-medium mb-2">Radarr</h3>
                        <DataCard icon={Download} label="Missing" value={isLoading || error ? "..." : (data?.radarr.wantedMissing.totalRecords ?? 0)} />
                        <DataCard icon={Film} label="Movies" value={isLoading || error ? "..." : (data?.radarr.movies.length ?? 0)} />
                    </div>
                    <div className="space-y-2 col-span-2 2xl:col-span-1">
                        <h3 className="text-zinc-400 text-xs 2xl:text-sm font-medium mb-2">Prowlarr</h3>
                        <DataCard icon={Download} label="Grabs" value={isLoading || error ? "..." : (data?.prowlarr.hosts[0].numberOfGrabs ?? 0)} />
                        <DataCard icon={Search} label="Queries" value={isLoading || error ? "..." : formatNumber(data?.prowlarr.hosts[0].numberOfQueries ?? 0)} />
                    </div>
                </div>
            </CardContent>
        </Card>
    );
}
