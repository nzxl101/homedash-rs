import type { ReactNode, JSX } from "react";
import { TuonoScripts } from "tuono";
import "../styles/globals.css";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

const queryClient = new QueryClient();

interface RootLayoutProps {
    children: ReactNode;
}

export default function RootLayout({ children }: RootLayoutProps): JSX.Element {
    return (
        <QueryClientProvider client={queryClient}>
            <html>
                <head>
                    <title>homedash-rs</title>
                    <link rel="icon" href="/favicon.ico" />
                    <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no" />
                    <link rel="manifest" href="/manifest.json" />
                    <meta name="apple-mobile-web-app-capable" content="yes" />
                    <meta name="apple-mobile-web-app-status-bar-style" content="black-translucent" />
                    <meta name="apple-mobile-web-app-title" content="homedash-rs" />
                    <link rel="apple-touch-icon" href="/icons/icon-192x192.png" />
                </head>
                <body>
                    <main>{children}</main>
                    <TuonoScripts />
                </body>
            </html>
        </QueryClientProvider>
    );
}
