import { useEffect, useState } from 'react';
import { toast } from 'sonner';
import { Loader2, ShieldCheck, ShieldAlert } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { isTauri } from '@/lib/platform';
import {
  getCryptoStatus,
  enableMasterPassword,
  changeMasterPassword,
  disableMasterPassword,
  type CryptoMode,
} from '@/lib/crypto';
import { useI18n } from '@/i18n';

export function SecuritySettingsPage() {
  const { t } = useI18n();
  const [mode, setMode] = useState<CryptoMode | null>(null);
  const [loading, setLoading] = useState(true);

  const refresh = async () => {
    const status = await getCryptoStatus();
    setMode(status?.mode ?? null);
    setLoading(false);
  };

  useEffect(() => {
    let active = true;
    void (async () => {
      const status = await getCryptoStatus();
      if (!active) return;
      setMode(status?.mode ?? null);
      setLoading(false);
    })();
    return () => {
      active = false;
    };
  }, []);

  if (!isTauri()) {
    return (
      <SettingsShell t={t}>
        <Card>
          <CardContent className="py-6 text-sm text-muted-foreground">
            {t(
              'security.desktop_only',
              'Encryption settings are only available in the desktop app.',
            )}
          </CardContent>
        </Card>
      </SettingsShell>
    );
  }

  if (loading) {
    return (
      <SettingsShell t={t}>
        <div className="flex justify-center py-10">
          <Loader2 className="h-6 w-6 animate-spin text-muted-foreground" />
        </div>
      </SettingsShell>
    );
  }

  return (
    <SettingsShell t={t}>
      <StatusCard mode={mode} t={t} />
      {mode === 'password' ? (
        <>
          <ChangePasswordCard t={t} onDone={refresh} />
          <DisablePasswordCard t={t} onDone={refresh} />
        </>
      ) : (
        <EnablePasswordCard t={t} onDone={refresh} />
      )}
    </SettingsShell>
  );
}

type T = (key: string, fallback?: string) => string;

function SettingsShell({ t, children }: { t: T; children: React.ReactNode }) {
  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-lg font-semibold">{t('security.title', 'Security')}</h2>
        <p className="text-sm text-muted-foreground">
          {t(
            'security.subtitle',
            'Manage encryption at rest and your master password.',
          )}
        </p>
      </div>
      {children}
    </div>
  );
}

function StatusCard({ mode, t }: { mode: CryptoMode | null; t: T }) {
  const isPassword = mode === 'password';
  const Icon = isPassword ? ShieldCheck : ShieldAlert;
  const stateText = isPassword
    ? t('security.state_password', 'Protected by a master password')
    : t('security.state_keychain', 'Protected by your operating system keychain');
  return (
    <Card>
      <CardHeader className="flex flex-row items-center gap-3 pb-3">
        <div className="flex h-9 w-9 shrink-0 items-center justify-center rounded-lg bg-muted">
          <Icon className="h-5 w-5 text-muted-foreground" />
        </div>
        <div>
          <CardTitle className="text-base">
            {t('security.encryption_active', 'Encryption at rest is active')}
          </CardTitle>
          <p className="text-sm text-muted-foreground">{stateText}</p>
        </div>
      </CardHeader>
      <CardContent className="pt-0 text-sm text-muted-foreground">
        {t(
          'security.encryption_explainer',
          'Your database and uploaded files are encrypted with AES-256. A master password adds zero-knowledge protection but cannot be recovered if forgotten.',
        )}
      </CardContent>
    </Card>
  );
}

