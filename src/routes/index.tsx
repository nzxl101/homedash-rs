import type { JSX } from "react";
import Dashboard from "@/components/dashboard";
import { TooltipProvider } from "@/components/ui/tooltip";
import type { TuonoRouteProps } from "tuono";
import { IndexData } from "tuono/types";

export default function IndexPage({ data }: TuonoRouteProps<IndexData>): JSX.Element {
    return (
        <TooltipProvider delayDuration={0}>
            <Dashboard username={data?.username} weather={data?.weather} background={data?.background} />
        </TooltipProvider>
    );
}
