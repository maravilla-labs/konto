import { useCallback, useEffect, useRef, useState } from 'react';
import type { Window as TauriWindow } from '@tauri-apps/api/window';

/** Cached Tauri window reference (resolved once, reused everywhere). */
let cachedWindow: TauriWindow | null = null;

async function getTauriWindow(): Promise<TauriWindow | null> {
  if (cachedWindow) return cachedWindow;
  try {
    const { getCurrentWindow } = await import('@tauri-apps/api/window');
    cachedWindow = getCurrentWindow();
    return cachedWindow;
  } catch {
    return null;
  }
}

function useTauriWindow() {
  const [isMaximized, setIsMaximized] = useState(false);
  const [isFullscreen, setIsFullscreen] = useState(false);
  const isSyncingRef = useRef(false);

  const syncState = useCallback(async () => {
    if (isSyncingRef.current) return;
    isSyncingRef.current = true;

    const win = await getTauriWindow();
    if (!win) {
      isSyncingRef.current = false;
      return;
    }

    try {
      // Small delay to let macOS animation settle
      await new Promise((r) => setTimeout(r, 100));
      setIsMaximized(await win.isMaximized());
      setIsFullscreen(await win.isFullscreen());
    } finally {
      isSyncingRef.current = false;
    }
  }, []);

  useEffect(() => {
    let unlisten: (() => void) | undefined;
    let cancelled = false;

    (async () => {
      const win = await getTauriWindow();
      if (!win || cancelled) return;
      setIsMaximized(await win.isMaximized());
      setIsFullscreen(await win.isFullscreen());
      const removeResizeListener = await win.onResized(syncState);
      if (cancelled) {
        removeResizeListener();
        return;
      }
      unlisten = removeResizeListener;
    })();

    return () => {
      cancelled = true;
      unlisten?.();
    };
  }, [syncState]);

  const minimize = useCallback(async () => {
    const win = await getTauriWindow();
    win?.minimize();
  }, []);

  const toggleMaximize = useCallback(async () => {
    const win = await getTauriWindow();
    win?.toggleMaximize();
  }, []);

  const toggleFullscreen = useCallback(async () => {
    const win = await getTauriWindow();
    if (!win) return;
    const fs = await win.isFullscreen();
    await win.setFullscreen(!fs);
  }, []);

  const close = useCallback(async () => {
    const win = await getTauriWindow();
    win?.close();
  }, []);

  return { isMaximized, isFullscreen, minimize, toggleMaximize, toggleFullscreen, close };
}

/** Syncs tauri-fullscreen class on <html> based on window state. */
export function useFullscreenSync() {
  const wasFullscreenRef = useRef(false);
  const isFixingPointerRef = useRef(false);

  useEffect(() => {
    const unlistenFns: Array<() => void> = [];
    let cancelled = false;
    let syncInFlight = false;
    let pointerFixTimer: number | undefined;
    let verifyTimer: number | undefined;

    (async () => {
      const win = await getTauriWindow();
      if (!win || cancelled) return;

      const fixPointerEvents = async () => {
        if (isFixingPointerRef.current) return;
        isFixingPointerRef.current = true;

        // macOS transparent + undecorated windows lose pointer event delivery
        // after exiting fullscreen. A resize bounce forces the WebView to
        // recalculate its content frame and restore hit-testing.
        try {
          const { PhysicalSize } = await import('@tauri-apps/api/dpi');
          const size = await win.innerSize();
          await win.setSize(new PhysicalSize(size.width + 1, size.height));
          await new Promise((r) => setTimeout(r, 50));
          await win.setSize(new PhysicalSize(size.width, size.height));
        } catch {
          /* ignore if APIs unavailable */
        } finally {
          isFixingPointerRef.current = false;
        }
      };

      const sync = async () => {
        if (syncInFlight || cancelled) return;
        syncInFlight = true;

        await new Promise((r) => setTimeout(r, 100));
        try {
          if (cancelled) return;

          const fs = await win.isFullscreen();
          const wasFs = wasFullscreenRef.current;
          wasFullscreenRef.current = fs;
          document.documentElement.classList.toggle('tauri-fullscreen', fs);

          if (wasFs && !fs) {
            // Wait for macOS fullscreen exit animation to complete
            if (pointerFixTimer) window.clearTimeout(pointerFixTimer);
            pointerFixTimer = window.setTimeout(() => {
              void fixPointerEvents();
            }, 550);
          }
        } finally {
          syncInFlight = false;
        }
      };

      const attach = async (listen: Promise<() => void>) => {
        const unlisten = await listen;
        if (cancelled) {
          unlisten();
          return;
        }
        unlistenFns.push(unlisten);
      };

      await sync();
      verifyTimer = window.setTimeout(() => {
        void sync();
      }, 300);

      await attach(win.onResized(sync));
      await attach(win.onMoved(sync));
      await attach(win.onScaleChanged(sync));
      await attach(win.onFocusChanged(sync));
    })();

    return () => {
      cancelled = true;
      if (pointerFixTimer) window.clearTimeout(pointerFixTimer);
      if (verifyTimer) window.clearTimeout(verifyTimer);
      for (const unlisten of unlistenFns) {
        unlisten();
      }
    };
  }, []);
}

