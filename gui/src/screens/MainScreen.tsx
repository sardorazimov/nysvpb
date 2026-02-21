/**
 * Main Screen — shows connection status, connect/disconnect button,
 * server info, IP address, uptime timer, and transfer stats.
 */
import { useState, useEffect, useCallback } from "react";
import {
  TunnelStatus,
  TunnelStats,
  ServerInfo,
  AppSettings,
  formatBytes,
  formatDuration,
  isConnected,
} from "../types";
import { vpnConnect, vpnDisconnect, vpnStatus, vpnStats } from "../tauri-commands";

interface Props {
  selectedServer: ServerInfo | null;
  settings: AppSettings;
}

const DEFAULT_SERVER: ServerInfo = {
  id: "us-ny-1",
  country: "United States",
  country_flag: "🇺🇸",
  city: "New York",
  address: "us-ny-1.nysvpb.example:51820",
  public_key: "REPLACE_WITH_REAL_PUBLIC_KEY=",
  ping_ms: null,
};

export default function MainScreen({ selectedServer, settings: _settings }: Props) {
  const [status, setStatus] = useState<TunnelStatus>("Disconnected");
  const [stats, setStats] = useState<TunnelStats>({
    bytes_sent: 0,
    bytes_received: 0,
    last_handshake: null,
  });
  const [uptime, setUptime] = useState(0);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const server = selectedServer ?? DEFAULT_SERVER;

  // Poll status every 2 s
  useEffect(() => {
    let cancelled = false;

    const poll = async () => {
      try {
        const [s, st] = await Promise.all([vpnStatus(), vpnStats()]);
        if (!cancelled) {
          setStatus(s);
          setStats(st);
        }
      } catch {
        // daemon not running — show disconnected
      }
    };

    poll();
    const id = setInterval(poll, 2000);
    return () => {
      cancelled = true;
      clearInterval(id);
    };
  }, []);

  // Uptime counter
  useEffect(() => {
    if (!isConnected(status)) {
      setUptime(0);
      return;
    }
    const connected = status as { Connected: { since: string; server: string } };
    const startMs = new Date(connected.Connected.since).getTime();

    const id = setInterval(() => {
      const elapsed = Math.floor((Date.now() - startMs) / 1000);
      setUptime(elapsed);
    }, 1000);
    return () => clearInterval(id);
  }, [status]);

  const handleToggle = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      if (isConnected(status)) {
        await vpnDisconnect();
        setStatus("Disconnected");
      } else {
        setStatus("Connecting");
        await vpnConnect({
          server_addr: server.address,
          server_public_key: server.public_key,
          client_private_key: "YOUR_PRIVATE_KEY=",
          client_ip: "10.0.0.2",
          dns_servers: ["1.1.1.1", "1.0.0.1"],
          allowed_ips: ["0.0.0.0/0"],
        });
      }
    } catch (e) {
      setError(String(e));
      setStatus("Disconnected");
    } finally {
      setLoading(false);
    }
  }, [status, server]);

  const connected = isConnected(status);
  const connecting = status === "Connecting";

  const dotColor = connected
    ? "bg-vpn-green shadow-[0_0_12px_#22c55e]"
    : connecting
    ? "bg-vpn-yellow animate-pulse"
    : "bg-vpn-red";

  const btnLabel = loading
    ? "Please wait…"
    : connected
    ? "Disconnect"
    : "Connect";

  const btnStyle = connected
    ? "bg-vpn-red hover:bg-red-600"
    : "bg-vpn-green hover:bg-green-600";

  return (
    <div className="flex flex-col items-center justify-between h-full px-6 py-8">
      {/* Status indicator */}
      <div className="flex flex-col items-center gap-4 mt-4">
        <div className={`w-20 h-20 rounded-full ${dotColor} transition-all duration-500`} />
        <span className="text-lg font-semibold tracking-wide">
          {connected ? "Connected" : connecting ? "Connecting…" : "Disconnected"}
        </span>
      </div>

      {/* Server info */}
      <div className="flex flex-col items-center gap-1">
        <span className="text-3xl">{server.country_flag}</span>
        <span className="text-base font-medium">
          {server.city}, {server.country}
        </span>
        {server.ping_ms !== null && (
          <span className="text-xs text-white/50">{server.ping_ms} ms</span>
        )}
      </div>

      {/* Uptime & stats */}
      <div className="flex flex-col items-center gap-3 w-full">
        {connected && (
          <>
            <div className="text-2xl font-mono font-semibold text-vpn-green">
              {formatDuration(uptime)}
            </div>
            <div className="flex gap-8 text-sm text-white/70">
              <div className="flex flex-col items-center">
                <span className="text-xs text-white/40">↑ Upload</span>
                <span>{formatBytes(stats.bytes_sent)}</span>
              </div>
              <div className="flex flex-col items-center">
                <span className="text-xs text-white/40">↓ Download</span>
                <span>{formatBytes(stats.bytes_received)}</span>
              </div>
            </div>
          </>
        )}

        {error && (
          <div className="text-xs text-vpn-red text-center px-4">{error}</div>
        )}
      </div>

      {/* Connect / Disconnect button */}
      <button
        onClick={handleToggle}
        disabled={loading || connecting}
        className={`w-full py-3 rounded-xl font-semibold text-white transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed ${btnStyle}`}
      >
        {btnLabel}
      </button>
    </div>
  );
}
