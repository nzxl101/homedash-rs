import React, { useState } from "react";
import { Input } from "./ui/input";
import { Button } from "./ui/button";
import { Search } from "lucide-react";

export function UnduckSearchBar() {
    const [query, setQuery] = useState("");

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        if (query.trim()) {
            window.open(`https://unduck.link?q=${encodeURIComponent(query)}`, "_blank");
        }
    };

    return (
        <div className="w-full max-w-sm items-center hidden lg:flex">
            <form onSubmit={handleSubmit} className="inline-flex gap-2 w-full translate-y-1">
                <Input type="text" value={query} onChange={(e) => setQuery(e.target.value)} placeholder="Search with Unduck.." className="bg-zinc-900 text-[#aaa] gray-500 border-zinc-700 rounded-md px-4 py-2 text-sm font-medium focus:outline-none focus:ring-1 focus:ring-zinc-600 focus:border-zinc-600 transition-all duration-200" />
                <Button type="submit" className="rounded-md bg-zinc-800 flex items-center justify-center text-2xl font-medium mb-2 border border-zinc-700 relative transition-all duration-300 ease-in-out transform hover:scale-105 hover:shadow-lg hover:border-zinc-900">
                    <Search />
                </Button>
            </form>
        </div>
    );
}
