import { useCallback, useEffect, useRef } from "react";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { Settings, Trash2 } from "lucide-react";
import { useClipboardStore } from "../store/clipboardStore";
import { useSettingsStore } from "../store/settingsStore";
import { useClipboard } from "../hooks/useClipboard";
import { useSettings } from "../hooks/useSettings";
import { ClipboardItem } from "../types";
import SearchBar from "./SearchBar";
import ClipItem from "./ClipItem";

interface ClipboardPanelProps {
  onOpenSettings: () => void;
}

export default function ClipboardPanel({ onOpenSettings }: ClipboardPanelProps) {
  const {
    items,
    selectedIndex,
    searchQuery,
    setItems,
    prependItem,
    removeItem,
    updateItem,
    setSelectedIndex,
    clearUnpinned,
    setSearchQuery,
  } = useClipboardStore();

  const { settings, setSettings } = useSettingsStore();
  const { fetchHistory, searchHistory, pinItem, deleteItem, clearHistory, copyItem, hideWindow } =
    useClipboard();
  const { fetchSettings } = useSettings();
  const listRef = useRef<HTMLDivElement>(null);
  const confirmingClear = useRef(false);

  // Load history and settings on mount
  useEffect(() => {
    fetchHistory(50, 0).then(setItems).catch(console.error);
    fetchSettings().then(setSettings).catch(console.error);
  }, []);

  // Listen for clipboard updates from Rust
  useEffect(() => {
    const unlisten = listen<ClipboardItem>("clipboard-updated", (event) => {
      prependItem(event.payload);
    });
    return () => {
      unlisten.then((fn) => fn());
    };
  }, [prependItem]);

  // Auto-hide on focus loss
  useEffect(() => {
    const unlisten = getCurrentWindow().onFocusChanged(({ payload: focused }) => {
      if (!focused) {
        hideWindow().catch(console.error);
      }
    });
    return () => {
      unlisten.then((fn) => fn());
    };
  }, [hideWindow]);

  // Keyboard navigation
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      switch (e.key) {
        case "Escape":
          hideWindow().catch(console.error);
          break;
        case "ArrowDown":
          e.preventDefault();
          setSelectedIndex(Math.min(selectedIndex + 1, items.length - 1));
          break;
        case "ArrowUp":
          e.preventDefault();
          setSelectedIndex(Math.max(selectedIndex - 1, -1));
          break;
        case "Enter":
          if (selectedIndex >= 0 && selectedIndex < items.length) {
            handleCopy(items[selectedIndex].id);
          }
          break;
      }
    };
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [selectedIndex, items, hideWindow]);

  // Scroll selected item into view
  useEffect(() => {
    if (selectedIndex >= 0 && listRef.current) {
      const el = listRef.current.children[selectedIndex] as HTMLElement;
      el?.scrollIntoView({ block: "nearest" });
    }
  }, [selectedIndex]);

  const handleSearch = useCallback(
    (query: string) => {
      setSearchQuery(query);
      searchHistory(query).then(setItems).catch(console.error);
    },
    [searchHistory, setItems, setSearchQuery]
  );

  const handleClearSearch = useCallback(() => {
    setSearchQuery("");
    fetchHistory(50, 0).then(setItems).catch(console.error);
  }, [fetchHistory, setItems, setSearchQuery]);

  const handleCopy = useCallback(
    async (id: number) => {
      try {
        // Hide window first so the target app regains focus
        await hideWindow();
        // Small delay to let the target app get focus
        await new Promise((r) => setTimeout(r, 50));
        // Copy to clipboard and auto-paste (Rust simulates Ctrl+V)
        await copyItem(id);
      } catch (e) {
        console.error("Failed to copy:", e);
      }
    },
    [copyItem, hideWindow]
  );

  const handlePin = useCallback(
    async (id: number) => {
      try {
        const newPinned = await pinItem(id);
        updateItem(id, { is_pinned: newPinned });
      } catch (e) {
        console.error("Failed to pin:", e);
      }
    },
    [pinItem, updateItem]
  );

  const handleDelete = useCallback(
    async (id: number) => {
      try {
        await deleteItem(id);
        removeItem(id);
      } catch (e) {
        console.error("Failed to delete:", e);
      }
    },
    [deleteItem, removeItem]
  );

  const handleClearAll = useCallback(async () => {
    if (!confirmingClear.current) {
      confirmingClear.current = true;
      // Reset after 3 seconds if not confirmed
      setTimeout(() => {
        confirmingClear.current = false;
      }, 3000);
      return;
    }
    try {
      await clearHistory();
      clearUnpinned();
      confirmingClear.current = false;
    } catch (e) {
      console.error("Failed to clear:", e);
    }
  }, [clearHistory, clearUnpinned]);

  // Split items into pinned and unpinned
  const pinnedItems = items.filter((i) => i.is_pinned);
  const recentItems = items.filter((i) => !i.is_pinned);

  return (
    <div className="flex flex-col h-full animate-slide-up">
      {/* Search Bar */}
      <SearchBar onSearch={handleSearch} onClear={handleClearSearch} />

      {/* Item List */}
      <div className="flex-1 overflow-y-auto mt-2 pb-1" ref={listRef}>
        {/* Pinned Section */}
        {pinnedItems.length > 0 && (
          <>
            <div
              className="px-3.5 py-1 text-[11px] font-semibold uppercase tracking-wider"
              style={{ color: "var(--pin-color)" }}
            >
              Pinned
            </div>
            {pinnedItems.map((item, i) => (
              <ClipItem
                key={item.id}
                item={item}
                isSelected={i === selectedIndex}
                showSourceApp={settings.show_source_app}
                onCopy={handleCopy}
                onPin={handlePin}
                onDelete={handleDelete}
              />
            ))}
            <div
              className="mx-3 my-1"
              style={{ borderBottom: "1px solid var(--border)" }}
            />
          </>
        )}

        {/* Recent Section */}
        {recentItems.length > 0 && (
          <>
            {pinnedItems.length > 0 && (
              <div
                className="px-3.5 py-1 text-[11px] font-semibold uppercase tracking-wider"
                style={{ color: "var(--text-muted)" }}
              >
                Recent
              </div>
            )}
            {recentItems.map((item, i) => (
              <ClipItem
                key={item.id}
                item={item}
                isSelected={pinnedItems.length + i === selectedIndex}
                showSourceApp={settings.show_source_app}
                onCopy={handleCopy}
                onPin={handlePin}
                onDelete={handleDelete}
              />
            ))}
          </>
        )}

        {/* Empty State */}
        {items.length === 0 && (
          <div
            className="flex items-center justify-center h-32 text-sm"
            style={{ color: "var(--text-muted)" }}
          >
            {searchQuery ? "No results found" : "No clipboard history yet"}
          </div>
        )}
      </div>

      {/* Footer */}
      <div
        className="flex items-center justify-between px-3 py-2"
        style={{ borderTop: "1px solid var(--border)" }}
      >
        <button
          onClick={handleClearAll}
          className="flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-[12px] border-none cursor-pointer transition-colors"
          style={{
            background: "transparent",
            color: "var(--text-muted)",
          }}
          onMouseEnter={(e) => { e.currentTarget.style.color = "var(--danger)"; e.currentTarget.style.background = "var(--bg-card)"; }}
          onMouseLeave={(e) => { e.currentTarget.style.color = "var(--text-muted)"; e.currentTarget.style.background = "transparent"; }}
        >
          <Trash2 size={13} />
          Clear all
        </button>
        <button
          onClick={onOpenSettings}
          className="flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-[12px] border-none cursor-pointer transition-colors"
          style={{
            background: "transparent",
            color: "var(--text-muted)",
          }}
          onMouseEnter={(e) => { e.currentTarget.style.color = "var(--text-primary)"; e.currentTarget.style.background = "var(--bg-card)"; }}
          onMouseLeave={(e) => { e.currentTarget.style.color = "var(--text-muted)"; e.currentTarget.style.background = "transparent"; }}
        >
          <Settings size={13} />
          Settings
        </button>
      </div>
    </div>
  );
}
