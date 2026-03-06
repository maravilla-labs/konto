/**
 * Platform-abstracted helpers for native desktop features.
 * Falls back to browser APIs when not running in Tauri.
 */
import { isTauri } from './platform';

/** Save a file with native Save As dialog (desktop) or browser download (web). */
export async function saveFile(data: Blob, filename: string): Promise<void> {
  if (isTauri()) {
    try {
      const { save } = await import('@tauri-apps/plugin-dialog');
      const path = await save({
        defaultPath: filename,
        filters: [{ name: 'All Files', extensions: ['*'] }],
      });
      if (path) {
        const { writeFile } = await import('@tauri-apps/plugin-fs');
        const buffer = await data.arrayBuffer();
        await writeFile(path, new Uint8Array(buffer));
      }
      return;
    } catch {
      // Fall through to browser method
    }
  }

  // Browser download fallback
  const url = URL.createObjectURL(data);
  const a = document.createElement('a');
  a.href = url;
  a.download = filename;
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
  URL.revokeObjectURL(url);
}

/** Open a file with native Open dialog (desktop) or file input (web). */
export async function openFile(
  accept?: string,
  filters?: Array<{ name: string; extensions: string[] }>,
): Promise<File | null> {
  if (isTauri()) {
    try {
      const { open } = await import('@tauri-apps/plugin-dialog');
      const path = await open({
        multiple: false,
        filters: filters ?? [{ name: 'All Files', extensions: ['*'] }],
      });
      if (path) {
        const { readFile } = await import('@tauri-apps/plugin-fs');
        const data = await readFile(path as string);
        const name = (path as string).split('/').pop() ?? 'file';
        return new File([data], name);
      }
      return null;
    } catch {
      // Fall through to browser method
    }
  }

  // Browser file input fallback
  return new Promise((resolve) => {
    const input = document.createElement('input');
    input.type = 'file';
    if (accept) input.accept = accept;
    input.onchange = () => {
      resolve(input.files?.[0] ?? null);
    };
    input.click();
  });
}

/** Show an OS notification (desktop) or browser notification (web). */
export async function showNotification(title: string, body: string): Promise<void> {
  if (isTauri()) {
    try {
      const { sendNotification } = await import('@tauri-apps/plugin-notification');
      sendNotification({ title, body });
      return;
    } catch {
      // Fall through to browser method
    }
  }

  // Browser notification fallback
  if ('Notification' in window && Notification.permission === 'granted') {
    new Notification(title, { body });
  } else if ('Notification' in window && Notification.permission !== 'denied') {
    const permission = await Notification.requestPermission();
    if (permission === 'granted') {
      new Notification(title, { body });
    }
  }
}

/** Reset the database (Tauri only). Deletes the DB file. */
export async function resetDatabase(): Promise<void> {
  if (!isTauri()) return;
  const { invoke } = await import('@tauri-apps/api/core');
  await invoke('reset_database');
}

/**
 * macOS transparent + undecorated windows can lose pointer event delivery
 * after layout changes. A 1px resize bounce forces the WebView to
 * recalculate its content frame and restore hit-testing.
 */
export async function refreshHitTest(): Promise<void> {
  if (!isTauri()) return;
  try {
    const { getCurrentWindow } = await import('@tauri-apps/api/window');
    const { PhysicalSize } = await import('@tauri-apps/api/dpi');
    const win = getCurrentWindow();
    const size = await win.innerSize();
    await win.setSize(new PhysicalSize(size.width + 1, size.height));
    await new Promise((r) => setTimeout(r, 50));
    await win.setSize(new PhysicalSize(size.width, size.height));
  } catch {
    /* ignore if APIs unavailable */
  }
}

/** Open a PDF in a new native Tauri window. */
export async function openPdfWindow(invoiceId: string, title: string): Promise<void> {
  if (!isTauri()) return;

  const { WebviewWindow } = await import('@tauri-apps/api/webviewWindow');

  const label = `pdf-${Date.now()}`;
  new WebviewWindow(label, {
    url: `/pdf-view/${invoiceId}`,
    title: `Preview — ${title}`,
    width: 800,
    height: 1000,
    center: true,
    decorations: true,
    transparent: false,
  });
}

/** Copy text to clipboard. */
export async function copyToClipboard(text: string): Promise<void> {
  if (isTauri()) {
    try {
      const { writeText } = await import('@tauri-apps/plugin-clipboard-manager');
      await writeText(text);
      return;
    } catch {
      // Fall through to browser method
    }
  }

  await navigator.clipboard.writeText(text);
}
