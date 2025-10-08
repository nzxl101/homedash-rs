import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { DragDropContext, Droppable, Draggable, DropResult } from "@hello-pangea/dnd";
import { useSwipeable } from "react-swipeable";
import { Lock, Unlock, Plus } from "lucide-react";
import { memo, useCallback, useEffect, useMemo, useState } from "react";
import Spinner from "@/components/loading-spinner";
import { ExpandableDataSection } from "@/components/data-section";
import { bytesToSize, getGradient, getIconURL, getTimeOfDay, truncateString } from "@/lib/utils";
import { WeatherWidget } from "@/components/weather-widget";
import { useQuery } from "@tanstack/react-query";
import { UnduckSearchBar } from "@/components/unduck";

interface App {
    id: number;
    name: string;
    status: number;
    last_check: number;
    url: string;
    app_order: number;
    is_favorite: boolean;
}

const AppIcon = memo(({ app, isDragging, isLocked }: { app: App; isDragging: boolean; isLocked: boolean }) => (
    <div className={`flex flex-col items-center ${isDragging ? "opacity-50" : ""} select-none`} onClick={isLocked ? () => window.open(`${!app.name.match(/Plex/gi) ? app.url : `${app.url}/web`}`, "_blank") : () => false} style={{ cursor: isLocked ? "pointer" : "grab" }}>
        <div className="w-16 h-16 rounded-md bg-zinc-800 flex items-center justify-center text-2xl font-medium mb-2 border border-zinc-700 relative transition-all duration-300 ease-in-out transform hover:scale-105 hover:shadow-lg hover:border-zinc-600">
            <img className={"drop-shadow"} src={getIconURL(app.name)} width={42} height={42} alt={app.name} />
            <div className={`animate-pulse absolute bottom-1 right-1 w-2 h-2 rounded-full ${[200, 401].includes(app.status) ? "bg-emerald-400" : "bg-red-400"} ring-2 ring-zinc-800`}></div>
        </div>
        <span className="text-sm text-center font-medium text-zinc-200 hover:text-white transition-colors duration-300 w-full truncate">{app.name.length > 10 ? truncateString(app.name) : app.name}</span>
    </div>
));
AppIcon.displayName = "AppIcon";

