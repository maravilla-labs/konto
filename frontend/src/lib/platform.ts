/**
 * Platform detection utilities for Tauri desktop vs web browser mode.
 */

let cachedPort: number | null = null;

/** Check if running inside a Tauri desktop app. */
export function isTauri(): boolean {
  return '__TAURI_INTERNALS__' in window;
}

/** Get the API base URL depending on platform. */
export async function getApiBaseUrl(): Promise<string> {
  if (!isTauri()) {
    return '/api/v1';
  }

  // In Tauri mode, get the server port from the IPC command
  if (cachedPort === null) {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      cachedPort = await invoke<number>('get_server_port');
    } catch {
      // Fallback if IPC fails
      cachedPort = 3000;
    }
  }

  return `http://127.0.0.1:${cachedPort}/api/v1`;
}

/** Get the uploads base URL depending on platform. */
export async function getUploadsBaseUrl(): Promise<string> {
  if (!isTauri()) {
    return '/uploads';
  }

  if (cachedPort === null) {
    await getApiBaseUrl(); // populates cachedPort
  }

  return `http://127.0.0.1:${cachedPort}/uploads`;
}

/**
 * Resolve an upload path (e.g. "/uploads/logo.png") to a full URL.
 * In web mode, returns the path as-is (Vite proxy handles it).
 * In Tauri mode, prepends the backend's dynamic host:port.
 * Safe to call synchronously after app init (cachedPort is set by ensureBaseUrl).
 */
export function resolveUploadUrl(path: string | null | undefined): string | null {
  if (!path) return null;
  // Normalize: ensure leading slash
  const normalized = path.startsWith('/') ? path : `/${path}`;
  if (!isTauri() || cachedPort === null) {
    return normalized;
  }
  // In Tauri mode, point directly to the embedded backend server
  return `http://127.0.0.1:${cachedPort}${normalized}`;
}
