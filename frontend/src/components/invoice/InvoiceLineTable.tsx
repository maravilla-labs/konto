import { useState } from 'react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { RichTextEditor } from '@/components/ui/rich-text-editor';
import { Badge } from '@/components/ui/badge';
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from '@/components/ui/card';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import { Separator } from '@/components/ui/separator';
import { Plus, Trash2, Settings } from 'lucide-react';
import { useI18n } from '@/i18n';
import type { LineFormData } from './InvoiceFormTypes';
import { computeLineTotals } from './InvoiceFormTypes';
import type { VatRate } from '@/types/vat-rate';

interface InvoiceLineTableProps {
  lines: LineFormData[];
  defaultVatRateId: string;
  defaultAccountId: string;
  accounts: { id: string; number: number; name: string }[];
  vatRates: VatRate[];
  onUpdateLine: (index: number, field: keyof LineFormData, value: string) => void;
  onAddLine: () => void;
  onRemoveLine: (index: number) => void;
}

export function InvoiceLineTable({
  lines,
  defaultVatRateId,
  defaultAccountId,
  accounts,
  vatRates,
  onUpdateLine,
  onAddLine,
  onRemoveLine,
}: InvoiceLineTableProps) {
  const { t } = useI18n();
  const activeVatRates = vatRates.filter((v) => v.is_active);
  const totals = computeLineTotals(lines, vatRates, defaultVatRateId);

  return (
    <Card>
      <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-4">
        <CardTitle className="text-base">{t('invoice_form.line_items', 'Line Items')}</CardTitle>
        <Button type="button" variant="outline" size="sm" onClick={onAddLine}>
          <Plus className="mr-1 h-3 w-3" /> {t('invoice_form.add_line', 'Add Line')}
        </Button>
      </CardHeader>
      <CardContent className="p-0">
        {/* Desktop table */}
        <div className="hidden sm:block">
          <Table>
            <TableHeader>
              <TableRow className="hover:bg-transparent">
                <TableHead className="w-[45%]">{t('common.description', 'Description')}</TableHead>
                <TableHead className="w-[8%] text-right">{t('invoice_form.qty_short', 'Qty')}</TableHead>
                <TableHead className="w-[14%] text-right">{t('invoice_form.unit_price', 'Price')}</TableHead>
                <TableHead className="w-[8%] text-right">{t('invoice_form.discount', 'Disc.%')}</TableHead>
                <TableHead className="w-[14%] text-right">{t('common.total', 'Total')}</TableHead>
                <TableHead className="w-8" />
              </TableRow>
            </TableHeader>
            <TableBody>
              {lines.map((line, i) => (
                <LineRow
                  key={line._key}
                  index={i}
                  line={line}
                  lineTotal={totals.lineSubtotals[i]}
                  defaultVatRateId={defaultVatRateId}
                  defaultAccountId={defaultAccountId}
                  activeVatRates={activeVatRates}
                  accounts={accounts}
                  canDelete={lines.length > 1}
                  onUpdate={onUpdateLine}
                  onRemove={onRemoveLine}
                />
              ))}
            </TableBody>
          </Table>
        </div>

        {/* Mobile cards */}
        <div className="space-y-3 p-4 sm:hidden">
          {lines.map((line, i) => (
            <MobileLineCard
              key={line._key}
              index={i}
              line={line}
              lineTotal={totals.lineSubtotals[i]}
              defaultVatRateId={defaultVatRateId}
              activeVatRates={activeVatRates}
              canDelete={lines.length > 1}
              onUpdate={onUpdateLine}
              onRemove={onRemoveLine}
            />
          ))}
        </div>
      </CardContent>
      <CardFooter className="flex-col items-end gap-1 border-t pt-4">
        <div className="text-sm text-muted-foreground">
          {t('invoice_form.subtotal', 'Subtotal')}:{' '}
          <span className="font-mono font-medium text-foreground">{totals.subtotal.toFixed(2)}</span>
        </div>
        {Array.from(totals.vatByRate.entries()).map(([id, { name, rate, amount }]) => (
          <div key={id} className="text-sm text-muted-foreground">
            {t('invoice_form.vat_breakdown', 'VAT')} {name} ({rate}%):{' '}
            <span className="font-mono text-foreground">{amount.toFixed(2)}</span>
          </div>
        ))}
        <Separator className="my-1 w-48" />
        <div className="text-base font-medium">
          {t('invoice_form.total_with_vat', 'Total incl. VAT')}:{' '}
          <span className="font-mono font-bold">{totals.grandTotal.toFixed(2)}</span>
        </div>
      </CardFooter>
    </Card>
  );
}