function EnablePasswordCard({ t, onDone }: { t: T; onDone: () => Promise<void> }) {
  const [pw, setPw] = useState('');
  const [confirm, setConfirm] = useState('');
  const [busy, setBusy] = useState(false);

  async function submit(e: React.FormEvent) {
    e.preventDefault();
    if (pw.length < 8) {
      toast.error(t('security.password_too_short', 'Use at least 8 characters.'));
      return;
    }
    if (pw !== confirm) {
      toast.error(t('security.passwords_mismatch', 'Passwords do not match.'));
      return;
    }
    setBusy(true);
    try {
      await enableMasterPassword(pw);
      toast.success(t('security.password_enabled', 'Master password enabled.'));
      setPw('');
      setConfirm('');
      await onDone();
    } catch (err) {
      toast.error(String(err));
    } finally {
      setBusy(false);
    }
  }

  return (
    <Card>
      <CardHeader className="pb-3">
        <CardTitle className="text-base">
          {t('security.enable_title', 'Set a master password')}
        </CardTitle>
        <p className="text-sm text-muted-foreground">
          {t(
            'security.enable_desc',
            'You will be asked for it each time the app starts. Keep it safe — it cannot be recovered.',
          )}
        </p>
      </CardHeader>
      <CardContent>
        <form onSubmit={submit} className="flex max-w-sm flex-col gap-3">
          <div className="space-y-1.5">
            <Label htmlFor="new-pw">{t('security.new_password', 'New password')}</Label>
            <Input id="new-pw" type="password" value={pw} onChange={(e) => setPw(e.target.value)} />
          </div>
          <div className="space-y-1.5">
            <Label htmlFor="confirm-pw">{t('security.confirm_password', 'Confirm password')}</Label>
            <Input
              id="confirm-pw"
              type="password"
              value={confirm}
              onChange={(e) => setConfirm(e.target.value)}
            />
          </div>
          <Button type="submit" disabled={busy} className="self-start">
            {busy && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
            {t('security.enable_button', 'Enable master password')}
          </Button>
        </form>
      </CardContent>
    </Card>
  );
}

function ChangePasswordCard({ t, onDone }: { t: T; onDone: () => Promise<void> }) {
  const [oldPw, setOldPw] = useState('');
  const [newPw, setNewPw] = useState('');
  const [busy, setBusy] = useState(false);

  async function submit(e: React.FormEvent) {
    e.preventDefault();
    if (newPw.length < 8) {
      toast.error(t('security.password_too_short', 'Use at least 8 characters.'));
      return;
    }
    setBusy(true);
    try {
      await changeMasterPassword(oldPw, newPw);
      toast.success(t('security.password_changed', 'Master password changed.'));
      setOldPw('');
      setNewPw('');
      await onDone();
    } catch (err) {
      toast.error(String(err));
    } finally {
      setBusy(false);
    }
  }

  return (
    <Card>
      <CardHeader className="pb-3">
        <CardTitle className="text-base">
          {t('security.change_title', 'Change master password')}
        </CardTitle>
      </CardHeader>
      <CardContent>
        <form onSubmit={submit} className="flex max-w-sm flex-col gap-3">
          <div className="space-y-1.5">
            <Label htmlFor="old-pw">{t('security.current_password', 'Current password')}</Label>
            <Input id="old-pw" type="password" value={oldPw} onChange={(e) => setOldPw(e.target.value)} />
          </div>
          <div className="space-y-1.5">
            <Label htmlFor="change-new-pw">{t('security.new_password', 'New password')}</Label>
            <Input
              id="change-new-pw"
              type="password"
              value={newPw}
              onChange={(e) => setNewPw(e.target.value)}
            />
          </div>
          <Button type="submit" disabled={busy} className="self-start">
            {busy && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
            {t('security.change_button', 'Change password')}
          </Button>
        </form>
      </CardContent>
    </Card>
  );
}

function DisablePasswordCard({ t, onDone }: { t: T; onDone: () => Promise<void> }) {
  const [pw, setPw] = useState('');
  const [busy, setBusy] = useState(false);

  async function submit(e: React.FormEvent) {
    e.preventDefault();
    setBusy(true);
    try {
      await disableMasterPassword(pw);
      toast.success(t('security.password_disabled', 'Master password removed.'));
      setPw('');
      await onDone();
    } catch (err) {
      toast.error(String(err));
    } finally {
      setBusy(false);
    }
  }

  return (
    <Card>
      <CardHeader className="pb-3">
        <CardTitle className="text-base">
          {t('security.disable_title', 'Remove master password')}
        </CardTitle>
        <p className="text-sm text-muted-foreground">
          {t(
            'security.disable_desc',
            'The database stays encrypted, but unlocks transparently via the OS keychain.',
          )}
        </p>
      </CardHeader>
      <CardContent>
        <form onSubmit={submit} className="flex max-w-sm flex-col gap-3">
          <div className="space-y-1.5">
            <Label htmlFor="disable-pw">{t('security.current_password', 'Current password')}</Label>
            <Input id="disable-pw" type="password" value={pw} onChange={(e) => setPw(e.target.value)} />
          </div>
          <Button type="submit" variant="destructive" disabled={busy} className="self-start">
            {busy && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
            {t('security.disable_button', 'Remove password')}
          </Button>
        </form>
      </CardContent>
    </Card>
  );
}
