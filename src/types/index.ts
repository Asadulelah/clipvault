export interface ClipboardItem {
  id: number;
  content_type: "text" | "image" | "url" | "html";
  content_text: string | null;
  content_blob: string | null;
  thumbnail_blob: string | null;
  preview_text: string | null;
  source_app: string | null;
  is_pinned: boolean;
  created_at: number;
  hash: string;
}

export interface Settings {
  theme: "light" | "dark" | "system";
  history_limit: number;
  autostart: boolean;
  hotkey: string;
  show_source_app: boolean;
}

export const DEFAULT_SETTINGS: Settings = {
  theme: "system",
  history_limit: 0,
  autostart: true,
  hotkey: "Super+V",
  show_source_app: true,
};
