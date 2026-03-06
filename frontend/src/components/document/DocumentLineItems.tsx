import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Plus, Trash2 } from 'lucide-react';
import type { CreateDocumentLine } from '@/types/document';
import { useI18n } from '@/i18n';

export interface DocLineFormData {
  description: string;
  quantity: string;
  unit: string;
  unit_price: string;
  discount_pct: string;
}

export function emptyDocLine(): DocLineFormData {
  return { description: '', quantity: '1', unit: '', unit_price: '', discount_pct: '0' };
}

export function toCreateDocLines(lines: DocLineFormData[]): CreateDocumentLine[] {
  return lines.map((l) => ({
    description: l.description,
    quantity: l.quantity || '0',
    unit: l.unit || undefined,
    unit_price: l.unit_price || '0',
    discount_pct: l.discount_pct || '0',
  }));
}

function calcLineTotal(line: DocLineFormData): number {
  const qty = parseFloat(line.quantity || '0');
  const price = parseFloat(line.unit_price || '0');
  const discount = parseFloat(line.discount_pct || '0');
  return qty * price * (1 - discount / 100);
}

interface DocumentLineItemsProps {
  lines: DocLineFormData[];
  onChange: (lines: DocLineFormData[]) => void;
}

export function DocumentLineItems({ lines, onChange }: DocumentLineItemsProps) {
  const { t } = useI18n();

  function updateLine(index: number, field: keyof DocLineFormData, value: string) {
    const next = [...lines];
    next[index] = { ...next[index], [field]: value };
    onChange(next);
  }

  function addLine() {
    onChange([...lines, emptyDocLine()]);
  }

  function removeLine(index: number) {
    if (lines.length <= 1) return;
    onChange(lines.filter((_, i) => i !== index));
  }

  const lineSubtotals = lines.map(calcLineTotal);
  const subtotal = lineSubtotals.reduce((a, b) => a + b, 0);

  return (
    <div>
      <div className="mb-2 flex items-center justify-between">
        <Label>{t('invoice_form.line_items', 'Line Items')}</Label>
        <Button type="button" variant="outline" size="sm" onClick={addLine}>
          <Plus className="mr-1 h-3 w-3" /> {t('invoice_form.add_line', 'Add Line')}
        </Button>
      </div>
      <div className="space-y-3">
        {lines.map((line, i) => (
          <div key={i} className="rounded-md border p-3">
            <div className="grid gap-2 sm:grid-cols-[1fr_60px_60px_90px_70px_auto]">
              <Input
                placeholder={t('common.description', 'Description')}
                value={line.description}
                onChange={(e) => updateLine(i, 'description', e.target.value)}
              />
              <Input
                type="number"
                placeholder={t('invoice_form.qty_short', 'Qty')}
                value={line.quantity}
                onChange={(e) => updateLine(i, 'quantity', e.target.value)}
                step="0.01"
              />
              <Input
                placeholder={t('documents.line.unit', 'Unit')}
                value={line.unit}
                onChange={(e) => updateLine(i, 'unit', e.target.value)}
              />
              <Input
                type="number"
                placeholder={t('documents.line.price', 'Price')}
                value={line.unit_price}
                onChange={(e) => updateLine(i, 'unit_price', e.target.value)}
                step="0.01"
              />
              <Input
                type="number"
                placeholder={t('documents.line.discount_pct', 'Disc %')}
                value={line.discount_pct}
                onChange={(e) => updateLine(i, 'discount_pct', e.target.value)}
                step="0.1"
              />
              <div className="flex items-center gap-2">
                <span className="font-mono text-sm">{lineSubtotals[i].toFixed(2)}</span>
                {lines.length > 1 && (
                  <Button
                    type="button"
                    variant="ghost"
                    size="icon"
                    className="h-8 w-8"
                    onClick={() => removeLine(i)}
                  >
                    <Trash2 className="h-3.5 w-3.5" />
                  </Button>
                )}
              </div>
            </div>
          </div>
        ))}
      </div>
      <div className="mt-3 text-right">
        <span className="text-sm text-muted-foreground">{t('invoice_form.subtotal', 'Subtotal')}: </span>
        <span className="font-mono font-semibold">{subtotal.toFixed(2)}</span>
      </div>
    </div>
  );
}
