import type { TuonoConfig } from "tuono/config";

const config: TuonoConfig = {
    vite: {
        plugins: [tailwindcss()],
    },
};

export default config;