function LineRow({
  index,
  line,
  lineTotal,
  defaultVatRateId,
  defaultAccountId,
  activeVatRates,
  accounts,
  canDelete,
  onUpdate,
  onRemove,
}: {
  index: number;
  line: LineFormData;
  lineTotal: number;
  defaultVatRateId: string;
  defaultAccountId: string;
  activeVatRates: VatRate[];
  accounts: { id: string; number: number; name: string }[];
  canDelete: boolean;
  onUpdate: (index: number, field: keyof LineFormData, value: string) => void;
  onRemove: (index: number) => void;
}) {
  const { t } = useI18n();
  const [vatOverrideOpen, setVatOverrideOpen] = useState(false);
  const [accountOverrideOpen, setAccountOverrideOpen] = useState(false);

  const effectiveVatId = line.vat_rate_id || defaultVatRateId;
  const vatRate = activeVatRates.find((v) => v.id === effectiveVatId);
  const isVatOverridden = !!line.vat_rate_id && line.vat_rate_id !== defaultVatRateId;
  const isAccountOverridden = !!line.account_id && line.account_id !== defaultAccountId;

  return (
    <TableRow className="group">
      <TableCell className="align-top py-2">
        <RichTextEditor
          placeholder={t('common.description', 'Description')}
          value={line.description}
          onChange={(md) => onUpdate(index, 'description', md)}
          minimal
          className="min-h-[36px] border-0 bg-transparent shadow-none focus-within:ring-1"
        />
        <div className="mt-1 flex items-center gap-1.5">
          {vatRate && (
            <Badge
              variant="outline"
              className={`cursor-pointer text-xs ${isVatOverridden ? 'border-primary text-primary' : 'text-muted-foreground'}`}
              onClick={() => setVatOverrideOpen(!vatOverrideOpen)}
            >
              {vatRate.rate}%
            </Badge>
          )}
          {isAccountOverridden && (
            <Badge variant="outline" className="border-primary text-xs text-primary">
              {accounts.find((a) => a.id === line.account_id)?.number}
            </Badge>
          )}
          <button
            type="button"
            className="invisible text-muted-foreground hover:text-foreground group-hover:visible"
            onClick={() => setAccountOverrideOpen(!accountOverrideOpen)}
            title={t('invoice_form.override_account', 'Override Account')}
          >
            <Settings className="h-3 w-3" />
          </button>
        </div>
        {vatOverrideOpen && (
          <div className="mt-1.5 max-w-[200px]">
            <Select
              value={line.vat_rate_id || '__default__'}
              onValueChange={(v) => {
                onUpdate(index, 'vat_rate_id', v === '__default__' ? '' : v);
                setVatOverrideOpen(false);
              }}
            >
              <SelectTrigger className="h-8 text-xs">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="__default__">{t('invoice_form.default_account', 'Default')}</SelectItem>
                {activeVatRates.map((v) => (
                  <SelectItem key={v.id} value={v.id}>
                    {v.name} ({v.rate}%)
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
        )}
        {accountOverrideOpen && (
          <div className="mt-1.5 max-w-[250px]">
            <Select
              value={line.account_id || '__default__'}
              onValueChange={(v) => {
                onUpdate(index, 'account_id', v === '__default__' ? '' : v);
                setAccountOverrideOpen(false);
              }}
            >
              <SelectTrigger className="h-8 text-xs">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="__default__">{t('invoice_form.default_account', 'Default')}</SelectItem>
                {accounts.map((a) => (
                  <SelectItem key={a.id} value={a.id}>
                    {a.number} {a.name}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
        )}
      </TableCell>
      <TableCell className="align-top py-2">
        <Input
          type="number"
          value={line.quantity}
          onChange={(e) => onUpdate(index, 'quantity', e.target.value)}
          step="0.01"
          className="border-0 bg-transparent text-right shadow-none focus-visible:ring-1"
        />
      </TableCell>
      <TableCell className="align-top py-2">
        <Input
          type="number"
          value={line.unit_price}
          onChange={(e) => onUpdate(index, 'unit_price', e.target.value)}
          step="0.01"
          className="border-0 bg-transparent text-right shadow-none focus-visible:ring-1"
        />
      </TableCell>
      <TableCell className="align-top py-2">
        <Input
          type="number"
          value={line.discount_percent}
          onChange={(e) => onUpdate(index, 'discount_percent', e.target.value)}
          step="0.1"
          min="0"
          max="100"
          placeholder="%"
          className="border-0 bg-transparent text-right shadow-none focus-visible:ring-1"
        />
      </TableCell>
      <TableCell className="align-top py-2 text-right">
        <span className="font-mono text-sm font-medium">{lineTotal.toFixed(2)}</span>
      </TableCell>
      <TableCell className="align-top py-2">
        {canDelete && (
          <Button
            type="button"
            variant="ghost"
            size="icon"
            className="h-8 w-8 opacity-0 group-hover:opacity-100"
            onClick={() => onRemove(index)}
          >
            <Trash2 className="h-3.5 w-3.5" />
          </Button>
        )}
      </TableCell>
    </TableRow>
  );
}

function MobileLineCard({
  index,
  line,
  lineTotal,
  defaultVatRateId,
  activeVatRates,
  canDelete,
  onUpdate,
  onRemove,
}: {
  index: number;
  line: LineFormData;
  lineTotal: number;
  defaultVatRateId: string;
  activeVatRates: VatRate[];
  canDelete: boolean;
  onUpdate: (index: number, field: keyof LineFormData, value: string) => void;
  onRemove: (index: number) => void;
}) {
  const { t } = useI18n();
  const effectiveVatId = line.vat_rate_id || defaultVatRateId;
  const vatRate = activeVatRates.find((v) => v.id === effectiveVatId);
  const isVatOverridden = !!line.vat_rate_id && line.vat_rate_id !== defaultVatRateId;

  return (
    <div className="rounded-lg border p-3 space-y-2">
      <RichTextEditor
        placeholder={t('common.description', 'Description')}
        value={line.description}
        onChange={(md) => onUpdate(index, 'description', md)}
        minimal
      />
      <div className="grid grid-cols-2 gap-2">
        <div>
          <label className="text-xs text-muted-foreground">{t('invoice_form.qty_short', 'Qty')}</label>
          <Input
            type="number"
            value={line.quantity}
            onChange={(e) => onUpdate(index, 'quantity', e.target.value)}
            step="0.01"
          />
        </div>
        <div>
          <label className="text-xs text-muted-foreground">{t('invoice_form.unit_price', 'Price')}</label>
          <Input
            type="number"
            value={line.unit_price}
            onChange={(e) => onUpdate(index, 'unit_price', e.target.value)}
            step="0.01"
          />
        </div>
      </div>
      <div className="grid grid-cols-2 gap-2">
        <div>
          <label className="text-xs text-muted-foreground">{t('invoice_form.discount', 'Disc.%')}</label>
          <Input
            type="number"
            value={line.discount_percent}
            onChange={(e) => onUpdate(index, 'discount_percent', e.target.value)}
            step="0.1"
            min="0"
            max="100"
          />
        </div>
        <div className="flex items-end justify-between">
          <div className="flex items-center gap-2">
            {vatRate && (
              <Badge
                variant="outline"
                className={`text-xs ${isVatOverridden ? 'border-primary text-primary' : 'text-muted-foreground'}`}
              >
                {vatRate.rate}%
              </Badge>
            )}
            <span className="font-mono text-sm font-medium">{lineTotal.toFixed(2)}</span>
          </div>
          {canDelete && (
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
