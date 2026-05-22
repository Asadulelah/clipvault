import { ClipboardItem } from "../types";
import { Copy, Pin, Trash2, Link, FileText } from "lucide-react";
import Thumbnail from "./Thumbnail";

interface ClipItemProps {
  item: ClipboardItem;
  isSelected: boolean;
  showSourceApp: boolean;
  onCopy: (id: number) => void;
  onPin: (id: number) => void;
  onDelete: (id: number) => void;
}

function timeAgo(timestamp: number): string {
  const seconds = Math.floor((Date.now() - timestamp) / 1000);
  if (seconds < 60) return "now";
  const minutes = Math.floor(seconds / 60);
  if (minutes < 60) return `${minutes}m`;
  const hours = Math.floor(minutes / 60);
  if (hours < 24) return `${hours}h`;
  const days = Math.floor(hours / 24);
  return `${days}d`;
}

export default function ClipItem({
  item,
  isSelected,
  showSourceApp,
  onCopy,
  onPin,
  onDelete,
}: ClipItemProps) {
  const isImage = item.content_type === "image";
  const isUrl = item.content_type === "url";

  return (
    <div
      className="group relative mx-2 mb-1.5 rounded-lg cursor-pointer transition-all"
      style={{
        background: isSelected ? "var(--bg-card-hover)" : "var(--bg-card)",
        border: isSelected ? "1px solid var(--accent)" : "1px solid transparent",
      }}
      onClick={() => onCopy(item.id)}
      onMouseEnter={(e) => {
        if (!isSelected) {
          e.currentTarget.style.background = "var(--bg-card-hover)";
          e.currentTarget.style.border = "1px solid var(--border)";
        }
      }}
      onMouseLeave={(e) => {
        if (!isSelected) {
          e.currentTarget.style.background = "var(--bg-card)";
          e.currentTarget.style.border = "1px solid transparent";
        }
      }}
    >
      <div className="flex items-start gap-3 px-3 py-2.5">
        {/* Icon or thumbnail */}
        {isImage ? (
          <Thumbnail blob={item.thumbnail_blob} />
        ) : (
          <div
            className="flex items-center justify-center mt-0.5"
            style={{ width: 20, height: 20, flexShrink: 0 }}
          >
            {isUrl ? (
              <Link size={14} style={{ color: "var(--accent)" }} />
            ) : (
              <FileText size={14} style={{ color: "var(--text-muted)" }} />
            )}
          </div>
        )}

        {/* Text content */}
        <div className="flex-1 min-w-0">
          <div
            className="text-[13px] leading-[1.4]"
            style={{
              color: "var(--text-primary)",
              display: "-webkit-box",
              WebkitLineClamp: isImage ? 1 : 2,
              WebkitBoxOrient: "vertical",
              overflow: "hidden",
              wordBreak: "break-word",
            }}
          >
            {item.preview_text || "(empty)"}
          </div>
          <div className="flex items-center gap-2 mt-1">
            {showSourceApp && item.source_app && (
              <>
                <span
                  className="text-[11px] truncate max-w-[120px]"
                  style={{ color: "var(--text-muted)" }}
                >
                  {item.source_app}
                </span>
                <span style={{ color: "var(--text-muted)", fontSize: 11 }}>·</span>
              </>
            )}
            <span className="text-[11px]" style={{ color: "var(--text-muted)" }}>
              {timeAgo(item.created_at)}
            </span>
          </div>
        </div>

        {/* Pin indicator */}
        {item.is_pinned && (
          <Pin
            size={12}
            className="mt-1"
            style={{ color: "var(--pin-color)", flexShrink: 0 }}
            fill="var(--pin-color)"
          />
        )}
      </div>

      {/* Hover action toolbar */}
      <div
        className="hidden group-hover:flex items-center gap-0.5 absolute top-1.5 right-1.5 rounded-md px-1 py-0.5"
        style={{
          background: "var(--bg-primary)",
          border: "1px solid var(--border)",
          boxShadow: "0 2px 8px rgba(0,0,0,0.3)",
        }}
        onClick={(e) => e.stopPropagation()}
      >
        <button
          onClick={() => onCopy(item.id)}
          className="p-1.5 rounded border-none cursor-pointer transition-colors"
          style={{ background: "transparent" }}
          title="Copy & Paste"
        >
          <Copy size={13} style={{ color: "var(--text-muted)" }} />
        </button>
        <button
          onClick={() => onPin(item.id)}
          className="p-1.5 rounded border-none cursor-pointer transition-colors"
          style={{ background: "transparent" }}
          title={item.is_pinned ? "Unpin" : "Pin"}
        >
          <Pin
            size={13}
            style={{
              color: item.is_pinned ? "var(--pin-color)" : "var(--text-muted)",
            }}
            fill={item.is_pinned ? "var(--pin-color)" : "none"}
          />
        </button>
        <button
          onClick={() => onDelete(item.id)}
          className="p-1.5 rounded border-none cursor-pointer transition-colors"
          style={{ background: "transparent" }}
          title="Delete"
        >
          <Trash2 size={13} style={{ color: "var(--text-muted)" }} />
        </button>
      </div>
    </div>
  );
}
