import { useState } from 'react';
import { useCurrencyExchanges, useRecordTransfer } from '@/hooks/useApi';
import { useBankAccounts } from '@/hooks/useSettingsApi';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Skeleton } from '@/components/ui/skeleton';
import {
  Dialog, DialogContent, DialogHeader, DialogTitle, DialogTrigger,
} from '@/components/ui/dialog';
import {
  Select, SelectContent, SelectItem, SelectTrigger, SelectValue,
} from '@/components/ui/select';
import {
  Table, TableBody, TableCell, TableHead, TableHeader, TableRow,
} from '@/components/ui/table';
import { Plus } from 'lucide-react';
import { toast } from 'sonner';
import { extractErrorMessage } from '@/api/client';

export function CurrencyExchangesPage() {
  const { data, isLoading } = useCurrencyExchanges();
  const { data: bankAccountsData } = useBankAccounts();
  const recordTransfer = useRecordTransfer();
  const [open, setOpen] = useState(false);
  const [form, setForm] = useState({
    from_bank_account_id: '',
    to_bank_account_id: '',
    from_amount: '',
    to_amount: '',
    date: new Date().toISOString().split('T')[0],
    notes: '',
  });

  const exchanges = data?.data ?? [];
  const bankAccounts = bankAccountsData ?? [];
  const bankAccountName = (id: string) => bankAccounts.find((a) => a.id === id)?.name ?? id;

  const fromAmountNum = parseFloat(form.from_amount);
  const toAmountNum = parseFloat(form.to_amount);
  const achievedRate = fromAmountNum > 0 && toAmountNum > 0 ? toAmountNum / fromAmountNum : null;

  function handleCreate() {
    recordTransfer.mutate(
      {
        from_bank_account_id: form.from_bank_account_id,
        to_bank_account_id: form.to_bank_account_id,
        from_amount: form.from_amount,
        to_amount: form.to_amount,
        date: form.date,
        notes: form.notes || undefined,
      },
      {
        onSuccess: () => {
          toast.success('Currency exchange recorded');
          setOpen(false);
          setForm({
            from_bank_account_id: '', to_bank_account_id: '',
            from_amount: '', to_amount: '', date: new Date().toISOString().split('T')[0], notes: '',
          });
        },
        onError: (err) => toast.error(extractErrorMessage(err)),
      },
    );
  }

  const canSubmit = form.from_bank_account_id
    && form.to_bank_account_id
    && form.from_bank_account_id !== form.to_bank_account_id
    && fromAmountNum > 0
    && toAmountNum > 0;

  return (
    <div className="space-y-4">
      <div className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h2 className="text-lg font-semibold">Currency Exchanges</h2>
          <p className="text-sm text-muted-foreground">
            Record transfers between bank accounts of different currencies — realized FX gain/loss is booked automatically at the real rate you achieved.
          </p>
        </div>
        <Dialog open={open} onOpenChange={setOpen}>
          <DialogTrigger asChild>
            <Button size="sm">
              <Plus className="mr-1 h-4 w-4" /> Record Transfer
            </Button>
          </DialogTrigger>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Record Currency Exchange</DialogTitle>
            </DialogHeader>
            <div className="space-y-4">
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <Label>From Account</Label>
                  <Select
                    value={form.from_bank_account_id}
                    onValueChange={(v) => setForm({ ...form, from_bank_account_id: v })}
                  >
                    <SelectTrigger><SelectValue placeholder="Select" /></SelectTrigger>
                    <SelectContent>
                      {bankAccounts.map((a) => (
                        <SelectItem key={a.id} value={a.id}>{a.name}</SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </div>
                <div>
                  <Label>To Account</Label>
                  <Select
                    value={form.to_bank_account_id}
                    onValueChange={(v) => setForm({ ...form, to_bank_account_id: v })}
                  >
                    <SelectTrigger><SelectValue placeholder="Select" /></SelectTrigger>
                    <SelectContent>
                      {bankAccounts.map((a) => (
                        <SelectItem key={a.id} value={a.id}>{a.name}</SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </div>
              </div>
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <Label>Amount Sent</Label>
                  <Input
                    type="number" step="0.01"
                    value={form.from_amount}
                    onChange={(e) => setForm({ ...form, from_amount: e.target.value })}
                    placeholder="0.00"
                  />
                </div>
                <div>
                  <Label>Amount Received</Label>
                  <Input
                    type="number" step="0.01"
                    value={form.to_amount}
                    onChange={(e) => setForm({ ...form, to_amount: e.target.value })}
                    placeholder="0.00"
                  />
                </div>
              </div>
              {achievedRate && (
                <p className="text-sm text-muted-foreground">
                  Achieved rate: <span className="font-mono">{achievedRate.toFixed(6)}</span>
                </p>
              )}
              <div>
                <Label>Date</Label>
                <Input
                  type="date"
                  value={form.date}
                  onChange={(e) => setForm({ ...form, date: e.target.value })}
                />
              </div>
              <div>
                <Label>Notes (optional)</Label>
                <Input
                  value={form.notes}
                  onChange={(e) => setForm({ ...form, notes: e.target.value })}
                  placeholder="e.g. wire transfer reference"
                />
              </div>
              <Button onClick={handleCreate} className="w-full" disabled={!canSubmit || recordTransfer.isPending}>
                Record Transfer
              </Button>
            </div>
          </DialogContent>
        </Dialog>
      </div>

      <Card>
        <CardContent className="p-0">
          {isLoading ? (
            <div className="space-y-2 p-4">
              {Array.from({ length: 3 }).map((_, i) => (
                <Skeleton key={i} className="h-10 w-full" />
              ))}
            </div>
          ) : exchanges.length > 0 ? (
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>Date</TableHead>
                  <TableHead>From</TableHead>
                  <TableHead className="text-right">Amount Sent</TableHead>
                  <TableHead>To</TableHead>
                  <TableHead className="text-right">Amount Received</TableHead>
                  <TableHead className="hidden sm:table-cell">Notes</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {exchanges.map((ex) => (
                  <TableRow key={ex.id}>
                    <TableCell className="font-mono text-sm">{ex.date}</TableCell>
                    <TableCell>{bankAccountName(ex.from_bank_account_id)}</TableCell>
                    <TableCell className="text-right font-mono text-sm">{ex.from_amount}</TableCell>
                    <TableCell>{bankAccountName(ex.to_bank_account_id)}</TableCell>
                    <TableCell className="text-right font-mono text-sm">{ex.to_amount}</TableCell>
                    <TableCell className="hidden sm:table-cell text-muted-foreground">{ex.notes ?? '—'}</TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          ) : (
            <p className="py-8 text-center text-sm text-muted-foreground">
              No currency exchanges recorded yet.
            </p>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
