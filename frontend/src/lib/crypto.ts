/**
 * Encryption-at-rest IPC wrappers (Tauri desktop only).
 *
 * The database and uploads are encrypted at rest with a key resolved from the
 * OS keychain or, optionally, an Argon2id master password. These helpers drive
 * the boot-time unlock gate and the security settings page. In web mode they
 * are inert (no local key store exists).
 */
import { isTauri } from './platform';

export type CryptoMode = 'env' | 'keychain' | 'password' | 'uninitialized';

export interface CryptoStatus {
  mode: CryptoMode;
  unlocked: boolean;
  port: number;
  /**
   * Set when the desktop app failed to start (most often a failed database
   * migration). The boot gate shows this instead of letting the user into a
   * session with no backend behind it.
   */
  bootError: string | null;
}

async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<T>(cmd, args);
}

/** Current encryption mode/unlock state, or `null` in web mode. */
export async function getCryptoStatus(): Promise<CryptoStatus | null> {
  if (!isTauri()) return null;
  try {
    return await invoke<CryptoStatus>('crypto_status');
  } catch {
    return null;
  }
}

/** Unlock the database with the master password; resolves to the server port. */
export async function unlockDatabase(password: string): Promise<number> {
  return invoke<number>('crypto_unlock', { password });
}

/** Turn on master-password protection (from transparent keychain mode). */
export async function enableMasterPassword(password: string): Promise<void> {
  await invoke('crypto_enable_password', { password });
}

/** Change the master password. */
export async function changeMasterPassword(
  oldPassword: string,
  newPassword: string,
): Promise<void> {
  await invoke('crypto_change_password', { oldPassword, newPassword });
}

/** Turn off master-password protection (back to transparent keychain mode). */
export async function disableMasterPassword(password: string): Promise<void> {
  await invoke('crypto_disable_password', { password });
}
