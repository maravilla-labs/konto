import { useEffect, useState } from 'react';
import {
  Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle, DialogFooter,
} from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { RichTextEditor } from '@/components/ui/rich-text-editor';
import { Switch } from '@/components/ui/switch';
import {
  Select, SelectContent, SelectItem, SelectTrigger, SelectValue,
} from '@/components/ui/select';
import { Plus, Trash2 } from 'lucide-react';
import type {
  RecurringInvoice, CreateRecurringInvoice, UpdateRecurringInvoice,
  RecurringFrequency, RecurringInvoiceLineTemplate,
} from '@/types/recurring-invoice';
import { SUPPORTED_LANGUAGES } from '@/lib/language';
import { useI18n } from '@/i18n';

interface Props {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  editing: RecurringInvoice | null;
  onSave: (data: CreateRecurringInvoice | UpdateRecurringInvoice) => void;
  isPending: boolean;
  contacts: Array<{ id: string; name?: string; company?: string }>;
  projects: Array<{ id: string; name: string }>;
  accounts: Array<{ id: string; number: number; name: string }>;
}

const emptyLine = (): RecurringInvoiceLineTemplate => ({
  description: '', quantity: 1, unit_price: 0, account_id: '',
});

export function RecurringInvoiceDialog({
  open, onOpenChange, editing, onSave, isPending, contacts, projects, accounts,
}: Props) {
  const { t } = useI18n();
  const frequencies: { value: RecurringFrequency; label: string }[] = [
    { value: 'monthly', label: t('recurring.frequency.monthly', 'Monthly') },
    { value: 'quarterly', label: t('recurring.frequency.quarterly', 'Quarterly') },
    { value: 'semi_annual', label: t('recurring.frequency.semi_annual', 'Semi-Annual') },
    { value: 'annual', label: t('recurring.frequency.annual', 'Annual') },
    { value: 'custom', label: t('recurring.frequency.custom', 'Custom') },
  ];
  const [contactId, setContactId] = useState('');
  const [projectId, setProjectId] = useState('');
  const [frequency, setFrequency] = useState<RecurringFrequency>('monthly');
  const [intervalDays, setIntervalDays] = useState('');
  const [nextRunDate, setNextRunDate] = useState('');
  const [endDate, setEndDate] = useState('');
  const [autoSend, setAutoSend] = useState(false);
  const [isActive, setIsActive] = useState(true);
  const [notes, setNotes] = useState('');
  const [language, setLanguage] = useState('');
  const [paymentTerms, setPaymentTerms] = useState('');
  const [lines, setLines] = useState<RecurringInvoiceLineTemplate[]>([emptyLine()]);

  useEffect(() => {
    if (!open) return;
    if (editing) {
      setContactId(editing.contact_id);
      setProjectId(editing.project_id ?? '');
      setFrequency(editing.frequency);
      setIntervalDays(editing.interval_days?.toString() ?? '');
      setNextRunDate(editing.next_run_date);
      setEndDate(editing.end_date ?? '');
      setAutoSend(editing.auto_send);
      setIsActive(editing.is_active);
      setLanguage(editing.template_data.language ?? '');
      setNotes(editing.template_data.notes ?? '');
      setPaymentTerms(editing.template_data.payment_terms ?? '');
      setLines(
        editing.template_data.lines.length > 0
          ? editing.template_data.lines.map((l) => ({ ...l }))
          : [emptyLine()],
      );
    } else {
      setContactId('');
      setProjectId('');
      setFrequency('monthly');
      setIntervalDays('');
      setNextRunDate(new Date().toISOString().slice(0, 10));
      setEndDate('');
      setAutoSend(false);
      setIsActive(true);
      setLanguage('');
      setNotes('');
      setPaymentTerms('');
      setLines([emptyLine()]);
    }
  }, [open, editing]);

  function handleSubmit() {
    const data: CreateRecurringInvoice | UpdateRecurringInvoice = {
      contact_id: contactId,
      project_id: projectId || undefined,
      frequency,
      interval_days: frequency === 'custom' ? Number(intervalDays) || undefined : undefined,
      next_run_date: nextRunDate,
      end_date: endDate || undefined,
      auto_send: autoSend,
      language: language || undefined,
      notes: notes || undefined,
      payment_terms: paymentTerms || undefined,
      lines: lines.filter((l) => l.description && l.account_id),
      ...(editing ? { is_active: isActive } : {}),
    };
    onSave(data);
  }

  function updateLine(idx: number, partial: Partial<RecurringInvoiceLineTemplate>) {
    setLines((prev) => prev.map((l, i) => (i === idx ? { ...l, ...partial } : l)));
  }

  function removeLine(idx: number) {
    setLines((prev) => prev.filter((_, i) => i !== idx));
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-h-[90vh] max-w-2xl overflow-y-auto">
        <DialogHeader>
          <DialogTitle>
            {editing
              ? t('recurring.edit_dialog_title', 'Edit Recurring Invoice')
              : t('recurring.new_dialog_title', 'New Recurring Invoice')}
          </DialogTitle>
          <DialogDescription>
            {editing
              ? t('recurring.edit_dialog_description', 'Update recurring invoice settings and line items.')
              : t('recurring.new_dialog_description', 'Set up a new recurring invoice with schedule and line items.')}
          </DialogDescription>
        </DialogHeader>

        <div className="grid gap-4 py-4">
          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-2">
              <Label>{t('common.contact', 'Contact')}</Label>
              <Select value={contactId} onValueChange={setContactId}>
                <SelectTrigger><SelectValue placeholder={t('recurring.select_contact', 'Select contact')} /></SelectTrigger>
                <SelectContent>
                  {contacts.map((c) => (
                    <SelectItem key={c.id} value={c.id}>
                      {c.name ?? c.company ?? c.id}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
            <div className="space-y-2">
              <Label>{t('recurring.project_optional', 'Project (optional)')}</Label>
              <Select
                value={projectId || '__none__'}
                onValueChange={(v) => setProjectId(v === '__none__' ? '' : v)}
              >
                <SelectTrigger><SelectValue placeholder={t('common.none', 'None')} /></SelectTrigger>
                <SelectContent>
                  <SelectItem value="__none__">{t('common.none', 'None')}</SelectItem>
                  {projects.map((p) => (
                    <SelectItem key={p.id} value={p.id}>{p.name}</SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
          </div>

          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-2">
              <Label>{t('recurring.frequency', 'Frequency')}</Label>
              <Select
                value={frequency}
                onValueChange={(v) => setFrequency(v as RecurringFrequency)}
              >
                <SelectTrigger><SelectValue /></SelectTrigger>
                <SelectContent>
                  {frequencies.map((f) => (
                    <SelectItem key={f.value} value={f.value}>{f.label}</SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
            {frequency === 'custom' && (
              <div className="space-y-2">
                <Label>{t('recurring.interval_days', 'Interval (days)')}</Label>
                <Input
                  type="number"
                  value={intervalDays}
                  onChange={(e) => setIntervalDays(e.target.value)}
                  min={1}
                />
              </div>
            )}
          </div>

          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-2">
              <Label>{t('recurring.next_run', 'Next Run')}</Label>
              <Input type="date" value={nextRunDate} onChange={(e) => setNextRunDate(e.target.value)} />
            </div>
            <div className="space-y-2">
              <Label>{t('recurring.end_date_optional', 'End Date (optional)')}</Label>
              <Input type="date" value={endDate} onChange={(e) => setEndDate(e.target.value)} />
            </div>
          </div>

          <div className="flex items-center gap-6">
            <div className="flex items-center gap-2">
              <Switch checked={autoSend} onCheckedChange={setAutoSend} />
              <Label>{t('recurring.auto_send_on_generation', 'Auto-send on generation')}</Label>
            </div>
            {editing && (
              <div className="flex items-center gap-2">
                <Switch checked={isActive} onCheckedChange={setIsActive} />
                <Label>{t('recurring.active', 'Active')}</Label>
              </div>
            )}
          </div>

          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-2">
              <Label>{t('common.language', 'Language')}</Label>
              <Select
                value={language || '__auto__'}
                onValueChange={(v) => setLanguage(v === '__auto__' ? '' : v)}
              >
                <SelectTrigger><SelectValue placeholder={t('common.automatic', 'Automatic')} /></SelectTrigger>
                <SelectContent>
                  <SelectItem value="__auto__">{t('common.automatic', 'Automatic')}</SelectItem>
                  {SUPPORTED_LANGUAGES.map((lang) => (
                    <SelectItem key={lang.code} value={lang.code}>
                      {lang.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
            <div className="space-y-2">
              <Label>{t('invoice_form.payment_terms', 'Payment Terms')}</Label>
              <Input
                value={paymentTerms}
                onChange={(e) => setPaymentTerms(e.target.value)}
                placeholder={t('invoice_form.payment_terms_placeholder', 'e.g. Net 30')}
              />
            </div>
          </div>

          <LineItemsEditor
            lines={lines}
            accounts={accounts}
            onUpdate={updateLine}
            onRemove={removeLine}
            onAdd={() => setLines((prev) => [...prev, emptyLine()])}
          />

          <div className="space-y-2">
            <Label>{t('common.notes', 'Notes')}</Label>
            <RichTextEditor value={notes} onChange={(md) => setNotes(md)} />
          </div>
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={() => onOpenChange(false)}>{t('common.cancel', 'Cancel')}</Button>
          <Button onClick={handleSubmit} disabled={isPending || !contactId || lines.length === 0}>
            {editing ? t('common.save_changes', 'Save Changes') : t('common.create', 'Create')}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}

function LineItemsEditor({
  lines, accounts, onUpdate, onRemove, onAdd,
}: {
  lines: RecurringInvoiceLineTemplate[];
  accounts: Array<{ id: string; number: number; name: string }>;
  onUpdate: (idx: number, partial: Partial<RecurringInvoiceLineTemplate>) => void;
  onRemove: (idx: number) => void;
  onAdd: () => void;
}) {
  const { t } = useI18n();
  return (
    <div className="space-y-2">
      <div className="flex items-center justify-between">
        <Label>{t('invoice_form.line_items', 'Line Items')}</Label>
        <Button variant="outline" size="sm" onClick={onAdd}>
          <Plus className="mr-1 h-3 w-3" /> {t('invoice_form.add_line', 'Add Line')}
        </Button>
      </div>
      <div className="space-y-2">
        {lines.map((line, idx) => (
          <div key={idx} className="grid grid-cols-12 gap-2 items-end">
            <div className="col-span-4">
              {idx === 0 && <Label className="text-xs">{t('common.description', 'Description')}</Label>}
              <Input
                value={line.description}
                onChange={(e) => onUpdate(idx, { description: e.target.value })}
                placeholder={t('common.description', 'Description')}
              />
            </div>
            <div className="col-span-2">
              {idx === 0 && <Label className="text-xs">{t('invoice_form.qty_short', 'Qty')}</Label>}
              <Input
                type="number"
                value={line.quantity}
                onChange={(e) => onUpdate(idx, { quantity: Number(e.target.value) })}
                min={0}
                step="0.01"
              />
            </div>
            <div className="col-span-2">
              {idx === 0 && <Label className="text-xs">{t('invoice_form.unit_price', 'Unit Price')}</Label>}
              <Input
                type="number"
                value={line.unit_price}
                onChange={(e) => onUpdate(idx, { unit_price: Number(e.target.value) })}
                min={0}
                step="0.01"
              />
            </div>
            <div className="col-span-3">
              {idx === 0 && <Label className="text-xs">{t('common.account', 'Account')}</Label>}
              <Select
                value={line.account_id}
                onValueChange={(v) => onUpdate(idx, { account_id: v })}
              >
                <SelectTrigger className="text-xs">
                  <SelectValue placeholder={t('common.account', 'Account')} />
                </SelectTrigger>
                <SelectContent>
                  {accounts.map((a) => (
                    <SelectItem key={a.id} value={a.id}>
                      {a.number} {a.name}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
            <div className="col-span-1">
              <Button
                variant="ghost"
                size="icon"
                onClick={() => onRemove(idx)}
                disabled={lines.length <= 1}
              >
                <Trash2 className="h-4 w-4" />
              </Button>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