export function MacTrafficLights() {
  const { isFullscreen, minimize, toggleFullscreen, close } = useTauriWindow();
  const [hovered, setHovered] = useState(false);

  if (isFullscreen) return null;

  return (
    <div
      data-no-drag
      className="flex items-center gap-2"
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
    >
      <button
        onClick={close}
        className="group flex h-3 w-3 items-center justify-center rounded-full bg-[#FF5F57] transition-colors hover:brightness-90"
        aria-label="Close"
      >
        {hovered && (
          <svg width="6" height="6" viewBox="0 0 6 6" stroke="#4D0000" strokeWidth="1.2" fill="none">
            <line x1="0.5" y1="0.5" x2="5.5" y2="5.5" />
            <line x1="5.5" y1="0.5" x2="0.5" y2="5.5" />
          </svg>
        )}
      </button>
      <button
        onClick={minimize}
        className="group flex h-3 w-3 items-center justify-center rounded-full bg-[#FEBC2E] transition-colors hover:brightness-90"
        aria-label="Minimize"
      >
        {hovered && (
          <svg width="6" height="2" viewBox="0 0 6 2" stroke="#995700" strokeWidth="1.2" fill="none">
            <line x1="0.5" y1="1" x2="5.5" y2="1" />
          </svg>
        )}
      </button>
      <button
        onClick={toggleFullscreen}
        className="group flex h-3 w-3 items-center justify-center rounded-full bg-[#28C840] transition-colors hover:brightness-90"
        aria-label="Fullscreen"
      >
        {hovered && (
          <svg width="6" height="6" viewBox="0 0 8 8" fill="none">
            <path d="M1,3 L1,1 L3,1" stroke="#006500" strokeWidth="1.2" fill="none" />
            <path d="M5,1 L7,1 L7,3" stroke="#006500" strokeWidth="1.2" fill="none" />
            <path d="M7,5 L7,7 L5,7" stroke="#006500" strokeWidth="1.2" fill="none" />
            <path d="M3,7 L1,7 L1,5" stroke="#006500" strokeWidth="1.2" fill="none" />
          </svg>
        )}
      </button>
    </div>
  );
}

export function WinControls() {
  const { isMaximized, isFullscreen, minimize, toggleMaximize, close } = useTauriWindow();

  if (isFullscreen) return null;

  return (
    <div data-no-drag className="flex">
      <button
        onClick={minimize}
        className="inline-flex h-8 w-11 items-center justify-center text-muted-foreground hover:bg-accent"
        aria-label="Minimize"
      >
        <svg width="10" height="1" viewBox="0 0 10 1" fill="currentColor">
          <rect width="10" height="1" />
        </svg>
      </button>
      <button
        onClick={toggleMaximize}
        className="inline-flex h-8 w-11 items-center justify-center text-muted-foreground hover:bg-accent"
        aria-label={isMaximized ? 'Restore' : 'Maximize'}
      >
        {isMaximized ? (
          <svg width="10" height="10" viewBox="0 0 10 10" fill="none" stroke="currentColor" strokeWidth="1">
            <rect x="2.5" y="0.5" width="7" height="7" />
            <rect x="0.5" y="2.5" width="7" height="7" />
          </svg>
        ) : (
          <svg width="10" height="10" viewBox="0 0 10 10" fill="none" stroke="currentColor" strokeWidth="1">
            <rect x="0.5" y="0.5" width="9" height="9" />
          </svg>
        )}
      </button>
      <button
        onClick={close}
        className="inline-flex h-8 w-11 items-center justify-center text-muted-foreground hover:bg-destructive hover:text-white"
        aria-label="Close"
      >
        <svg width="10" height="10" viewBox="0 0 10 10" fill="none" stroke="currentColor" strokeWidth="1.2">
          <line x1="0" y1="0" x2="10" y2="10" />
          <line x1="10" y1="0" x2="0" y2="10" />
        </svg>
      </button>
    </div>
  );
}

export function useIsMac() {
  const [isMac, setIsMac] = useState(false);
  useEffect(() => { setIsMac(navigator.userAgent.includes('Mac')); }, []);
  return isMac;
}
