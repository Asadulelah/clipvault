import { create } from "zustand";
import { ClipboardItem } from "../types";

interface ClipboardStore {
  items: ClipboardItem[];
  searchQuery: string;
  selectedIndex: number;
  isLoading: boolean;
  setItems: (items: ClipboardItem[]) => void;
  prependItem: (item: ClipboardItem) => void;
  removeItem: (id: number) => void;
  updateItem: (id: number, updates: Partial<ClipboardItem>) => void;
  setSearchQuery: (query: string) => void;
  setSelectedIndex: (index: number) => void;
  clearUnpinned: () => void;
  setLoading: (loading: boolean) => void;
}

export const useClipboardStore = create<ClipboardStore>((set) => ({
  items: [],
  searchQuery: "",
  selectedIndex: -1,
  isLoading: false,
  setItems: (items) => set({ items }),
  prependItem: (item) =>
    set((state) => ({
      items: [item, ...state.items.filter((i) => i.id !== item.id)],
    })),
  removeItem: (id) =>
    set((state) => ({
      items: state.items.filter((i) => i.id !== id),
    })),
  updateItem: (id, updates) =>
    set((state) => ({
      items: state.items.map((i) => (i.id === id ? { ...i, ...updates } : i)),
    })),
  setSearchQuery: (searchQuery) => set({ searchQuery }),
  setSelectedIndex: (selectedIndex) => set({ selectedIndex }),
  clearUnpinned: () =>
    set((state) => ({
      items: state.items.filter((i) => i.is_pinned),
    })),
  setLoading: (isLoading) => set({ isLoading }),
}));
