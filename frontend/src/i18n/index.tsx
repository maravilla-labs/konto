import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
  type ReactNode,
} from 'react';
import { MESSAGES } from '@/i18n/messages';
import { normalizeLanguage, type SupportedLanguage } from '@/lib/language';
import { useAuthStore } from '@/stores/authStore';

const STORAGE_KEY = 'hope.ui_language';
const DEFAULT_LANGUAGE: SupportedLanguage = 'en';

interface I18nContextValue {
  language: SupportedLanguage;
  setLanguage: (language: SupportedLanguage) => void;
  t: (key: string, fallback?: string) => string;
}

const I18nContext = createContext<I18nContextValue | undefined>(undefined);

function readInitialLanguage(): SupportedLanguage {
  return readLoggedOutLanguage();
}

function readStoredLanguage(): SupportedLanguage | undefined {
  if (typeof window === 'undefined') return undefined;
  return normalizeLanguage(window.localStorage.getItem(STORAGE_KEY));
}

function readBrowserLanguage(): SupportedLanguage | undefined {
  if (typeof window === 'undefined') return undefined;
  return normalizeLanguage(window.navigator.language);
}

function readLoggedOutLanguage(): SupportedLanguage {
  return readStoredLanguage() ?? readBrowserLanguage() ?? DEFAULT_LANGUAGE;
}

function persistLanguage(language: SupportedLanguage) {
  if (typeof window !== 'undefined') {
    window.localStorage.setItem(STORAGE_KEY, language);
  }
}

export function I18nProvider({ children }: { children: ReactNode }) {
  const user = useAuthStore((s) => s.user);
  const isAuthenticated = useAuthStore((s) => s.isAuthenticated);
  const setLanguagePreference = useAuthStore((s) => s.setLanguagePreference);
  const [language, setLanguageState] = useState<SupportedLanguage>(readInitialLanguage);

  const setLanguage = useCallback((next: SupportedLanguage) => {
    setLanguageState(next);
    persistLanguage(next);
    if (isAuthenticated) {
      void setLanguagePreference(next).catch(() => undefined);
    }
  }, [isAuthenticated, setLanguagePreference]);

  useEffect(() => {
    const next = isAuthenticated
      ? normalizeLanguage(user?.language) ?? readLoggedOutLanguage()
      : readLoggedOutLanguage();
    setLanguageState(next);
    persistLanguage(next);
  }, [isAuthenticated, user?.language]);

  const t = useCallback(
    (key: string, fallback?: string) =>
      MESSAGES[language][key] ?? MESSAGES.en[key] ?? fallback ?? key,
    [language],
  );

  const value = useMemo(
    () => ({ language, setLanguage, t }),
    [language, setLanguage, t],
  );

  return <I18nContext.Provider value={value}>{children}</I18nContext.Provider>;
}

export function useI18n() {
  const ctx = useContext(I18nContext);
  if (!ctx) {
    throw new Error('useI18n must be used within I18nProvider');
  }
  return ctx;
}
