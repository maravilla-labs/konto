import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import {
  Dialog, DialogContent, DialogHeader, DialogTitle,
} from '@/components/ui/dialog';
import {
  Select, SelectContent, SelectItem, SelectTrigger, SelectValue,
} from '@/components/ui/select';
import { formatAmount } from '@/lib/format';
import type { DunningLevel } from '@/types/dunning';
import { useI18n } from '@/i18n';

interface Account {
  id: string;
  number: number;
  name: string;
}

interface ReminderDialogProps {
  open: boolean;
  onOpenChange: (v: boolean) => void;
  levelId: string;
  onLevelChange: (v: string) => void;
  levels: DunningLevel[];
  onSend: () => void;
  isPending: boolean;
}

export function ReminderDialog({ open, onOpenChange, levelId, onLevelChange, levels, onSend, isPending }: ReminderDialogProps) {
  const { t } = useI18n();
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader><DialogTitle>{t('invoice_dialogs.reminder.title', 'Send Payment Reminder')}</DialogTitle></DialogHeader>
        <div className="space-y-4">
          <div>
            <Label>{t('invoice_dialogs.reminder.dunning_level', 'Dunning Level')}</Label>
            <Select value={levelId} onValueChange={onLevelChange}>
              <SelectTrigger><SelectValue placeholder={t('invoice_dialogs.reminder.select_level', 'Select level')} /></SelectTrigger>
              <SelectContent>
                {levels.filter((l) => l.is_active).map((l) => (
                  <SelectItem key={l.id} value={l.id}>
                    {t('invoice_dialogs.reminder.level', 'Level')} {l.level} - {l.days_after_due} {t('invoice_dialogs.days', 'days')}, CHF {parseFloat(l.fee_amount).toFixed(2)}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
          <Button onClick={onSend} className="w-full" disabled={isPending || !levelId}>
            {t('invoice_dialogs.reminder.send', 'Send Reminder')}
          </Button>
        </div>
      </DialogContent>
    </Dialog>
  );
}

interface MarkPaidDialogProps {
  open: boolean;
  onOpenChange: (v: boolean) => void;
  date: string;
  onDateChange: (v: string) => void;
  accountId: string;
  onAccountChange: (v: string) => void;
  accounts: Account[];
  onConfirm: () => void;
  isPending: boolean;
}

export function MarkPaidDialog({ open, onOpenChange, date, onDateChange, accountId, onAccountChange, accounts, onConfirm, isPending }: MarkPaidDialogProps) {
  const { t } = useI18n();
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader><DialogTitle>{t('invoice_dialogs.mark_paid.title', 'Mark Invoice as Paid')}</DialogTitle></DialogHeader>
        <div className="space-y-4">
          <div>
            <Label>{t('invoice_dialogs.payment_date', 'Payment Date')}</Label>
            <Input type="date" value={date} onChange={(e) => onDateChange(e.target.value)} />
          </div>
          <div>
            <Label>{t('invoice_dialogs.payment_account', 'Payment Account')}</Label>
            <Select value={accountId} onValueChange={onAccountChange}>
              <SelectTrigger><SelectValue placeholder={t('invoice_dialogs.select_bank_cash_account', 'Select bank/cash account')} /></SelectTrigger>
              <SelectContent>
                {accounts.map((a) => (
                  <SelectItem key={a.id} value={a.id}>{a.number} — {a.name}</SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
          <Button onClick={onConfirm} className="w-full" disabled={isPending || !accountId}>
            {t('invoice_dialogs.mark_paid.confirm', 'Confirm Payment')}
          </Button>
        </div>
      </DialogContent>
    </Dialog>
  );
}

interface RecordPaymentForm {
  amount: string;
  payment_date: string;
  payment_account_id: string;
  payment_method: string;
  reference: string;
}

interface RecordPaymentDialogProps {
  open: boolean;
  onOpenChange: (v: boolean) => void;
  form: RecordPaymentForm;
  onFormChange: (f: RecordPaymentForm) => void;
  accounts: Account[];
  invoiceTotal: string;
  remaining: string;
  onRecord: () => void;
  isPending: boolean;
}

export function RecordPaymentDialog({ open, onOpenChange, form, onFormChange, accounts, invoiceTotal, remaining, onRecord, isPending }: RecordPaymentDialogProps) {
  const { t } = useI18n();
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader><DialogTitle>{t('invoice_dialogs.record.title', 'Record Payment')}</DialogTitle></DialogHeader>
        <div className="space-y-4">
          <p className="text-sm text-muted-foreground">
            {t('invoice_dialogs.invoice_total', 'Invoice total')}: {formatAmount(invoiceTotal)} - {t('invoice_dialogs.remaining', 'Remaining')}: {formatAmount(remaining)}
          </p>
          <div>
            <Label>{t('invoice_dialogs.amount_chf', 'Amount (CHF)')}</Label>
            <Input type="number" step="0.01" value={form.amount} onChange={(e) => onFormChange({ ...form, amount: e.target.value })} placeholder="0.00" />
          </div>
          <div>
            <Label>{t('invoice_dialogs.payment_date', 'Payment Date')}</Label>
            <Input type="date" value={form.payment_date} onChange={(e) => onFormChange({ ...form, payment_date: e.target.value })} />
          </div>
          <div>
            <Label>{t('invoice_dialogs.payment_account', 'Payment Account')}</Label>
            <Select value={form.payment_account_id} onValueChange={(v) => onFormChange({ ...form, payment_account_id: v })}>
              <SelectTrigger><SelectValue placeholder={t('invoice_dialogs.select_bank_cash_account', 'Select bank/cash account')} /></SelectTrigger>
              <SelectContent>
                {accounts.map((a) => (
                  <SelectItem key={a.id} value={a.id}>{a.number} — {a.name}</SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
          <div>
            <Label>{t('invoice_dialogs.payment_method_optional', 'Payment Method (optional)')}</Label>
            <Input value={form.payment_method} onChange={(e) => onFormChange({ ...form, payment_method: e.target.value })} placeholder={t('invoice_dialogs.payment_method_placeholder', 'e.g. Bank Transfer, Cash')} />
          </div>
          <div>
            <Label>{t('invoice_dialogs.reference_optional', 'Reference (optional)')}</Label>
            <Input value={form.reference} onChange={(e) => onFormChange({ ...form, reference: e.target.value })} placeholder={t('invoice_dialogs.reference_placeholder', 'e.g. Transaction ID')} />
          </div>
          <Button onClick={onRecord} className="w-full" disabled={isPending || !form.amount || !form.payment_account_id}>
            {t('invoice_dialogs.record.action', 'Record Payment')}
          </Button>
        </div>
      </DialogContent>
    </Dialog>
  );
}
