import { useState } from 'react';
import { Button } from '@/components/ui/button';
import { Checkbox } from '@/components/ui/checkbox';
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from '@/components/ui/dialog';
import { useTimeEntries, useProjectActivityTypes } from '@/hooks/useApi';
import { useI18n } from '@/i18n';
import type { LineFormData } from './InvoiceFormTypes';

interface TimeEntryImportDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  projectId: string;
  defaultAccountId: string;
  defaultVatRateId: string;
  defaultRate?: number;
  onImport: (lines: LineFormData[]) => void;
}

export function TimeEntryImportDialog({
  open,
  onOpenChange,
  projectId,
  defaultAccountId,
  defaultVatRateId,
  defaultRate,
  onImport,
}: TimeEntryImportDialogProps) {
  const { t } = useI18n();
  const [selected, setSelected] = useState<Set<string>>(new Set());
  const { data: entriesData, isLoading } = useTimeEntries({
    project_id: projectId,
    billed: false,
    per_page: 200,
  });
  const { data: projectActivityTypes } = useProjectActivityTypes(projectId);

  const entries = entriesData?.data ?? [];

  // Build rate lookup: activity_type_id → effective_rate
  // Fallback chain: project activity type effective_rate → project hourly_rate → 0
  const activityRateMap = new Map<string, number>();
  if (projectActivityTypes) {
    for (const pat of projectActivityTypes) {
      if (pat.effective_rate != null) {
        activityRateMap.set(pat.activity_type_id, Number(pat.effective_rate));
      }
    }
  }

  function resolveRate(activityTypeId?: string): number {
    if (activityTypeId) {
      const activityRate = activityRateMap.get(activityTypeId);
      if (activityRate != null && !isNaN(activityRate)) return activityRate;
    }
    return defaultRate ?? 0;
  }

  function toggleEntry(id: string) {
    setSelected((prev) => {
      const next = new Set(prev);
      if (next.has(id)) next.delete(id);
      else next.add(id);
      return next;
    });
  }

  function toggleAll() {
    if (selected.size === entries.length) {
      setSelected(new Set());
    } else {
      setSelected(new Set(entries.map((e) => e.id)));
    }
  }

  function handleImport() {
    const lines: LineFormData[] = entries
      .filter((e) => selected.has(e.id))
      .map((e) => {
        const hours = e.actual_minutes / 60;
        const rate = resolveRate(e.activity_type_id);
        return {
          description: e.description || `${e.date} — ${e.actual_minutes} min`,
          quantity: hours.toFixed(2),
          unit_price: rate.toString(),
          vat_rate_id: defaultVatRateId,
          account_id: defaultAccountId,
          discount_percent: '',
        };
      });
    onImport(lines);
    onOpenChange(false);
    setSelected(new Set());
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-2xl">
        <DialogHeader>
          <DialogTitle>{t('invoice_form.import_time_entries', 'Import Time Entries')}</DialogTitle>
        </DialogHeader>

        {isLoading ? (
          <div className="py-8 text-center text-muted-foreground">Loading...</div>
        ) : entries.length === 0 ? (
          <div className="py-8 text-center text-muted-foreground">
            {t('invoice_form.no_unbilled_entries', 'No unbilled time entries found for this project.')}
          </div>
        ) : (
          <div className="max-h-96 overflow-y-auto">
            <table className="w-full text-sm">
              <thead className="sticky top-0 bg-background">
                <tr className="border-b text-left text-muted-foreground">
                  <th className="p-2">
                    <Checkbox
                      checked={selected.size === entries.length && entries.length > 0}
                      onCheckedChange={toggleAll}
                    />
                  </th>
                  <th className="p-2">{t('common.date', 'Date')}</th>
                  <th className="p-2">{t('common.description', 'Description')}</th>
                  <th className="p-2 text-right">{t('common.hours', 'Hours')}</th>
                  <th className="p-2 text-right">{t('common.rate', 'Rate')}</th>
                </tr>
              </thead>
              <tbody>
                {entries.map((entry) => (
                  <tr key={entry.id} className="border-b hover:bg-muted/50">
                    <td className="p-2">
                      <Checkbox
                        checked={selected.has(entry.id)}
                        onCheckedChange={() => toggleEntry(entry.id)}
                      />
                    </td>
                    <td className="p-2">{entry.date}</td>
                    <td className="p-2 max-w-xs truncate">
                      {entry.description || `${entry.actual_minutes} min`}
                    </td>
                    <td className="p-2 text-right font-mono">
                      {(entry.actual_minutes / 60).toFixed(1)}h
                    </td>
                    <td className="p-2 text-right font-mono">
                      {resolveRate(entry.activity_type_id).toFixed(2)}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}

        <DialogFooter>
          <Button variant="outline" onClick={() => onOpenChange(false)}>
            {t('common.cancel', 'Cancel')}
          </Button>
          <Button onClick={handleImport} disabled={selected.size === 0}>
            {t('invoice_form.import_selected', 'Import Selected')} ({selected.size})
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
