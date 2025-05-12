import { Clock, CloudDrizzle, CloudFog, CloudMoon, CloudRain, Cloud, Moon, Snowflake, Sun } from "lucide-react";
import { useState, useEffect, useMemo } from "react";
import { useQuery } from "@tanstack/react-query";

interface WeatherResponse {
    current: {
        temperature_2m: number;
        is_day: number;
        weather_code: number;
    };
}

type Weather = {
    [key: number]: {
        day: WeatherDescription;
        night: WeatherDescription;
    };
};
type WeatherDescription = {
    description: string;
    icon: React.ReactNode;
};
const weatherCodes: Weather = {
    0: {
        day: { description: "Clear", icon: <Sun className="w-6 h-6" /> },
        night: { description: "Clear", icon: <Moon className="w-6 h-6 opacity-90" /> },
    },
    1: {
        day: { description: "Mainly Clear", icon: <Sun className="w-6 h-6" /> },
        night: { description: "Mainly Clear", icon: <Moon className="w-6 h-6 opacity-90" /> },
    },
    2: {
        day: { description: "Partly Cloudy", icon: <Cloud className="w-6 h-6" /> },
        night: { description: "Partly Cloudy", icon: <CloudMoon className="w-6 h-6 opacity-90" /> },
    },
    3: {
        day: { description: "Overcast", icon: <Cloud className="w-6 h-6" /> },
        night: { description: "Overcast", icon: <CloudMoon className="w-6 h-6 opacity-90" /> },
    },
    45: {
        day: { description: "Fog", icon: <CloudFog className="w-6 h-6" /> },
        night: { description: "Fog", icon: <CloudFog className="w-6 h-6 opacity-75" /> },
    },
    48: {
        day: { description: "Depositing Rime Fog", icon: <CloudFog className="w-6 h-6" /> },
        night: { description: "Depositing Rime Fog", icon: <CloudFog className="w-6 h-6 opacity-75" /> },
    },
    51: {
        day: { description: "Light Drizzle", icon: <CloudDrizzle className="w-6 h-6" /> },
        night: { description: "Light Drizzle", icon: <CloudDrizzle className="w-6 h-6 opacity-75" /> },
    },
    53: {
        day: { description: "Moderate Drizzle", icon: <CloudDrizzle className="w-6 h-6" /> },
        night: { description: "Moderate Drizzle", icon: <CloudDrizzle className="w-6 h-6 opacity-75" /> },
    },
    55: {
        day: { description: "Dense Drizzle", icon: <CloudDrizzle className="w-6 h-6" /> },
        night: { description: "Dense Drizzle", icon: <CloudDrizzle className="w-6 h-6 opacity-75" /> },
    },
    56: {
        day: { description: "Light Freezing Drizzle", icon: <CloudDrizzle className="w-6 h-6" /> },
        night: { description: "Light Freezing Drizzle", icon: <CloudDrizzle className="w-6 h-6 opacity-75" /> },
    },
    57: {
        day: { description: "Dense Freezing Drizzle", icon: <CloudDrizzle className="w-6 h-6" /> },
        night: { description: "Dense Freezing Drizzle", icon: <CloudDrizzle className="w-6 h-6 opacity-75" /> },
    },
    61: {
        day: { description: "Slight Rain", icon: <CloudRain className="w-6 h-6" /> },
        night: { description: "Slight Rain", icon: <CloudRain className="w-6 h-6 opacity-75" /> },
    },
    63: {
        day: { description: "Moderate Rain", icon: <CloudRain className="w-6 h-6" /> },
        night: { description: "Moderate Rain", icon: <CloudRain className="w-6 h-6 opacity-75" /> },
    },
    65: {
        day: { description: "Heavy Rain", icon: <CloudRain className="w-6 h-6" /> },
        night: { description: "Heavy Rain", icon: <CloudRain className="w-6 h-6 opacity-75" /> },
    },
    66: {
        day: { description: "Light Freezing Rain", icon: <CloudRain className="w-6 h-6" /> },
        night: { description: "Light Freezing Rain", icon: <CloudRain className="w-6 h-6 opacity-75" /> },
    },
    67: {
        day: { description: "Heavy Freezing Rain", icon: <CloudRain className="w-6 h-6" /> },
        night: { description: "Heavy Freezing Rain", icon: <CloudRain className="w-6 h-6 opacity-75" /> },
    },
    71: {
        day: { description: "Slight Snow", icon: <Snowflake className="w-6 h-6" /> },
        night: { description: "Slight Snow", icon: <Snowflake className="w-6 h-6 opacity-75" /> },
    },
    73: {
        day: { description: "Moderate Snow", icon: <Snowflake className="w-6 h-6" /> },
        night: { description: "Moderate Snow", icon: <Snowflake className="w-6 h-6 opacity-75" /> },
    },
    75: {
        day: { description: "Heavy Snow", icon: <Snowflake className="w-6 h-6" /> },
        night: { description: "Heavy Snow", icon: <Snowflake className="w-6 h-6 opacity-75" /> },
    },
    77: {
        day: { description: "Snow Grains", icon: <Snowflake className="w-6 h-6" /> },
        night: { description: "Snow Grains", icon: <Snowflake className="w-6 h-6 opacity-75" /> },
    },
    80: {
        day: { description: "Slight Rain Showers", icon: <CloudRain className="w-6 h-6" /> },
        night: { description: "Slight Rain Showers", icon: <CloudRain className="w-6 h-6 opacity-75" /> },
    },
    81: {
        day: { description: "Moderate Rain Showers", icon: <CloudRain className="w-6 h-6" /> },
        night: { description: "Moderate Rain Showers", icon: <CloudRain className="w-6 h-6 opacity-75" /> },
    },
    82: {
        day: { description: "Violent Rain Showers", icon: <CloudRain className="w-6 h-6" /> },
        night: { description: "Violent Rain Showers", icon: <CloudRain className="w-6 h-6 opacity-75" /> },
    },
    85: {
        day: { description: "Slight Snow Showers", icon: <Snowflake className="w-6 h-6" /> },
        night: { description: "Slight Snow Showers", icon: <Snowflake className="w-6 h-6 opacity-75" /> },
    },
    86: {
        day: { description: "Heavy Snow Showers", icon: <Snowflake className="w-6 h-6" /> },
        night: { description: "Heavy Snow Showers", icon: <Snowflake className="w-6 h-6 opacity-75" /> },
    },
    95: {
        day: { description: "Thunderstorm", icon: <CloudRain className="w-6 h-6" /> },
        night: { description: "Thunderstorm", icon: <CloudRain className="w-6 h-6 opacity-75" /> },
    },
    96: {
        day: { description: "Thunderstorm with Slight Hail", icon: <CloudRain className="w-6 h-6" /> },
        night: { description: "Thunderstorm with Slight Hail", icon: <CloudRain className="w-6 h-6 opacity-75" /> },
    },
    99: {
        day: { description: "Thunderstorm with Heavy Hail", icon: <CloudRain className="w-6 h-6" /> },
        night: { description: "Thunderstorm with Heavy Hail", icon: <CloudRain className="w-6 h-6 opacity-75" /> },
    },
};

