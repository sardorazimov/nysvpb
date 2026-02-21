/**
 * Server List Screen — shows available VPN servers with flag, city, and latency.
 * Users can search/filter and click a server to select it.
 */
import { useState, useEffect, useMemo } from "react";
import { ServerInfo } from "../types";
import { listServers } from "../tauri-commands";

interface Props {
  onSelect: (server: ServerInfo) => void;
}

export default function ServerListScreen({ onSelect }: Props) {
  const [servers, setServers] = useState<ServerInfo[]>([]);
  const [search, setSearch] = useState("");
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    listServers()
      .then(setServers)
      .catch(() => {})
      .finally(() => setLoading(false));
  }, []);

  const filtered = useMemo(() => {
    const q = search.toLowerCase();
    return servers.filter(
      (s) =>
        s.city.toLowerCase().includes(q) ||
        s.country.toLowerCase().includes(q)
    );
  }, [servers, search]);

  return (
    <div className="flex flex-col h-full">
      {/* Search bar */}
      <div className="px-4 py-3 bg-[#16213e] border-b border-white/10">
        <input
          type="text"
          placeholder="Search servers…"
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          className="w-full bg-white/10 text-white placeholder-white/30 rounded-lg px-3 py-2 text-sm outline-none focus:ring-1 focus:ring-white/30"
        />
      </div>

      {/* Server list */}
      <div className="flex-1 overflow-y-auto px-2 py-2">
        {loading ? (
          <div className="flex items-center justify-center h-full text-white/40 text-sm">
            Loading servers…
          </div>
        ) : filtered.length === 0 ? (
          <div className="flex items-center justify-center h-full text-white/40 text-sm">
            No servers found
          </div>
        ) : (
          filtered.map((server) => (
            <ServerRow key={server.id} server={server} onSelect={onSelect} />
          ))
        )}
      </div>
    </div>
  );
}

function ServerRow({
  server,
  onSelect,
}: {
  server: ServerInfo;
  onSelect: (s: ServerInfo) => void;
}) {
  return (
    <button
      onClick={() => onSelect(server)}
      className="w-full flex items-center gap-3 px-3 py-3 rounded-xl hover:bg-white/10 transition-colors text-left"
    >
      <span className="text-2xl">{server.country_flag}</span>
      <div className="flex-1 min-w-0">
        <div className="text-sm font-medium truncate">{server.city}</div>
        <div className="text-xs text-white/50 truncate">{server.country}</div>
      </div>
      <div className="text-xs text-white/40 shrink-0">
        {server.ping_ms !== null ? `${server.ping_ms} ms` : "—"}
      </div>
    </button>
  );
}
