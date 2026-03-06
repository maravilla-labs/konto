import { useState, useCallback, useEffect } from 'react';
import { useLocation } from 'react-router-dom';
import { navItems, type NavItem } from '@/lib/navigation';

const STORAGE_KEY = 'maravilla_recent_pages';
const MAX_RECENT = 10;

function loadRecent(): string[] {
  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    return stored ? JSON.parse(stored) : [];
  } catch {
    return [];
  }
}

function saveRecent(paths: string[]) {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(paths));
}

export function useRecentPages() {
  const [recentPaths, setRecentPaths] = useState<string[]>(loadRecent);
  const location = useLocation();

  useEffect(() => {
    const path = location.pathname;
    // Only track known nav paths
    const isKnown = navItems.some((item) => item.path === path);
    if (!isKnown) return;

    setRecentPaths((prev) => {
      const filtered = prev.filter((p) => p !== path);
      const updated = [path, ...filtered].slice(0, MAX_RECENT);
      saveRecent(updated);
      return updated;
    });
  }, [location.pathname]);

  const recentItems: NavItem[] = recentPaths
    .map((path) => navItems.find((item) => item.path === path))
    .filter((item): item is NavItem => item !== undefined);

  const clearRecent = useCallback(() => {
    setRecentPaths([]);
    localStorage.removeItem(STORAGE_KEY);
  }, []);

  return { recentItems, clearRecent };
}
