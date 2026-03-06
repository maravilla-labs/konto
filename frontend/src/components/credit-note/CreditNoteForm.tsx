import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { RichTextEditor } from '@/components/ui/rich-text-editor';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { useAccounts, useContacts, useInvoices } from '@/hooks/useApi';
import { Plus, Trash2 } from 'lucide-react';
import type { CreateCreditNoteLine } from '@/types/credit-note';
import { useI18n } from '@/i18n';

export interface CreditNoteFormData {
  contact_id: string;
  invoice_id: string;
  issue_date: string;
  notes: string;
  lines: LineFormData[];
}

export interface LineFormData {
  description: string;
  quantity: string;
  unit_price: string;
  vat_rate_id: string;
  account_id: string;
}

interface CreditNoteFormProps {
  form: CreditNoteFormData;
  setForm: React.Dispatch<React.SetStateAction<CreditNoteFormData>>;
  onSubmit: () => void;
  isPending: boolean;
  submitLabel: string;
  secondaryAction?: { label: string; onClick: () => void; isPending?: boolean };
}

export function emptyLine(): LineFormData {
  return { description: '', quantity: '1', unit_price: '', vat_rate_id: '', account_id: '' };
}

export function toCreateLines(lines: LineFormData[]): CreateCreditNoteLine[] {
  return lines.map((l) => ({
    description: l.description,
    quantity: parseFloat(l.quantity || '0'),
    unit_price: parseFloat(l.unit_price || '0'),
    vat_rate_id: l.vat_rate_id || undefined,
    account_id: l.account_id,
  }));
}