export function WeatherWidget() {
    const [currentTime, setCurrentTime] = useState(new Date());
    const lat = 50.7685;
    const long = 8.5808;

    const { data: weather } = useQuery<WeatherResponse>({
        queryKey: ["weather"],
        queryFn: async () => {
            const res = await fetch(`https://api.open-meteo.com/v1/forecast?latitude=${lat}&longitude=${long}&current=temperature_2m,is_day,weather_code&timezone=Europe%2FBerlin`);
            if (!res.ok) {
                throw new Error("Weather API error");
            }
            return res.json();
        },
        refetchInterval: 30 * 60 * 1000,
        retry: 3,
        retryDelay: 60 * 1000,
        staleTime: 25 * 60 * 1000,
    });

    useEffect(() => {
        const timer = setInterval(() => {
            setCurrentTime(new Date());
        }, 1000);
        return () => clearInterval(timer);
    }, []);

    const formatTime = (date: Date) => {
        const hours = date.getHours().toString().padStart(2, "0");
        const minutes = date.getMinutes().toString().padStart(2, "0");
        return `${hours}:${minutes}`;
    };

    useEffect(() => {
        const timer = setInterval(() => {
            setCurrentTime(new Date());
        }, 1000);
        return () => clearInterval(timer);
    }, []);

    const formattedTime = formatTime(currentTime);

    const { currentTemp, currentWeather } = useMemo(() => {
        const temp = Math.round(weather?.current.temperature_2m ?? 0);
        const isDay = weather?.current.is_day === 1;
        const weatherCode = weather?.current.weather_code ?? 0;

        const weatherInfo = weatherCodes[weatherCode] || weatherCodes[0];

        return {
            currentTemp: temp,
            currentWeather: weatherInfo[isDay ? "day" : "night"],
        };
    }, [weather]);

    useEffect(() => {
        const timer = setInterval(() => {
            const now = new Date();
            setCurrentTime(now);
        }, 1000);
        return () => clearInterval(timer);
    }, []);

    const formatDate = (date: Date) => {
        return date.toLocaleDateString("en-US", { weekday: "long", month: "long", day: "numeric" });
    };

    return (
        <div className="flex items-center space-x-4 text-white">
            <div className="flex sm:hidden flex-col space-y-2">
                <div className="flex items-center">
                    <div className="w-6 flex items-center justify-center">
                        <Clock className="w-6 h-6" />
                    </div>
                    <span className="ml-2">{formattedTime}</span>
                </div>
                <div className="flex items-center">
                    <div className="w-6 flex items-center justify-center">{currentWeather.icon}</div>
                    <span className="ml-2 font-bold">{currentTemp}°C</span>
                </div>
            </div>

            <div className="hidden sm:flex items-center space-x-6">
                <div className="flex items-center">
                    <div className="w-6 flex items-center justify-center">
                        <Clock className="w-6 h-6" />
                    </div>
                    <div className="ml-2">
                        <div className="font-bold">{formattedTime}</div>
                        <div className="text-sm text-gray-300">{formatDate(currentTime)}</div>
                    </div>
                </div>
                <div className="flex items-center">
                    <div className="w-6 flex items-center justify-center">{currentWeather.icon}</div>
                    <div className="ml-2">
                        <div className="font-bold">{currentTemp}°C</div>
                        <div className="text-sm text-gray-300">{currentWeather.description}</div>
                    </div>
                </div>
            </div>
        </div>
    );
}
