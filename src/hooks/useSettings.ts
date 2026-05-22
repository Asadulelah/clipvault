import { invoke } from "@tauri-apps/api/core";
import { Settings } from "../types";

export function useSettings() {
  const fetchSettings = async (): Promise<Settings> => {
    return invoke("get_settings");
  };

  const saveSettings = async (settings: Settings): Promise<void> => {
    return invoke("save_settings", { settings });
  };

  return { fetchSettings, saveSettings };
}
