import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { trimText } from "@/lib/utils";
import { useQuery, useQueries, UseQueryResult } from "@tanstack/react-query";
import { useMemo } from "react";
import { OverseerrV1Request, OverseerrV1Requests, OverseerrV1RequestsCount, TMDB3Movie, TVDBV4SeriesData } from "tuono/types";

type MediaDetails = TMDB3Movie | TVDBV4SeriesData;

interface RequestItemProps {
    request: OverseerrV1Request;
    mediaQuery: UseQueryResult<MediaDetails>;
}

const RequestItem = ({ request, mediaQuery }: RequestItemProps) => {
    const mediaData = mediaQuery.data;
    const type = request.media.mediaType;

    let title, posterUrl;

    if (type === "tv" && mediaData && typeof mediaData === "object" && "data" in mediaData) {
        title = mediaData.data.name || "Unknown";
        posterUrl = mediaData.data.image || "";
    } else {
        const tmdbData = mediaData as TMDB3Movie;
        title = tmdbData?.title || "Unknown";
        const posterPath = tmdbData?.poster_path || "";
        posterUrl = posterPath ? `https://image.tmdb.org/t/p/w200${posterPath}` : "";
    }

    return (
        <li className="text-sm">
            <Tooltip>
                <TooltipTrigger className="cursor-pointer text-zinc-400">
                    <span className="text-white">{trimText(title.trim(), 25)}</span> <span className="text-zinc-400">by {request.requestedBy.plexUsername.trim()}</span>
                </TooltipTrigger>
                <TooltipContent side="left" align="start" alignOffset={-100} className="!bg-transparent">
                    {posterUrl && (
                        <>
                            <link rel="preload" as="image" href={posterUrl} />
                            <img src={posterUrl} width={100} height={150} alt={title} loading="eager" className="rounded-sm" />
                        </>
                    )}
                </TooltipContent>
            </Tooltip>
        </li>
    );
};

export function Overseerr() {
    const { data, isLoading, error } = useQuery({
        queryKey: ["overseerr-data"] as const,
        queryFn: async () => {
            try {
                const [requests, requestsCount] = await Promise.all([
                    fetch("/api/overseerr/requests").then((res) => {
                        if (!res.ok) throw new Error("Failed to fetch requests");
                        return res.json() as Promise<OverseerrV1Requests>;
                    }),
                    fetch("/api/overseerr/requests_count").then((res) => {
                        if (!res.ok) throw new Error("Failed to fetch request count");
                        return res.json() as Promise<OverseerrV1RequestsCount>;
                    }),
                ]);

                const limitedResults = {
                    ...requests,
                    results: requests.results?.slice(0, 2) ?? [],
                };

                return { requests: limitedResults, requestsCount };
            } catch (err) {
                throw new Error("Failed to fetch Overseerr data");
            }
        },
        refetchInterval: 3 * 60 * 1000,
    });

    const mediaQueries = useQueries({
        queries: useMemo(() => {
            if (!data?.requests?.results) return [];

            return data.requests.results
                .map((request) => {
                    const id = request.media.tvdbId ?? request.media.tmdbId ?? -1;
                    const type = request.media.mediaType;

                    if (id === -1) return null;

                    return {
                        queryKey: [`media-${type}`, id] as const,
                        queryFn: async () => {
                            const endpoint = type === "movie" ? `/api/tmdb/${id}` : `/api/tvdb/${id}`;
                            const res = await fetch(endpoint);
                            if (!res.ok) throw new Error(`Failed to fetch ${type} data`);
                            return res.json() as Promise<MediaDetails>;
                        },
                        staleTime: 60 * 60 * 1000,
                        refetchOnWindowFocus: false,
                    };
                })
                .filter((q): q is NonNullable<typeof q> => q !== null);
        }, [data?.requests?.results]),
    });

    const requestList = useMemo(() => {
        if (isLoading || error || !data?.requests?.results) return "...";
        return data.requests.results.map((request, index) => <RequestItem key={`${request.media.tvdbId}-${request.media.tmdbId}-${index}`} request={request} mediaQuery={mediaQueries[index]} />);
    }, [data?.requests?.results, mediaQueries, isLoading, error]);

    return (
        <Card className="bg-zinc-900 border-zinc-800 h-full">
            <CardHeader>
                <CardTitle className="text-lg font-medium text-white">Overseerr</CardTitle>
            </CardHeader>
            <CardContent>
                <div className="grid grid-cols-2 sm:grid-cols-4 gap-2">
                    <div className="bg-zinc-800 p-2 rounded-md col-span-1 sm:col-span-2">
                        <span className="text-zinc-400 text-xs">Processing</span>
                        <div className="text-white text-sm">{isLoading || error ? "..." : data?.requestsCount.processing}</div>
                    </div>
                    <div className="bg-zinc-800 p-2 rounded-md col-span-1 sm:col-span-2">
                        <span className="text-zinc-400 text-xs">Available</span>
                        <div className="text-white text-sm">{isLoading || error ? "..." : data?.requestsCount.available}</div>
                    </div>
                    <div className="col-span-2 sm:col-span-4 mt-2">
                        <h4 className="text-sm font-medium text-zinc-400 mb-2">Recent Requests:</h4>
                        <ul className="space-y-2 text-left text-nowrap">{requestList}</ul>
                    </div>
                </div>
            </CardContent>
        </Card>
    );
}