export default function Dashboard({ username, weather }: { username: string | undefined; weather: { lat: number; long: number } | undefined }) {
    const { data, isLoading, error } = useQuery({
        queryKey: ["metrics-data"],
        queryFn: async () => {
            const [metrics, apps] = await Promise.all([fetch("/api/metrics").then((res) => res.json()), fetch("/api/ping").then((res) => res.json())]);
            return {
                metrics,
                apps,
            };
        },
        refetchInterval: 3 * 60 * 1000,
    });

    const cpuUsage = isLoading || error ? 0 : data?.metrics.cpu_usage.toFixed(2);
    const memUsage = isLoading || error ? 0 : data?.metrics.mem_used.toFixed(2);
    const memTotal = isLoading || error ? 0 : data?.metrics.mem_total.toFixed(2);
    const storageUsage = isLoading || error ? 0 : bytesToSize(data?.metrics.storage_used);
    const storageTotal = isLoading || error ? 0 : bytesToSize(data?.metrics.storage_total);
    const usedPercentage = (data?.metrics.storage_used * 100) / data?.metrics.storage_total;

    const [search, setSearch] = useState("");
    const [currentPage, setCurrentPage] = useState(1);
    const [favorites, setFavorites] = useState<App[]>([]);
    const [allApps, setAllApps] = useState<App[]>([]);
    const [isDragging, setIsDragging] = useState(false);
    const [isLocked, setIsLocked] = useState(true);
    const [isInitialized, setIsInitialized] = useState(false);
    const [itemsPerPage, setItemsPerPage] = useState(10);
    const [columns, setColumns] = useState(5);

    // First, let's modify the filteredApps useMemo to also check acronyms
    const filteredApps = useMemo(() => {
        return allApps.filter((app) => {
            // Get the acronym that would be displayed
            const acronym = app.name.length > 10 ? truncateString(app.name) : app.name;

            // Check if either the full name or acronym matches the search
            const matchesSearch = app.name.toLowerCase().includes(search.toLowerCase()) || acronym.toLowerCase().includes(search.toLowerCase());

            // Make sure it's not already in favorites
            return matchesSearch && !favorites.some((fav) => fav.id === app.id);
        });
    }, [allApps, favorites, search]);
    const totalPages = Math.ceil(filteredApps.length / itemsPerPage);

    const getListStyle = (isDraggingOver: boolean) => ({
        background: isDraggingOver ? "rgba(255, 255, 255, 0.1)" : "transparent",
    });

    const updateLayout = useCallback(() => {
        if (typeof window !== "undefined") {
            if (window.innerWidth <= 640) {
                setColumns(2);
                setItemsPerPage(6);
            } else if (window.innerWidth <= 1024) {
                setColumns(3);
                setItemsPerPage(9);
            } else {
                setColumns(5);
                setItemsPerPage(10);
            }
        }
    }, []);

    useEffect(() => {
        updateLayout();
        window.addEventListener("resize", updateLayout);
        return () => window.removeEventListener("resize", updateLayout);
    }, [updateLayout]);

    // Update the getRows function to handle the grid layout better
    const getRows = () => {
        const rowCount = Math.ceil(filteredApps.length / columns);
        return Array.from({ length: Math.min(rowCount, Math.ceil(itemsPerPage / columns)) }, (_, i) => i);
    };

    const getItemsForRow = (rowIndex: number) => {
        const startIndex = (currentPage - 1) * itemsPerPage + rowIndex * columns;
        return filteredApps.slice(startIndex, startIndex + columns);
    };

    const onDragEnd = useCallback(
        (result: DropResult) => {
            if (isLocked || !result.destination) return;
            setIsDragging(false);

            const { source, destination } = result;

            const sourceRowIndex = parseInt(source.droppableId.split("-")[1]);
            const destRowIndex = parseInt(destination.droppableId.split("-")[1]);

            const newAllApps = [...allApps];
            const newFavorites = [...favorites];

            const startIndex = (currentPage - 1) * itemsPerPage + sourceRowIndex * columns + source.index;
            const endIndex = (currentPage - 1) * itemsPerPage + destRowIndex * columns + destination.index;

            if (source.droppableId === destination.droppableId) {
                // Reordering within the same row
                const [reorderedItem] = newAllApps.splice(startIndex, 1);
                newAllApps.splice(endIndex, 0, reorderedItem);
            } else if (source.droppableId.startsWith("row") && destination.droppableId.startsWith("row")) {
                // Moving between rows
                const [movedItem] = newAllApps.splice(startIndex, 1);
                newAllApps.splice(endIndex, 0, movedItem);
            } else if (destination.droppableId === "favorites" && newFavorites.length < 5) {
                // Moving to favorites
                const [movedItem] = newAllApps.splice(startIndex, 1);
                newFavorites.splice(destination.index, 0, movedItem);
            } else if (source.droppableId === "favorites" && destination.droppableId.startsWith("row")) {
                // Moving from favorites to a row
                const [movedItem] = newFavorites.splice(source.index, 1);
                newAllApps.splice(endIndex, 0, movedItem);
            }

            setAllApps(newAllApps);
            setFavorites(newFavorites);
        },
        [allApps, favorites, isLocked, currentPage, itemsPerPage, columns]
    );

    const onDragStart = useCallback(() => {
        if (isLocked) return;
        setIsDragging(true);
    }, [isLocked]);

    const handlers = useSwipeable({
        onSwipedLeft: (eventData) => {
            if (!isDragging && eventData.velocity > 0.5) {
                setCurrentPage((prev) => Math.min(prev + 1, totalPages));
            }
        },
        onSwipedRight: (eventData) => {
            if (!isDragging && eventData.velocity > 0.5) {
                setCurrentPage((prev) => Math.max(prev - 1, 1));
            }
        },
        trackMouse: true,
        preventScrollOnSwipe: true,
        delta: 100,
    });

    useEffect(() => {
        if (!isLoading && !error && data?.apps) {
            // Filter favorites and non-favorites based on is_favorite flag
            const favoriteApps = data.apps.filter((app: App) => app.is_favorite);
            const nonFavoriteApps = data.apps.filter((app: App) => !app.is_favorite);

            // Sort non-favorite apps by app_order
            const sortedApps = [...nonFavoriteApps].sort((a, b) => a.app_order - b.app_order);

            setFavorites(favoriteApps);
            setAllApps(sortedApps);
            setIsInitialized(true);
        }
    }, [data, isLoading, error]);

    useEffect(() => {
        const saveData = async () => {
            if (!isInitialized || allApps.length === 0 || favorites === undefined) {
                return;
            }

            // Only save when the user locks the dashboard (indicating they're done with changes)
            if (isLocked) {
                try {
                    // Create updated app data with new order and favorite status
                    const updatedApps = [
                        ...favorites.map((app, index) => ({
                            id: app.id,
                            is_favorite: true,
                            app_order: index,
                        })),
                        ...allApps.map((app, index) => ({
                            id: app.id,
                            is_favorite: false,
                            app_order: index,
                        })),
                    ];

                    await fetch("/api/save", {
                        method: "POST",
                        headers: {
                            "Content-Type": "application/json",
                        },
                        body: JSON.stringify({ apps: updatedApps }),
                    });
                    console.log("Saved app order and favorites");
                } catch (error) {
                    console.error("Error saving app order and favorites:", error);
                }
            }
        };

        saveData();
    }, [isLocked]);

    if (isInitialized) {
        return (
            <DragDropContext onDragEnd={onDragEnd} onDragStart={onDragStart}>
                <div className="min-h-screen bg-black text-white p-6 font-sans select-none">
                    <header className="mb-12 flex justify-between items-start">
                        <h1 className="text-4xl font-bold tracking-tight">
                            Good {getTimeOfDay()}, {username ?? "user"}.
                        </h1>
                        <UnduckSearchBar />
                        <div className="flex items-center space-x-4">
                            <WeatherWidget lat={weather?.lat ?? 0} long={weather?.long ?? 0} />
                            <button onClick={() => setIsLocked(!isLocked)} className="text-zinc-400 hover:text-white transition-colors duration-200">
                                {isLocked ? <Lock className="h-6 w-6" /> : <Unlock className="h-6 w-6" />}
                            </button>
                        </div>
                    </header>
                    <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-12">
                        <Card className="bg-zinc-900 border-zinc-800 backdrop-filter backdrop-blur-sm">
                            <CardHeader className="pb-2">
                                <CardTitle className="text-lg font-medium text-white">CPU Usage</CardTitle>
                                <CardDescription className="text-zinc-300">Current: {`${cpuUsage}`}%</CardDescription>
                            </CardHeader>
                            <CardContent>
                                <div className="h-1 bg-zinc-700 rounded-full overflow-hidden">
                                    <div
                                        className="h-full rounded-full transition-all"
                                        style={{
                                            width: `${cpuUsage}%`,
                                            background: getGradient(cpuUsage),
                                        }}
                                    />
                                </div>
                            </CardContent>
                        </Card>
                        <Card className="bg-zinc-900 border-zinc-800 backdrop-filter backdrop-blur-sm">
                            <CardHeader className="pb-2">
                                <CardTitle className="text-lg font-medium text-white">Memory Usage</CardTitle>
                                <CardDescription className="text-zinc-300">
                                    {memUsage} GB / {memTotal} GB
                                </CardDescription>
                            </CardHeader>
                            <CardContent>
                                <div className="h-1 bg-zinc-700 rounded-full overflow-hidden">
                                    <div
                                        className="h-full rounded-full transition-all"
                                        style={{
                                            width: `${(memUsage * 100) / memTotal}%`,
                                            background: getGradient((memUsage * 100) / memTotal),
                                        }}
                                    />
                                </div>
                            </CardContent>
                        </Card>
                        <Card className="bg-zinc-900 border-zinc-800 backdrop-filter backdrop-blur-sm">
                            <CardHeader className="pb-2">
                                <CardTitle className="text-lg font-medium text-white">Storage Usage</CardTitle>
                                <CardDescription className="text-zinc-300">
                                    {storageUsage} / {storageTotal}
                                </CardDescription>
                            </CardHeader>
                            <CardContent>
                                <div className="h-1 bg-zinc-700 rounded-full overflow-hidden">
                                    <div
                                        className="h-full rounded-full transition-all"
                                        style={{
                                            width: `${usedPercentage}%`,
                                            background: getGradient(usedPercentage),
                                        }}
                                    />
                                </div>
                            </CardContent>
                        </Card>
                    </div>
                    <div {...handlers} className="overflow-hidden">
                        <div className="overflow-hidden max-h-[calc(100vh-500px)] min-h-[350px] lg:min-h-[234px] xl:min-h-[234px] pt-[2px]">
                            {getRows().map((rowIndex) => (
                                <Droppable key={`row-${rowIndex}`} droppableId={`row-${rowIndex}`} direction="horizontal" isDropDisabled={isLocked}>
                                    {(provided, snapshot) => (
                                        <div
                                            {...provided.droppableProps}
                                            ref={provided.innerRef}
                                            style={{
                                                ...getListStyle(snapshot.isDraggingOver),
                                                display: "grid",
                                                gridTemplateColumns: `repeat(${columns}, minmax(0, 1fr))`,
                                            }}
                                            className="gap-6 mb-6">
                                            {getItemsForRow(rowIndex).map((app, index) => (
                                                <Draggable key={app.id} draggableId={app.id.toString()} index={index} isDragDisabled={isLocked}>
                                                    {(provided, snapshot) => (
                                                        <div ref={provided.innerRef} {...provided.draggableProps} {...provided.dragHandleProps} className="flex items-center justify-center">
                                                            <AppIcon app={app} isDragging={snapshot.isDragging} isLocked={isLocked} />
                                                        </div>
                                                    )}
                                                </Draggable>
                                            ))}
                                            {provided.placeholder}
                                        </div>
                                    )}
                                </Droppable>
                            ))}
                        </div>
                    </div>
                    <div className="flex justify-center items-center mt-6 mb-12">
                        {Array.from({ length: totalPages }, (_, i) => (
                            <Button key={i} onClick={() => setCurrentPage(i + 1)} className={`mx-1 w-2 h-2 rounded-full p-0 ${currentPage === i + 1 ? "bg-white" : "bg-zinc-600"}`} />
                        ))}
                    </div>
                    <div className="mt-8 mb-8">
                        <Input placeholder="Search apps.." value={search} onChange={(e) => setSearch(e.target.value)} className="bg-zinc-900 text-[#aaa] border-zinc-700 rounded-md px-4 py-2 text-sm font-medium focus:outline-none focus:ring-1 focus:ring-zinc-600 focus:border-zinc-600 transition-all duration-200" />
                    </div>
                    <Card className="bg-zinc-900 border-zinc-800 backdrop-filter backdrop-blur-sm">
                        <CardHeader className="pb-2">
                            <CardTitle className="text-lg font-medium text-white">Favorites</CardTitle>
                        </CardHeader>
                        <CardContent className="scale-75 md:scale-100">
                            <Droppable droppableId="favorites" direction="horizontal" isDropDisabled={isLocked}>
                                {(provided) => (
                                    <div {...provided.droppableProps} ref={provided.innerRef} className="w-full max-w-[440px] mx-auto flex items-center justify-center">
                                        <div className="flex items-center justify-center gap-6">
                                            {favorites.map((app, index) => (
                                                <Draggable key={app.id} draggableId={app.id.toString()} index={index} isDragDisabled={isLocked}>
                                                    {(provided, snapshot) => (
                                                        <div ref={provided.innerRef} {...provided.draggableProps} {...provided.dragHandleProps}>
                                                            <AppIcon app={app} isDragging={snapshot.isDragging} isLocked={isLocked} />
                                                        </div>
                                                    )}
                                                </Draggable>
                                            ))}
                                            {favorites.length < 5 && !isLocked && (
                                                <div className="flex items-center justify-center w-16 h-24 text-zinc-500">
                                                    <Plus className="w-8 h-8" />
                                                </div>
                                            )}
                                            {favorites.length === 0 && isLocked && <div className="flex items-center justify-center w-auto h-12 text-zinc-500">No favorites added</div>}
                                            {provided.placeholder}
                                        </div>
                                    </div>
                                )}
                            </Droppable>
                        </CardContent>
                    </Card>
                    <ExpandableDataSection />
                </div>
            </DragDropContext>
        );
    } else {
        return <Spinner />;
    }
}
