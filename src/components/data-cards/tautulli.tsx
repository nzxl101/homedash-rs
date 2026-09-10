import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Info } from "lucide-react";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { useQuery } from "@tanstack/react-query";
import { trimText } from "@/lib/utils";
import type { TautulliV2Session, TautulliV2Sessions } from "tuono/types";
import { Key } from "react";
import { FAKE_SERIES_TITLES, FAKE_EPISODE_TITLES, FAKE_USERS } from "@/lib/anonymize";

export function Tautulli({ isAnonymized }: { isAnonymized: boolean }) {
    const { data, isLoading, error } = useQuery<TautulliV2Sessions>({
        queryKey: ["tautulli-data"],
        queryFn: () => fetch("/api/tautulli/sessions").then((res) => res.json()),
        refetchInterval: 3 * 60e3,
    });

    const realSessions = data?.response.data?.sessions ?? [];
    const sessions = isAnonymized
        ? realSessions.map((session, index) => ({
              ...session,
              grandparent_title: FAKE_SERIES_TITLES[index % FAKE_SERIES_TITLES.length],
              title: FAKE_EPISODE_TITLES[index % FAKE_EPISODE_TITLES.length],
              user: FAKE_USERS[index % FAKE_USERS.length],
          }))
        : realSessions;

    if (!isAnonymized && !data) return null;

    return (
        <Card className="backdrop-filter backdrop-blur-lg bg-zinc-900/60 border border-white/10 shadow-lg h-full sm:col-span-1">
            <CardHeader>
                <CardTitle className="text-lg font-medium text-white">Tautulli</CardTitle>
            </CardHeader>
            <CardContent>
                <h4 className="text-sm font-medium text-zinc-400 mb-2">{!isAnonymized && (isLoading || error) ? "..." : sessions.length > 0 ? "Current Active Streams:" : "No Active Streams"}</h4>
                <ul className="space-y-2">
                    {sessions.map((session: TautulliV2Session, index: Key | null | undefined) => (
                        <li key={index} className="flex items-center justify-between text-sm">
                            <span className="text-white">
                                {trimText(session.grandparent_title ?? "", 20)} - {trimText(session.title ?? "", 20)} (S{session.parent_media_index}E{session.media_index})
                            </span>
                            <div className="flex items-center">
                                <span className="text-zinc-400 mr-2">
                                    {session.user} ({session.stream_video_full_resolution})
                                </span>
                                <Tooltip>
                                    <TooltipTrigger>
                                        <Info className="w-4 h-4 text-zinc-400" />
                                    </TooltipTrigger>
                                    <TooltipContent className="bg-white text-black mb-2">
                                        <p>{session.transcode_decision === "direct play" ? "Direct Play" : `Transcoding: ${session.video_full_resolution} -> ${session.stream_video_full_resolution} (${session.stream_video_codec})`}</p>
                                    </TooltipContent>
                                </Tooltip>
                            </div>
                        </li>
                    ))}
                </ul>
            </CardContent>
        </Card>
    );
}
