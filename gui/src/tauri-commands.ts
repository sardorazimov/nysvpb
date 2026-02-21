/**
 * Tauri command wrappers.
 *
 * Each function calls a Rust command registered in gui/src-tauri/src/lib.rs.
 * During web-only development (without Tauri) the functions degrade gracefully.
 */

import { invoke } from "@tauri-apps/api/core";
import type { ServerInfo, TunnelStats, TunnelStatus, VpnConfig } from "./types";

export async function vpnConnect(config: VpnConfig): Promise<void> {
  await invoke<void>("vpn_connect", { config });
}

export async function vpnDisconnect(): Promise<void> {
  await invoke<void>("vpn_disconnect");
}

export async function vpnStatus(): Promise<TunnelStatus> {
  return invoke<TunnelStatus>("vpn_status");
}

export async function vpnStats(): Promise<TunnelStats> {
  return invoke<TunnelStats>("vpn_stats");
}

export async function listServers(): Promise<ServerInfo[]> {
  return invoke<ServerInfo[]>("list_servers");
}
