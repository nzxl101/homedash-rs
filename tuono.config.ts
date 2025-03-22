import tailwindcss from "@tailwindcss/vite";
import type { TuonoConfig } from "tuono/config";
import path, { dirname } from "node:path";
import { fileURLToPath } from "node:url";

function getDirname(importMetaUrl: string): string {
    return dirname(fileURLToPath(importMetaUrl));
}

const __dirname = getDirname(import.meta.url);

const config: TuonoConfig = {
    vite: {
        plugins: [tailwindcss()],
        alias: {
            "@": path.resolve(__dirname, "./src"),
        },
    },
    // server: {
    //     host: "0.0.0.0",
    //     port: 3000,
    // },
};

export default config;
