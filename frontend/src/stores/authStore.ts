import { create } from 'zustand';
import type { User } from '@/types/auth';
import { authApi } from '@/api/auth';

interface AuthState {
  user: User | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  login: (email: string, password: string) => Promise<void>;
  logout: () => void;
  initialize: () => Promise<void>;
  setLanguagePreference: (language: string) => Promise<void>;
  updateProfile: (data: { full_name?: string; language?: string }) => Promise<void>;
  uploadAvatar: (file: File) => Promise<void>;
}

export const useAuthStore = create<AuthState>((set) => ({
  user: null,
  isAuthenticated: false,
  isLoading: true,

  login: async (email, password) => {
    const { data } = await authApi.login({ email, password });
    localStorage.setItem(
      'konto_tokens',
      JSON.stringify({
        access_token: data.access_token,
        refresh_token: data.refresh_token,
      })
    );

    // Fetch user profile
    const { data: user } = await authApi.me();
    localStorage.setItem('konto_user', JSON.stringify(user));
    set({ user, isAuthenticated: true });
  },

  logout: () => {
    localStorage.removeItem('konto_tokens');
    localStorage.removeItem('konto_user');
    set({ user: null, isAuthenticated: false });
  },

  initialize: async () => {
    const tokens = localStorage.getItem('konto_tokens');
    if (!tokens) {
      set({ isLoading: false });
      return;
    }

    try {
      JSON.parse(tokens); // validate JSON

      // Try to use stored user first for fast restore
      const savedUser = localStorage.getItem('konto_user');
      if (savedUser) {
        const user = JSON.parse(savedUser) as User;
        set({ user, isAuthenticated: true, isLoading: false });

        // Refresh user data in background
        authApi.me().then(({ data }) => {
          localStorage.setItem('konto_user', JSON.stringify(data));
          set({ user: data });
        }).catch((err) => {
          // Only clear tokens on 401 (expired/invalid) — not on network errors
          const status = err?.response?.status;
          if (status === 401) {
            localStorage.removeItem('konto_tokens');
            localStorage.removeItem('konto_user');
            set({ user: null, isAuthenticated: false });
          }
        });
        return;
      }

      // No cached user, fetch from API
      const { data: user } = await authApi.me();
      localStorage.setItem('konto_user', JSON.stringify(user));
      set({ user, isAuthenticated: true, isLoading: false });
    } catch (err: unknown) {
      const status = (err as { response?: { status?: number } })?.response?.status;
      if (status === 401) {
        // Token truly invalid — clear and force re-login
        localStorage.removeItem('konto_tokens');
        localStorage.removeItem('konto_user');
        set({ isLoading: false });
      } else {
        // Network error / backend not ready — keep tokens, use cached user
        const savedUser = localStorage.getItem('konto_user');
        if (savedUser) {
          set({ user: JSON.parse(savedUser) as User, isAuthenticated: true, isLoading: false });
        } else {
          set({ isLoading: false });
        }
      }
    }
  },

  setLanguagePreference: async (language) => {
    const { data } = await authApi.updateLanguage(language);
    localStorage.setItem('konto_user', JSON.stringify(data));
    set({ user: data });
  },

  updateProfile: async (profile) => {
    const { data } = await authApi.updateProfile(profile);
    localStorage.setItem('konto_user', JSON.stringify(data));
    set({ user: data });
  },

  uploadAvatar: async (file) => {
    const { data } = await authApi.uploadAvatar(file);
    localStorage.setItem('konto_user', JSON.stringify(data));
    set({ user: data });
  },
}));
