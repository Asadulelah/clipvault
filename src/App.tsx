import { useState } from "react";
import ClipboardPanel from "./components/ClipboardPanel";
import SettingsPanel from "./components/SettingsPanel";

function App() {
  const [showSettings, setShowSettings] = useState(false);

  return (
    <div
      className="h-screen w-screen overflow-hidden"
      style={{
        background: "var(--bg-primary)",
        borderRadius: "8px",
        border: "1px solid var(--border)",
      }}
    >
      {showSettings ? (
        <SettingsPanel onBack={() => setShowSettings(false)} />
      ) : (
        <ClipboardPanel onOpenSettings={() => setShowSettings(true)} />
      )}
    </div>
  );
}

export default App;
