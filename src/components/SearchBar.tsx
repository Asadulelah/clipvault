import { useCallback, useEffect, useRef, useState } from "react";
import { Search, X } from "lucide-react";

interface SearchBarProps {
  onSearch: (query: string) => void;
  onClear: () => void;
}

export default function SearchBar({ onSearch, onClear }: SearchBarProps) {
  const [value, setValue] = useState("");
  const inputRef = useRef<HTMLInputElement>(null);
  const timerRef = useRef<ReturnType<typeof setTimeout>>();

  const handleChange = useCallback(
    (text: string) => {
      setValue(text);
      if (timerRef.current) clearTimeout(timerRef.current);
      timerRef.current = setTimeout(() => {
        if (text.trim()) {
          onSearch(text.trim());
        } else {
          onClear();
        }
      }, 300);
    },
    [onSearch, onClear]
  );

  const handleClear = () => {
    setValue("");
    onClear();
    inputRef.current?.focus();
  };

  useEffect(() => {
    inputRef.current?.focus();
    return () => {
      if (timerRef.current) clearTimeout(timerRef.current);
    };
  }, []);

  return (
    <div
      className="flex items-center gap-2.5 px-3 py-2 mx-2.5 mt-2.5 rounded-lg"
      style={{ background: "var(--bg-card)", border: "1px solid var(--border)" }}
    >
      <Search size={14} style={{ color: "var(--text-muted)", flexShrink: 0 }} />
      <input
        ref={inputRef}
        type="text"
        placeholder="Search clipboard..."
        value={value}
        onChange={(e) => handleChange(e.target.value)}
        className="flex-1 bg-transparent border-none outline-none text-[13px]"
        style={{ color: "var(--text-primary)" }}
      />
      {value && (
        <button onClick={handleClear} className="p-0 bg-transparent border-none cursor-pointer">
          <X size={14} style={{ color: "var(--text-muted)" }} />
        </button>
      )}
    </div>
  );
}
