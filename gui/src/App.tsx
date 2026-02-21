import { useState, useEffect, useCallback } from "react";
import MainScreen from "./screens/MainScreen";
import ServerListScreen from "./screens/ServerListScreen";
import SettingsScreen from "./screens/SettingsScreen";
import { AppSettings, DEFAULT_SETTINGS, ServerInfo } from "./types";

type Screen = "main" | "servers" | "settings";

const SETTINGS_KEY = "nysvpb_settings";

function loadSettings(): AppSettings {
  try {
    const raw = localStorage.getItem(SETTINGS_KEY);
    return raw ? { ...DEFAULT_SETTINGS, ...JSON.parse(raw) } : DEFAULT_SETTINGS;
  } catch {
    return DEFAULT_SETTINGS;
  }
}

function saveSettings(s: AppSettings): void {
  localStorage.setItem(SETTINGS_KEY, JSON.stringify(s));
}

export default function App() {
  const [screen, setScreen] = useState<Screen>("main");
  const [settings, setSettings] = useState<AppSettings>(loadSettings);
  const [selectedServer, setSelectedServer] = useState<ServerInfo | null>(null);

  useEffect(() => {
    saveSettings(settings);
  }, [settings]);

  const handleSettingsChange = useCallback((updated: AppSettings) => {
    setSettings(updated);
  }, []);

  const handleServerSelect = useCallback(
    (server: ServerInfo) => {
      setSelectedServer(server);
      setSettings((s) => ({ ...s, selected_server_id: server.id }));
      setScreen("main");
    },
    []
  );

  return (
    <div className="flex flex-col h-screen w-full bg-[#1a1a2e] text-white overflow-hidden">
      {/* Header navigation */}
      <nav className="flex items-center justify-between px-4 py-3 bg-[#16213e] border-b border-white/10">
        <button
          onClick={() => setScreen("main")}
          className={`text-sm font-semibold transition-colors ${
            screen === "main" ? "text-white" : "text-white/40 hover:text-white/70"
          }`}
        >
          NySVPN
        </button>
        <div className="flex gap-3">
          <NavButton
            label="Servers"
            active={screen === "servers"}
            onClick={() => setScreen("servers")}
          />
          <NavButton
            label="Settings"
            active={screen === "settings"}
            onClick={() => setScreen("settings")}
          />
        </div>
      </nav>

      {/* Screen content */}
      <main className="flex-1 overflow-hidden">
        {screen === "main" && (
          <MainScreen selectedServer={selectedServer} settings={settings} />
        )}
        {screen === "servers" && (
          <ServerListScreen onSelect={handleServerSelect} />
        )}
        {screen === "settings" && (
          <SettingsScreen settings={settings} onChange={handleSettingsChange} />
        )}
      </main>
    </div>
  );
}

function NavButton({
  label,
  active,
  onClick,
}: {
  label: string;
  active: boolean;
  onClick: () => void;
}) {
  return (
    <button
      onClick={onClick}
      className={`text-xs px-3 py-1 rounded-full transition-colors ${
        active
          ? "bg-white/20 text-white"
          : "text-white/40 hover:text-white/70 hover:bg-white/10"
      }`}
    >
      {label}
    </button>
  );
}
