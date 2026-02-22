/** Mirror of shared::TunnelStatus from Rust. */
export type TunnelStatus =
  | "Disconnected"
  | "Connecting"
  | { Connected: { since: string; server: string } }
  | { Error: string };

/** Mirror of shared::TunnelStats from Rust. */
export interface TunnelStats {
  bytes_sent: number;
  bytes_received: number;
  last_handshake: string | null;
}

/** Mirror of shared::VpnConfig from Rust. */
export interface VpnConfig {
  server_addr: string;
  server_public_key: string;
  client_private_key: string;
  client_ip: string;
  dns_servers: string[];
  allowed_ips: string[];
}

/** Mirror of shared::ServerInfo from Rust. */
export interface ServerInfo {
  name: any;
  lng: any;
  lat: any;
  id: string;
  country: string;
  country_flag: string;
  city: string;
  address: string;
  public_key: string;
  ping_ms: number | null;
}

/** Application settings (persisted in localStorage). */
export interface AppSettings {
  kill_switch: boolean;
  auto_connect: boolean;
  dns_leak_protection: boolean;
  launch_at_login: boolean;
  protocol: "wireguard" | "openvpn";
  selected_server_id: string | null;
}

export const DEFAULT_SETTINGS: AppSettings = {
  kill_switch: false,
  auto_connect: false,
  dns_leak_protection: true,
  launch_at_login: false,
  protocol: "wireguard",
  selected_server_id: null,
};

/** Format bytes as a human-readable string. */
export function formatBytes(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  if (bytes < 1024 * 1024 * 1024) return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
  return `${(bytes / 1024 / 1024 / 1024).toFixed(2)} GB`;
}

/** Format seconds as hh:mm:ss. */
export function formatDuration(seconds: number): string {
  const h = Math.floor(seconds / 3600);
  const m = Math.floor((seconds % 3600) / 60);
  const s = seconds % 60;
  return [h, m, s].map((v) => String(v).padStart(2, "0")).join(":");
}

/** Return true when the status indicates a live connection. */
export function isConnected(status: TunnelStatus): boolean {
  return typeof status === "object" && "Connected" in status;
}