export function CreditNoteForm({
  form,
  setForm,
  onSubmit,
  isPending,
  submitLabel,
  secondaryAction,
}: CreditNoteFormProps) {
  const { t } = useI18n();
  const { data: contactsData } = useContacts({ per_page: 200 });
  const { data: accountsData } = useAccounts({ per_page: 500 });
  const { data: invoicesData } = useInvoices({ per_page: 200, status: 'sent' });
  const contacts = contactsData?.data ?? [];
  const accounts = (accountsData?.data ?? []).filter(
    (a) => a.account_type === 'revenue',
  );
  const invoices = invoicesData?.data ?? [];

  function updateLine(index: number, field: keyof LineFormData, value: string) {
    const next = [...form.lines];
    next[index] = { ...next[index], [field]: value };
    setForm({ ...form, lines: next });
  }

  function addLine() {
    setForm({ ...form, lines: [...form.lines, emptyLine()] });
  }

  function removeLine(index: number) {
    if (form.lines.length <= 1) return;
    setForm({ ...form, lines: form.lines.filter((_, i) => i !== index) });
  }

  const lineSubtotals = form.lines.map(
    (l) => parseFloat(l.quantity || '0') * parseFloat(l.unit_price || '0'),
  );
  const subtotal = lineSubtotals.reduce((a, b) => a + b, 0);

  return (
    <div className="space-y-6">
      <div className="grid gap-4 sm:grid-cols-2">
        <div>
          <Label>{t('common.contact', 'Contact')}</Label>
          <Select
            value={form.contact_id}
            onValueChange={(v) => setForm({ ...form, contact_id: v })}
          >
            <SelectTrigger><SelectValue placeholder={t('common.contact', 'Contact')} /></SelectTrigger>
            <SelectContent>
              {contacts.map((c) => (
                <SelectItem key={c.id} value={c.id}>
                  {c.name1}{c.name2 ? ` (${c.name2})` : ''}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>
        <div>
          <Label>{t('credit_notes.from_invoice_optional', 'From Invoice (optional)')}</Label>
          <Select
            value={form.invoice_id}
            onValueChange={(v) => setForm({ ...form, invoice_id: v })}
          >
            <SelectTrigger><SelectValue placeholder={t('common.none', 'None')} /></SelectTrigger>
            <SelectContent>
              {invoices.map((inv) => (
                <SelectItem key={inv.id} value={inv.id}>
                  {inv.invoice_number ?? 'DRAFT'} — {inv.total}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>
      </div>

      <div>
        <Label>{t('common.issue_date', 'Issue Date')}</Label>
        <Input
          type="date"
          value={form.issue_date}
          onChange={(e) => setForm({ ...form, issue_date: e.target.value })}
        />
      </div>

      <div>
        <div className="mb-2 flex items-center justify-between">
          <Label>{t('invoice_form.line_items', 'Line Items')}</Label>
          <Button type="button" variant="outline" size="sm" onClick={addLine}>
            <Plus className="mr-1 h-3 w-3" /> {t('invoice_form.add_line', 'Add Line')}
          </Button>
        </div>
        <div className="space-y-3">
          {form.lines.map((line, i) => (
            <LineItemRow
              key={i}
              line={line}
              index={i}
              accounts={accounts}
              onUpdate={updateLine}
              onRemove={removeLine}
              canRemove={form.lines.length > 1}
              lineTotal={lineSubtotals[i]}
            />
          ))}
        </div>
        <div className="mt-3 text-right">
          <span className="text-sm text-muted-foreground">{t('invoice_form.subtotal', 'Subtotal')}: </span>
          <span className="font-mono font-semibold">{subtotal.toFixed(2)}</span>
        </div>
      </div>

      <div>
        <Label>{t('common.notes', 'Notes')}</Label>
        <RichTextEditor
          value={form.notes}
          onChange={(md) => setForm({ ...form, notes: md })}
          placeholder={t('invoice_form.optional_notes', 'Optional notes')}
        />
      </div>

      <div className="flex gap-2">
        <Button onClick={onSubmit} disabled={isPending || !form.contact_id || form.lines.length === 0}>
          {submitLabel}
        </Button>
        {secondaryAction && (
          <Button
            variant="outline"
            onClick={secondaryAction.onClick}
            disabled={secondaryAction.isPending || !form.contact_id}
          >
            {secondaryAction.label}
          </Button>
        )}
      </div>
    </div>
  );
}

function LineItemRow({
  line,
  index,
  accounts,
  onUpdate,
  onRemove,
  canRemove,
  lineTotal,
}: {
  line: LineFormData;
  index: number;
  accounts: { id: string; number: number; name: string }[];
  onUpdate: (i: number, field: keyof LineFormData, value: string) => void;
  onRemove: (i: number) => void;
  canRemove: boolean;
  lineTotal: number;
}) {
  const { t } = useI18n();
  return (
    <div className="rounded-md border p-3">
      <div className="grid gap-2 sm:grid-cols-[1fr_80px_100px_120px_auto]">
        <div>
          <Input
            placeholder={t('common.description', 'Description')}
            value={line.description}
            onChange={(e) => onUpdate(index, 'description', e.target.value)}
          />
        </div>
        <div>
          <Input
            type="number"
            placeholder={t('invoice_form.qty_short', 'Qty')}
            value={line.quantity}
            onChange={(e) => onUpdate(index, 'quantity', e.target.value)}
            step="0.01"
          />
        </div>
        <div>
          <Input
            type="number"
            placeholder={t('invoice_form.unit_price', 'Unit Price')}
            value={line.unit_price}
            onChange={(e) => onUpdate(index, 'unit_price', e.target.value)}
            step="0.01"
          />
        </div>
        <div>
          <Select
            value={line.account_id}
            onValueChange={(v) => onUpdate(index, 'account_id', v)}
          >
            <SelectTrigger><SelectValue placeholder={t('common.account', 'Account')} /></SelectTrigger>
            <SelectContent>
              {accounts.map((a) => (
                <SelectItem key={a.id} value={a.id}>{a.number} {a.name}</SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>
        <div className="flex items-center gap-2">
          <span className="font-mono text-sm">{lineTotal.toFixed(2)}</span>
          {canRemove && (
            <Button
              type="button"
              variant="ghost"
              size="icon"
              className="h-8 w-8"
              onClick={() => onRemove(index)}
            >
              <Trash2 className="h-3.5 w-3.5" />
            </Button>
          )}
        </div>
      </div>
    </div>
  );
}
