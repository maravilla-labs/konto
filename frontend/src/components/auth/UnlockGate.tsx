import { useEffect, useState, type ReactNode } from 'react';
import { AlertTriangle, Check, Copy, Loader2, Lock } from 'lucide-react';
import { isTauri } from '@/lib/platform';
import { getCryptoStatus, unlockDatabase } from '@/lib/crypto';
import { useI18n } from '@/i18n';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';

type Phase = 'checking' | 'locked' | 'failed' | 'open';

/**
 * Boot-time gate for master-password mode. When the local database is encrypted
 * with a master password, the embedded server is not started until the user
 * unlocks it here. In web mode or transparent (keychain) mode it renders its
 * children immediately.
 *
 * It is also where a failed startup surfaces: the desktop shell records boot
 * failures instead of panicking (a panic in Tauri's `setup` aborts the process
 * outright on macOS), so this is the only place the user ever sees them.
 */
export function UnlockGate({ children }: { children: ReactNode }) {
  const { t } = useI18n();
  const [phase, setPhase] = useState<Phase>('checking');
  const [password, setPassword] = useState('');
  const [error, setError] = useState<string | null>(null);
  const [bootError, setBootError] = useState<string | null>(null);
  const [copied, setCopied] = useState(false);
  const [submitting, setSubmitting] = useState(false);

  useEffect(() => {
    let active = true;
    void (async () => {
      if (!isTauri()) {
        if (active) setPhase('open');
        return;
      }
      const status = await getCryptoStatus();
      if (!active) return;
      if (status?.bootError) {
        setBootError(status.bootError);
        setPhase('failed');
        return;
      }
      setPhase(status?.mode === 'password' && !status.unlocked ? 'locked' : 'open');
    })();
    return () => {
      active = false;
    };
  }, []);

  async function copyDetails() {
    if (!bootError) return;
    try {
      await navigator.clipboard.writeText(bootError);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch {
      // Clipboard unavailable; the text is on screen and selectable anyway.
    }
  }

  async function onSubmit(e: React.FormEvent) {
    e.preventDefault();
    if (!password || submitting) return;
    setSubmitting(true);
    setError(null);
    try {
      await unlockDatabase(password);
      // Reload so the API client picks up the now-running server port cleanly.
      window.location.reload();
    } catch {
      setError(t('security.unlock_failed', 'Incorrect master password'));
      setSubmitting(false);
      setPassword('');
    }
  }

  if (phase === 'checking') {
    return (
      <div className="flex h-screen items-center justify-center">
        <Loader2 className="h-8 w-8 animate-spin text-primary" />
      </div>
    );
  }

  if (phase === 'failed') {
    return (
      <div className="flex h-screen flex-col items-center justify-center gap-6 p-6">
        <div className="flex h-14 w-14 items-center justify-center rounded-2xl bg-destructive/10">
          <AlertTriangle className="h-7 w-7 text-destructive" />
        </div>
        <div className="max-w-md text-center">
          <h1 className="text-lg font-semibold">
            {t('boot.failed_title', 'Maravilla Konto could not start')}
          </h1>
          <p className="mt-1 text-sm text-muted-foreground">
            {t(
              'boot.failed_subtitle',
              'Your data has not been changed. Restart the app to try again, or send the details below to support.',
            )}
          </p>
        </div>
        <pre className="max-h-48 w-full max-w-xl overflow-auto rounded-lg bg-muted p-4 text-left text-xs whitespace-pre-wrap select-text">
          {bootError}
        </pre>
        <Button variant="outline" onClick={() => void copyDetails()}>
          {copied ? (
            <Check className="mr-2 h-4 w-4" />
          ) : (
            <Copy className="mr-2 h-4 w-4" />
          )}
          {copied
            ? t('boot.copied', 'Copied')
            : t('boot.copy_details', 'Copy details')}
        </Button>
      </div>
    );
  }

  if (phase === 'open') return <>{children}</>;

  return (
    <div className="flex h-screen flex-col items-center justify-center gap-6 p-6">
      <div className="flex h-14 w-14 items-center justify-center rounded-2xl bg-muted">
        <Lock className="h-7 w-7 text-muted-foreground" />
      </div>
      <div className="text-center">
        <h1 className="text-lg font-semibold">
          {t('security.unlock_title', 'Database locked')}
        </h1>
        <p className="mt-1 text-sm text-muted-foreground">
          {t('security.unlock_subtitle', 'Enter your master password to continue')}
        </p>
      </div>
      <form onSubmit={onSubmit} className="flex w-full max-w-xs flex-col gap-3">
        <Input
          type="password"
          autoFocus
          value={password}
          onChange={(e) => setPassword(e.target.value)}
          placeholder={t('security.master_password', 'Master password')}
          aria-label={t('security.master_password', 'Master password')}
        />
        {error && <p className="text-sm text-destructive">{error}</p>}
        <Button type="submit" disabled={submitting || !password}>
          {submitting && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
          {t('security.unlock_button', 'Unlock')}
        </Button>
      </form>
    </div>
  );
}
