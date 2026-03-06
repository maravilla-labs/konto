import { useState, useCallback } from 'react';
import { useNavigate } from 'react-router-dom';
import { useCreateJournalEntry, usePostJournal } from '@/hooks/useApi';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { AccountSelect } from '@/components/ui/AccountSelect';
import { Plus, Trash2, Check, AlertTriangle } from 'lucide-react';
import { toast } from 'sonner';
import { extractErrorMessage } from '@/api/client';
import type { CreateJournalLine } from '@/types/journal';

interface GridLine {
  account_id: string;
  debit: string;
  credit: string;
  description: string;
}

const emptyLine = (): GridLine => ({ account_id: '', debit: '', credit: '', description: '' });

export function JournalCreatePage() {
  const navigate = useNavigate();
  const createEntry = useCreateJournalEntry();
  const postJournal = usePostJournal();

  const [date, setDate] = useState(new Date().toISOString().split('T')[0]);
  const [description, setDescription] = useState('');
  const [reference, setReference] = useState('');
  const [lines, setLines] = useState<GridLine[]>([emptyLine(), emptyLine()]);

  const updateLine = useCallback((index: number, field: keyof GridLine, value: string) => {
    setLines((prev) => {
      const next = [...prev];
      next[index] = { ...next[index], [field]: value };
      return next;
    });
  }, []);

  function addLine() {
    setLines((prev) => [...prev, emptyLine()]);
  }

  function removeLine(index: number) {
    if (lines.length <= 2) return;
    setLines((prev) => prev.filter((_, i) => i !== index));
  }

  const totalDebit = lines.reduce((s, l) => s + (parseFloat(l.debit) || 0), 0);
  const totalCredit = lines.reduce((s, l) => s + (parseFloat(l.credit) || 0), 0);
  const isBalanced = Math.abs(totalDebit - totalCredit) < 0.005 && totalDebit > 0;

  function buildPayload(): { date: string; description: string; reference?: string; lines: CreateJournalLine[] } {
    const apiLines: CreateJournalLine[] = lines
      .filter((l) => l.account_id)
      .map((l) => ({
        account_id: l.account_id,
        debit_amount: (parseFloat(l.debit) || 0).toFixed(2),
        credit_amount: (parseFloat(l.credit) || 0).toFixed(2),
        description: l.description || undefined,
      }));
    return {
      date,
      description,
      reference: reference || undefined,
      lines: apiLines,
    };
  }

  function handleSaveDraft() {
    createEntry.mutate(buildPayload(), {
      onSuccess: (res) => {
        toast.success('Draft saved');
        navigate(`/journal/${res.data.entry.id}`);
      },
      onError: (err) => toast.error(extractErrorMessage(err)),
    });
  }

  function handleSaveAndPost() {
    createEntry.mutate(buildPayload(), {
      onSuccess: (res) => {
        postJournal.mutate(res.data.entry.id, {
          onSuccess: () => {
            toast.success('Entry posted');
            navigate(`/journal/${res.data.entry.id}`);
          },
          onError: () => toast.error('Saved as draft, but failed to post'),
        });
      },
      onError: (err) => toast.error(extractErrorMessage(err)),
    });
  }

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-lg font-semibold">New Journal Entry</h2>
          <p className="text-sm text-muted-foreground">
            Type account numbers or names, Tab between cells
          </p>
        </div>
      </div>

      <Card>
        <CardContent className="space-y-4 pt-6">
          <div className="grid grid-cols-1 gap-4 sm:grid-cols-3">
            <div>
              <Label>Date</Label>
              <Input type="date" value={date} onChange={(e) => setDate(e.target.value)} />
            </div>
            <div>
              <Label>Description</Label>
              <Input
                value={description}
                onChange={(e) => setDescription(e.target.value)}
                placeholder="Entry description"
              />
            </div>
            <div>
              <Label>Reference</Label>
              <Input
                value={reference}
                onChange={(e) => setReference(e.target.value)}
                placeholder="Optional reference"
              />
            </div>
          </div>

          {/* Spreadsheet Grid */}
          <div className="overflow-x-auto">
            <table className="w-full border-collapse text-sm">
              <thead>
                <tr className="border-b bg-muted/50">
                  <th className="px-2 py-2 text-left font-medium" style={{ minWidth: 220 }}>Account</th>
                  <th className="px-2 py-2 text-right font-medium" style={{ width: 140 }}>Debit</th>
                  <th className="px-2 py-2 text-right font-medium" style={{ width: 140 }}>Credit</th>
                  <th className="px-2 py-2 text-left font-medium" style={{ minWidth: 160 }}>Description</th>
                  <th className="w-10" />
                </tr>
              </thead>
              <tbody>
                {lines.map((line, i) => (
                  <tr key={i} className="border-b">
                    <td className="px-1 py-1">
                      <AccountSelect
                        value={line.account_id}
                        onChange={(v) => updateLine(i, 'account_id', v)}
                        className="h-9 text-sm"
                      />
                    </td>
                    <td className="px-1 py-1">
                      <Input
                        type="number"
                        step="0.01"
                        min="0"
                        value={line.debit}
                        onChange={(e) => updateLine(i, 'debit', e.target.value)}
                        className="h-9 text-right font-mono text-sm"
                        placeholder="0.00"
                      />
                    </td>
                    <td className="px-1 py-1">
                      <Input
                        type="number"
                        step="0.01"
                        min="0"
                        value={line.credit}
                        onChange={(e) => updateLine(i, 'credit', e.target.value)}
                        className="h-9 text-right font-mono text-sm"
                        placeholder="0.00"
                      />
                    </td>
                    <td className="px-1 py-1">
                      <Input
                        value={line.description}
                        onChange={(e) => updateLine(i, 'description', e.target.value)}
                        className="h-9 text-sm"
                        placeholder="Line note"
                      />
                    </td>
                    <td className="px-1 py-1">
                      {lines.length > 2 && (
                        <Button
                          variant="ghost"
                          size="icon"
                          className="h-8 w-8"
                          onClick={() => removeLine(i)}
                        >
                          <Trash2 className="h-3.5 w-3.5 text-muted-foreground" />
                        </Button>
                      )}
                    </td>
                  </tr>
                ))}
              </tbody>
              <tfoot>
                <tr className="border-t-2">
                  <td className="px-2 py-2">
                    <Button variant="ghost" size="sm" onClick={addLine}>
                      <Plus className="mr-1 h-3.5 w-3.5" /> Add Row
                    </Button>
                  </td>
                  <td className="px-2 py-2 text-right font-mono font-semibold">
                    {totalDebit.toFixed(2)}
                  </td>
                  <td className="px-2 py-2 text-right font-mono font-semibold">
                    {totalCredit.toFixed(2)}
                  </td>
                  <td className="px-2 py-2">
                    {isBalanced ? (
                      <span className="flex items-center gap-1 text-green-600 text-xs">
                        <Check className="h-3.5 w-3.5" /> Balanced
                      </span>
                    ) : totalDebit > 0 || totalCredit > 0 ? (
                      <span className="flex items-center gap-1 text-red-600 text-xs">
                        <AlertTriangle className="h-3.5 w-3.5" />
                        Diff: {Math.abs(totalDebit - totalCredit).toFixed(2)}
                      </span>
                    ) : null}
                  </td>
                  <td />
                </tr>
              </tfoot>
            </table>
          </div>

          {/* Actions */}
          <div className="flex justify-end gap-2 border-t pt-4">
            <Button variant="outline" onClick={() => navigate('/journal')}>
              Cancel
            </Button>
            <Button
              variant="secondary"
              onClick={handleSaveDraft}
              disabled={!description || !isBalanced || createEntry.isPending}
            >
              Save as Draft
            </Button>
            <Button
              onClick={handleSaveAndPost}
              disabled={!description || !isBalanced || createEntry.isPending}
            >
              Save & Post
            </Button>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
