import { useEffect, useState } from "react";
import { ArrowLeft, Sun, Moon, Monitor } from "lucide-react";
import { useSettingsStore } from "../store/settingsStore";
import { useSettings } from "../hooks/useSettings";
import { Settings } from "../types";

interface SettingsPanelProps {
  onBack: () => void;
}

export default function SettingsPanel({ onBack }: SettingsPanelProps) {
  const { settings, setSettings, updateSetting } = useSettingsStore();
  const { fetchSettings, saveSettings } = useSettings();
  const [loaded, setLoaded] = useState(false);

  useEffect(() => {
    fetchSettings()
      .then((s) => {
        setSettings(s);
        setLoaded(true);
      })
      .catch(console.error);
  }, []);

  const handleChange = async <K extends keyof Settings>(key: K, value: Settings[K]) => {
    updateSetting(key, value);
    const updated = { ...settings, [key]: value };
    try {
      await saveSettings(updated);
    } catch (e) {
      console.error("Failed to save settings:", e);
    }
  };

  // Apply theme
  useEffect(() => {
    const root = document.documentElement;
    if (settings.theme === "light") {
      root.setAttribute("data-theme", "light");
    } else if (settings.theme === "dark") {
      root.removeAttribute("data-theme");
    } else {
      // system
      const prefersDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
      if (prefersDark) {
        root.removeAttribute("data-theme");
      } else {
        root.setAttribute("data-theme", "light");
      }
    }
  }, [settings.theme]);

  if (!loaded) return null;

  return (
    <div className="flex flex-col h-full animate-slide-up">
      {/* Header */}
      <div
        className="flex items-center gap-3 px-4 py-3"
        style={{ borderBottom: "1px solid var(--border)" }}
      >
        <button
          onClick={onBack}
          className="p-1 rounded-md border-none cursor-pointer"
          style={{ background: "transparent", color: "var(--text-primary)" }}
        >
          <ArrowLeft size={18} />
        </button>
        <span className="text-[14px] font-semibold" style={{ color: "var(--text-primary)" }}>
          Settings
        </span>
      </div>

      <div className="flex-1 overflow-y-auto px-4 pt-4">
        {/* Theme */}
        <SettingSection label="Theme">
          <div className="flex gap-2">
            {(["light", "dark", "system"] as const).map((theme) => (
              <button
                key={theme}
                onClick={() => handleChange("theme", theme)}
                className="flex items-center gap-1.5 px-3 py-2 rounded-lg text-[13px] border-none cursor-pointer transition-colors"
                style={{
                  background: settings.theme === theme ? "var(--accent)" : "var(--bg-card)",
                  color: settings.theme === theme ? "#fff" : "var(--text-primary)",
                  border: settings.theme === theme ? "none" : "1px solid var(--border)",
                }}
              >
                {theme === "light" && <Sun size={14} />}
                {theme === "dark" && <Moon size={14} />}
                {theme === "system" && <Monitor size={14} />}
                {theme.charAt(0).toUpperCase() + theme.slice(1)}
              </button>
            ))}
          </div>
        </SettingSection>

        {/* Auto-start */}
        <ToggleSetting
          label="Launch on startup"
          description="Start ClipVault when you log in to Windows"
          value={settings.autostart}
          onChange={(v) => handleChange("autostart", v)}
        />

        {/* Show source app */}
        <ToggleSetting
          label="Show source app"
          description="Display which app the content was copied from"
          value={settings.show_source_app}
          onChange={(v) => handleChange("show_source_app", v)}
        />

        {/* History limit */}
        <SettingSection label="History limit">
          <p className="text-[12px] mb-2" style={{ color: "var(--text-muted)" }}>
            Maximum items to keep (0 = unlimited)
          </p>
          <input
            type="number"
            min={0}
            value={settings.history_limit}
            onChange={(e) => handleChange("history_limit", parseInt(e.target.value) || 0)}
            className="w-24 px-3 py-1.5 rounded-lg text-[13px] outline-none"
            style={{
              background: "var(--bg-card)",
              color: "var(--text-primary)",
              border: "1px solid var(--border)",
            }}
          />
        </SettingSection>

        {/* Hotkey */}
        <SettingSection label="Hotkey">
          <div
            className="inline-flex items-center px-3 py-1.5 rounded-lg text-[13px]"
            style={{
              background: "var(--bg-card)",
              color: "var(--text-muted)",
              border: "1px solid var(--border)",
            }}
          >
            {settings.hotkey}
          </div>
        </SettingSection>
      </div>
    </div>
  );
}

function SettingSection({ label, children }: { label: string; children: React.ReactNode }) {
  return (
    <div className="mb-5">
      <label
        className="text-[11px] font-semibold uppercase tracking-wider mb-2 block"
        style={{ color: "var(--text-muted)" }}
      >
        {label}
      </label>
      {children}
    </div>
  );
}

function ToggleSetting({
  label,
  description,
  value,
  onChange,
}: {
  label: string;
  description: string;
  value: boolean;
  onChange: (value: boolean) => void;
}) {
  return (
    <div
      className="flex items-center justify-between mb-5 px-3 py-3 rounded-lg"
      style={{ background: "var(--bg-card)", border: "1px solid var(--border)" }}
    >
      <div className="mr-4">
        <div className="text-[13px] font-medium" style={{ color: "var(--text-primary)" }}>
          {label}
        </div>
        <div className="text-[11px] mt-0.5" style={{ color: "var(--text-muted)" }}>
          {description}
        </div>
      </div>
      <button
        onClick={() => onChange(!value)}
        className="relative w-10 h-[22px] rounded-full border-none cursor-pointer transition-colors"
        style={{
          background: value ? "var(--accent)" : "var(--bg-card-hover)",
          flexShrink: 0,
        }}
      >
        <div
          className="absolute top-[3px] w-[16px] h-[16px] rounded-full bg-white transition-all"
          style={{
            left: value ? 21 : 3,
          }}
        />
      </button>
    </div>
  );
}
