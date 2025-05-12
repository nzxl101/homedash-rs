import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs));
}

export const bytesToSize = (bytes: number): string => {
    const sizes = ["B", "Kb", "MB", "GB", "TB"];
    if (bytes === 0) {
        return "n/a";
    }

    const i = parseInt(`${Math.floor(Math.log(bytes) / Math.log(1024))}`, 10);
    if (i === 0) {
        return `${bytes}${sizes[i]}`;
    }

    return `${(bytes / 1024 ** i).toFixed(1)} ${sizes[i]}`;
};

export function formatNumber(num: number): string {
    const units = ["", "K", "M", "B", "T"];
    const order = Math.floor(Math.log10(Math.abs(num)) / 3);

    if (order === 0 || isNaN(num)) return num.toString();

    const divisor = Math.pow(10, order * 3);
    const shortened = (num / divisor).toFixed(1);

    // Remove trailing .0 if present
    const formatted = shortened.endsWith(".0") ? shortened.slice(0, -2) : shortened;

    return formatted + units[order];
}

export const getGradient = (percentage: number): string => {
    if (percentage <= 33) {
        return "linear-gradient(90deg, #22c55e, #4ade80)";
    } else if (percentage <= 66) {
        return "linear-gradient(90deg, #eab308, #f97316)";
    } else {
        return "linear-gradient(90deg, #f97316, #ef4444)";
    }
};

export const getTimeOfDay = (): string => {
    const date = new Date();
    const hour = date.getHours();

    if (hour < 4) return "night";
    if (hour < 12) return "morning";
    if (hour < 18) return "afternoon";
    if (hour < 22) return "evening";
    return "night";
};

export const truncateString = (text: string): string => {
    let result = text;

    switch (text) {
        case "Home Assistant":
            result = "HA";
            break;
        case "AdGuard Home":
            result = "AdGuard";
            break;
        case "Vaultwarden":
            result = "Vault";
            break;
        case "Pterodactyl":
            result = "Panel";
            break;
        case "JDownloader2":
            result = "JD";
            break;
        case "qBittorrent":
            result = "qBit";
            break;
    }

    return result;
};

export const getIconURL = (app: string): string => {
    let cdn_url = "https://cdn.jsdelivr.net/gh/selfhst/icons/png";

    switch (app) {
        case "Dockwatch":
            return "https://raw.githubusercontent.com/Notifiarr/images/refs/heads/main/icons/dockwatch.png";
        case "AdGuard":
            return `${cdn_url}/adguard-home.png`;
        case "JDownloader2":
            return `${cdn_url}/jdownloader.png`;
        case "Pterodactyl":
            return `${cdn_url}/pelican-panel.png`;
    }

    return `${cdn_url}/${app.toLowerCase().replace(" ", "-")}.png`;
};

export const trimText = (text: string, maxLength: number): string => {
    if (text.length <= maxLength) {
        return text;
    }

    let visibleLength = 0;
    let i = 0;

    // Count characters based on their visual width
    while (i < text.length && visibleLength < maxLength) {
        // Check if the character is a wide character (CJK, emoji, etc.)
        const char = text.charAt(i);
        const code = text.codePointAt(i) || 0;

        // East Asian Wide (Chinese, Japanese, Korean)
        const isWideChar =
            (code >= 0x1100 && code <= 0x11ff) || // Hangul Jamo
            (code >= 0x2e80 && code <= 0x9fff) || // CJK Unified Ideographs
            (code >= 0xac00 && code <= 0xd7af) || // Hangul Syllables
            (code >= 0xf900 && code <= 0xfaff) || // CJK Compatibility Ideographs
            (code >= 0xff00 && code <= 0xffef) || // Halfwidth and Fullwidth Forms
            (code >= 0x20000 && code <= 0x2fa1f); // CJK Unified Ideographs Extension

        // Add 2 for wide characters, 1 for others
        visibleLength += isWideChar ? 2 : 1;

        // If this character would exceed the limit, break
        if (visibleLength > maxLength) break;

        // Move to next character (accounting for surrogate pairs)
        i += code > 0xffff ? 2 : 1;
    }

    return i < text.length ? text.substring(0, i) + "..." : text;
};
