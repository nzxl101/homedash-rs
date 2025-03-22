import type { JSX } from "react";
import Dashboard from "@/components/dashboard";
import { TooltipProvider } from "@/components/ui/tooltip";

export default function IndexPage(): JSX.Element {
    return (
        <TooltipProvider delayDuration={0}>
            <Dashboard />
        </TooltipProvider>
    );
}
