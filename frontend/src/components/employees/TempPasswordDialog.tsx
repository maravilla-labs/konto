import { useState } from 'react';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import { Copy, Check } from 'lucide-react';
import { useI18n } from '@/i18n';

interface TempPasswordDialogProps {
  open: boolean;
  password: string;
  onConfirm: () => void;
}

export function TempPasswordDialog({ open, password, onConfirm }: TempPasswordDialogProps) {
  const [copied, setCopied] = useState(false);
  const [confirmed, setConfirmed] = useState(false);
  const { t } = useI18n();

  function handleCopy() {
    navigator.clipboard.writeText(password);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  }

  return (
    <Dialog open={open} onOpenChange={() => {}}>
      <DialogContent onInteractOutside={(e) => e.preventDefault()}>
        <DialogHeader>
          <DialogTitle>{t('employees.temp_password_title', 'Temporary Password')}</DialogTitle>
          <DialogDescription>
            {t('employees.temp_password_notice', 'This password will only be shown once. Please save it securely and share it with the employee.')}
          </DialogDescription>
        </DialogHeader>
        <div className="my-4">
          <div className="flex items-center gap-2">
            <code className="flex-1 rounded bg-muted px-3 py-2 font-mono text-sm select-all">
              {password}
            </code>
            <Button variant="outline" size="icon" onClick={handleCopy}>
              {copied ? <Check className="h-4 w-4" /> : <Copy className="h-4 w-4" />}
            </Button>
          </div>
        </div>
        <div className="flex items-center gap-2">
          <input
            type="checkbox"
            id="pwd-confirm"
            checked={confirmed}
            onChange={(e) => setConfirmed(e.target.checked)}
            className="rounded border-input"
          />
          <label htmlFor="pwd-confirm" className="text-sm">
            {t('employees.temp_password_confirm', 'I have saved this password')}
          </label>
        </div>
        <DialogFooter>
          <Button onClick={onConfirm} disabled={!confirmed}>
            {t('common.close', 'Close')}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
