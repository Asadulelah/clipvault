import { Image as ImageIcon } from "lucide-react";

interface ThumbnailProps {
  blob: string | null;
}

export default function Thumbnail({ blob }: ThumbnailProps) {
  if (!blob) {
    return (
      <div
        className="flex items-center justify-center rounded"
        style={{
          width: 44,
          height: 44,
          background: "var(--bg-card-hover)",
          flexShrink: 0,
        }}
      >
        <ImageIcon size={18} style={{ color: "var(--text-muted)" }} />
      </div>
    );
  }

  return (
    <img
      src={`data:image/png;base64,${blob}`}
      alt="Clipboard image"
      className="rounded object-cover"
      style={{ width: 44, height: 44, flexShrink: 0 }}
    />
  );
}
