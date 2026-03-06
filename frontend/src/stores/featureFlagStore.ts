import { create } from 'zustand';

const STORAGE_KEY = 'konto_experimental';

interface FeatureFlagState {
  experimental: boolean;
  setExperimental: (value: boolean) => void;
  initialize: () => void;
}

export const useFeatureFlagStore = create<FeatureFlagState>((set) => ({
  experimental: false,

  setExperimental: (value: boolean) => {
    localStorage.setItem(STORAGE_KEY, String(value));
    set({ experimental: value });
  },

  initialize: () => {
    // Check localStorage first
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored === 'true') {
      set({ experimental: true });
      return;
    }

    // Check URL query params (for Tauri CLI forwarding)
    const params = new URLSearchParams(window.location.search);
    if (params.get('experimental') === 'true') {
      localStorage.setItem(STORAGE_KEY, 'true');
      set({ experimental: true });
    }
  },
}));
