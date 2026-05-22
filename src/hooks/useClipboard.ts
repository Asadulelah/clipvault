import { invoke } from "@tauri-apps/api/core";
import { ClipboardItem } from "../types";

export function useClipboard() {
  const fetchHistory = async (limit: number, offset: number): Promise<ClipboardItem[]> => {
    return invoke("get_history", { limit, offset });
  };

  const searchHistory = async (query: string): Promise<ClipboardItem[]> => {
    return invoke("search_history", { query });
  };

  const pinItem = async (id: number): Promise<boolean> => {
    return invoke("pin_item", { id });
  };

  const deleteItem = async (id: number): Promise<void> => {
    return invoke("delete_item", { id });
  };

  const clearHistory = async (): Promise<void> => {
    return invoke("clear_history");
  };

  const copyItem = async (id: number): Promise<void> => {
    return invoke("copy_item", { id });
  };

  const hideWindow = async (): Promise<void> => {
    return invoke("hide_window");
  };

  return {
    fetchHistory,
    searchHistory,
    pinItem,
    deleteItem,
    clearHistory,
    copyItem,
    hideWindow,
  };
}
