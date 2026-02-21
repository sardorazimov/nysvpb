/**
 * StatusBadge — displays a contextual status notification with three variants:
 *   - "bug"      : error / failure (red)
 *   - "success"  : connection succeeded (green)
 *   - "readplay" : ready to connect (amber)
 */

export type StatusBadgeVariant = "bug" | "success" | "readplay";

interface Props {
  variant: StatusBadgeVariant;
  message: string;
}

const styles: Record<StatusBadgeVariant, { wrapper: string; icon: string }> = {
  bug: {
    wrapper: "bg-vpn-red/20 border border-vpn-red/40 text-vpn-red",
    icon: "🐛",
  },
  success: {
    wrapper: "bg-vpn-green/20 border border-vpn-green/40 text-vpn-green",
    icon: "✅",
  },
  readplay: {
    wrapper: "bg-vpn-yellow/20 border border-vpn-yellow/40 text-vpn-yellow",
    icon: "▶️",
  },
};

export default function StatusBadge({ variant, message }: Props) {
  const { wrapper, icon } = styles[variant];
  return (
    <div
      className={`flex items-center gap-2 w-full px-4 py-2 rounded-xl text-sm font-medium ${wrapper}`}
    >
      <span>{icon}</span>
      <span>{message}</span>
    </div>
  );
}
