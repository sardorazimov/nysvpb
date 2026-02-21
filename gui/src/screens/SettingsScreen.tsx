/**
 * Settings Screen — protocol selection, kill switch, auto-connect, DNS leak
 * protection, and launch-at-login toggles.  Settings are persisted by the
 * parent App component via localStorage.
 */
import { AppSettings } from "../types";

interface Props {
  settings: AppSettings;
  onChange: (updated: AppSettings) => void;
}

export default function SettingsScreen({ settings, onChange }: Props) {
  const set = <K extends keyof AppSettings>(key: K, value: AppSettings[K]) =>
    onChange({ ...settings, [key]: value });

  return (
    <div className="flex flex-col h-full overflow-y-auto px-4 py-4 gap-6">
      {/* Protocol */}
      <Section title="Protocol">
        <div className="flex gap-2">
          {(["wireguard", "openvpn"] as const).map((p) => (
            <button
              key={p}
              onClick={() => set("protocol", p)}
              className={`flex-1 py-2 rounded-lg text-sm font-medium transition-colors ${
                settings.protocol === p
                  ? "bg-white/20 text-white"
                  : "bg-white/5 text-white/50 hover:bg-white/10"
              }`}
            >
              {p === "wireguard" ? "WireGuard" : "OpenVPN"}
            </button>
          ))}
        </div>
      </Section>

      {/* Toggles */}
      <Section title="Connection">
        <Toggle
          label="Kill Switch"
          description="Block all traffic if VPN drops"
          value={settings.kill_switch}
          onChange={(v) => set("kill_switch", v)}
        />
        <Toggle
          label="Auto-Connect"
          description="Connect automatically on launch"
          value={settings.auto_connect}
          onChange={(v) => set("auto_connect", v)}
        />
        <Toggle
          label="DNS Leak Protection"
          description="Route DNS through the VPN tunnel"
          value={settings.dns_leak_protection}
          onChange={(v) => set("dns_leak_protection", v)}
        />
      </Section>

      <Section title="System">
        <Toggle
          label="Launch at Login"
          description="Start NySVPN when you log in"
          value={settings.launch_at_login}
          onChange={(v) => set("launch_at_login", v)}
        />
      </Section>

      <div className="text-center text-xs text-white/20 mt-auto pb-2">
        NySVPN v0.1.0
      </div>
    </div>
  );
}

function Section({
  title,
  children,
}: {
  title: string;
  children: React.ReactNode;
}) {
  return (
    <div className="flex flex-col gap-3">
      <h2 className="text-xs font-semibold text-white/40 uppercase tracking-wider">
        {title}
      </h2>
      <div className="flex flex-col gap-2 bg-white/5 rounded-xl px-4 py-2">
        {children}
      </div>
    </div>
  );
}

function Toggle({
  label,
  description,
  value,
  onChange,
}: {
  label: string;
  description: string;
  value: boolean;
  onChange: (v: boolean) => void;
}) {
  return (
    <div
      className="flex items-center justify-between py-2 cursor-pointer"
      onClick={() => onChange(!value)}
    >
      <div>
        <div className="text-sm font-medium">{label}</div>
        <div className="text-xs text-white/40">{description}</div>
      </div>
      <div
        className={`relative w-10 h-6 rounded-full transition-colors duration-200 ${
          value ? "bg-vpn-green" : "bg-white/20"
        }`}
      >
        <div
          className={`absolute top-1 w-4 h-4 bg-white rounded-full shadow transition-transform duration-200 ${
            value ? "translate-x-5" : "translate-x-1"
          }`}
        />
      </div>
    </div>
  );
}
