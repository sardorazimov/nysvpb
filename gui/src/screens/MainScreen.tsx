import { useRef, useEffect, useState } from "react";
import Globe from "react-globe.gl";

import { ServerInfo } from "../types";
import { invoke } from "@tauri-apps/api/core";

interface Props {
  selectedServer: ServerInfo | null;
}



export default function MainScreen({ selectedServer }: Props) {
  const globeRef = useRef<any>();
  const [connected, setConnected] = useState(false);

  const [ping, setPing] = useState<number | null>(null);
  const [ip, setIp] = useState("0.0.0.0");

  useEffect(() => {
    if (globeRef.current) {
      globeRef.current.controls().autoRotate = true;
      globeRef.current.controls().autoRotateSpeed = 0.35;
    }
  }, []);

  // Fake ping simulation
  useEffect(() => {
    if (connected) {
      setIp("185.199.108.153");
      const interval = setInterval(() => {
        setPing(Math.floor(Math.random() * 40) + 20);
      }, 2000);
      return () => clearInterval(interval);
    } else {
      setPing(null);
      setIp("0.0.0.0");
    }
  }, [connected]);
  const toggle = async () => {
    if (!selectedServer) return;

    try {
      if (!connected) {
        await invoke("vpn_connect", {
          config: {
            server_id: selectedServer.id
          }
        });

        setConnected(true);
      } else {
        await invoke("vpn_disconnect");
        setConnected(false);
      }
    } catch (err) {
      console.error("VPN error:", err);
    }
  };

  return (
    <div className="relative h-full w-full overflow-hidden bg-gradient-to-br from-[#0f172a] via-[#0a0f1c] to-black text-white">

      {/* Stars */}
      <div className="absolute inset-0 stars pointer-events-none" />

      {/* Status */}
      <div className="absolute top-6 right-8 z-20 text-sm font-medium">
        <span className={connected ? "text-green-400" : "text-red-400"}>
          {connected ? "Connected" : "Disconnected"}
        </span>
      </div>

      {/* Globe */}
      <div className="relative z-10 flex items-center justify-center h-full">
        <Globe
          ref={globeRef}
          width={420}
          height={420}
          globeImageUrl="//unpkg.com/three-globe/example/img/earth-dark.jpg"
          backgroundColor="rgba(0,0,0,0)"
          atmosphereColor="#3b82f6"
          atmosphereAltitude={0.25}
          pointsData={selectedServer ? [selectedServer] : []}
          pointLat="lat"
          pointLng="lng"
          pointColor={() => "#22c55e"}
          pointAltitude={0.02}
          pointRadius={0.6}
        />
      </div>

      {/* Bottom Glass Panel */}
      <div className="absolute bottom-0 w-full p-6 bg-white/5 backdrop-blur-xl border-t border-white/10 flex flex-col items-center space-y-5">

        {/* Live Data */}
        <div className="flex gap-6 text-xs text-gray-400">
          <div>IP: <span className="text-white">{ip}</span></div>
          <div>
            Ping:{" "}
            <span className={connected ? "text-green-400" : "text-gray-500"}>
              {ping ? `${ping} ms` : "--"}
            </span>
          </div>
        </div>

        {/* Server */}
        <div className="text-xs text-gray-400 tracking-wide">
          {selectedServer
            ? `Server: ${selectedServer.name}`
            : "No server selected"}
        </div>

        {/* Power Button */}
        <button
          disabled={!selectedServer}
          onClick={toggle}
          className={`w-40 h-40 rounded-full flex items-center justify-center
            transition-all duration-500
            ${connected
              ? "bg-green-400 text-black animate-[slow-pulse_3s_infinite]"
              : "bg-blue-500 hover:bg-blue-600"
            }
            disabled:opacity-30 disabled:cursor-not-allowed
          `}
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            className="w-12 h-12"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            strokeWidth={2}
          >
            <path strokeLinecap="round" strokeLinejoin="round"
              d="M12 3v9m5.657-5.657a8 8 0 11-11.314 0" />
          </svg>
        </button>
      </div>
    </div>
  );
}