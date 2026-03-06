import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Plus, Trash2 } from 'lucide-react';
import { useI18n } from '@/i18n';

export interface FormLine {
  account_id: string;
  debit: number;
  credit: number;
}

interface JournalCreateFormProps {
  form: { date: string; description: string; reference: string };
  setForm: (f: { date: string; description: string; reference: string }) => void;
  lines: FormLine[];
  addLine: () => void;
  removeLine: (i: number) => void;
  updateLine: (i: number, field: keyof FormLine, value: string | number) => void;
  totalDebit: number;
  totalCredit: number;
  onSubmit: () => void;
  isPending: boolean;
}

export function JournalCreateForm({
  form,
  setForm,
  lines,
  addLine,
  removeLine,
  updateLine,
  totalDebit,
  totalCredit,
  onSubmit,
  isPending,
}: JournalCreateFormProps) {
  const { t } = useI18n();
  return (
    <div className="space-y-4">
      <div className="grid grid-cols-3 gap-4">
        <div>
          <Label>{t('common.date', 'Date')}</Label>
          <Input
            type="date"
            value={form.date}
            onChange={(e) => setForm({ ...form, date: e.target.value })}
          />
        </div>
        <div>
          <Label>{t('journal.reference', 'Reference')}</Label>
          <Input
            value={form.reference}
            onChange={(e) => setForm({ ...form, reference: e.target.value })}
            placeholder="REF-001"
          />
        </div>
        <div>
          <Label>{t('common.description', 'Description')}</Label>
          <Input
            value={form.description}
            onChange={(e) => setForm({ ...form, description: e.target.value })}
            placeholder={t('common.description', 'Description')}
          />
        </div>
      </div>
      <div>
        <div className="mb-2 flex items-center justify-between">
          <Label>{t('journal.lines', 'Lines')}</Label>
          <Button type="button" variant="outline" size="sm" onClick={addLine}>
            <Plus className="mr-1 h-3 w-3" /> {t('journal.add_line', 'Add Line')}
          </Button>
        </div>
        <div className="space-y-2">
          {lines.map((line, i) => (
            <div key={i} className="flex gap-2">
              <Input
                placeholder={t('journal.account_placeholder', 'Account ID')}
                value={line.account_id}
                onChange={(e) => updateLine(i, 'account_id', e.target.value)}
                className="flex-1"
              />
              <Input
                type="number"
                placeholder={t('journal.debit', 'Debit')}
                value={line.debit || ''}
                onChange={(e) => updateLine(i, 'debit', parseFloat(e.target.value) || 0)}
                className="w-28"
              />
              <Input
                type="number"
                placeholder={t('journal.credit', 'Credit')}
                value={line.credit || ''}
                onChange={(e) => updateLine(i, 'credit', parseFloat(e.target.value) || 0)}
                className="w-28"
              />
              <Button
                type="button"
                variant="ghost"
                size="icon"
                onClick={() => removeLine(i)}
                disabled={lines.length <= 2}
              >
                <Trash2 className="h-4 w-4" />
              </Button>
            </div>
          ))}
        </div>
        <div className="mt-2 flex justify-end gap-4 text-sm">
          <span>{t('journal.debit', 'Debit')}: {totalDebit.toFixed(2)}</span>
          <span>{t('journal.credit', 'Credit')}: {totalCredit.toFixed(2)}</span>
          {totalDebit !== totalCredit && (
            <span className="text-destructive">{t('journal.unbalanced', 'Unbalanced!')}</span>
          )}
        </div>
      </div>
      <Button
        onClick={onSubmit}
        className="w-full"
        disabled={isPending || totalDebit !== totalCredit}
      >
        {t('journal.create_entry', 'Create Entry')}
      </Button>
    </div>
  );
}
