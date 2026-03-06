import { useState } from 'react';
import { useExchangeRates, useCreateExchangeRate, useDeleteExchangeRate } from '@/hooks/useApi';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Skeleton } from '@/components/ui/skeleton';
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog';
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from '@/components/ui/alert-dialog';
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
import { Plus, Trash2 } from 'lucide-react';
import { toast } from 'sonner';

const currencies = ['CHF', 'EUR', 'USD'];

export function ExchangeRatesPage() {
  const { data, isLoading } = useExchangeRates();
  const createRate = useCreateExchangeRate();
  const deleteRate = useDeleteExchangeRate();
  const [open, setOpen] = useState(false);
  const [form, setForm] = useState({
    from_currency_id: '',
    to_currency_id: '',
    rate: '',
    valid_date: new Date().toISOString().split('T')[0],
    source: '',
  });

  const rates = data?.data ?? [];

  function handleCreate() {
    createRate.mutate(
      {
        from_currency_id: form.from_currency_id,
        to_currency_id: form.to_currency_id,
        rate: form.rate,
        valid_date: form.valid_date,
        source: form.source || undefined,
      },
      {
        onSuccess: () => {
          toast.success('Exchange rate created');
          setOpen(false);
          setForm({
            from_currency_id: '',
            to_currency_id: '',
            rate: '',
            valid_date: new Date().toISOString().split('T')[0],
            source: '',
          });
        },
        onError: () => toast.error('Failed to create exchange rate'),
      }
    );
  }

  function handleDelete(id: string) {
    deleteRate.mutate(id, {
      onSuccess: () => toast.success('Exchange rate deleted'),
      onError: () => toast.error('Failed to delete exchange rate'),
    });
  }

  return (
    <div className="space-y-4">
      <div className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h2 className="text-lg font-semibold">Exchange Rates</h2>
          <p className="text-sm text-muted-foreground">Manage currency exchange rates</p>
        </div>
        <Dialog open={open} onOpenChange={setOpen}>
          <DialogTrigger asChild>
            <Button size="sm">
              <Plus className="mr-1 h-4 w-4" /> Add Rate
            </Button>
          </DialogTrigger>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Create Exchange Rate</DialogTitle>
            </DialogHeader>
            <div className="space-y-4">
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <Label>From Currency</Label>
                  <Select
                    value={form.from_currency_id}
                    onValueChange={(v) => setForm({ ...form, from_currency_id: v })}
                  >
                    <SelectTrigger><SelectValue placeholder="Select" /></SelectTrigger>
                    <SelectContent>
                      {currencies.map((c) => (
                        <SelectItem key={c} value={c}>{c}</SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </div>
                <div>
                  <Label>To Currency</Label>
                  <Select
                    value={form.to_currency_id}
                    onValueChange={(v) => setForm({ ...form, to_currency_id: v })}
                  >
                    <SelectTrigger><SelectValue placeholder="Select" /></SelectTrigger>
                    <SelectContent>
                      {currencies.map((c) => (
                        <SelectItem key={c} value={c}>{c}</SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </div>
              </div>
              <div>
                <Label>Rate</Label>
                <Input
                  value={form.rate}
                  onChange={(e) => setForm({ ...form, rate: e.target.value })}
                  placeholder="1.0850"
                />
              </div>
              <div>
                <Label>Valid Date</Label>
                <Input
                  type="date"
                  value={form.valid_date}
                  onChange={(e) => setForm({ ...form, valid_date: e.target.value })}
                />
              </div>
              <div>
                <Label>Source</Label>
                <Input
                  value={form.source}
                  onChange={(e) => setForm({ ...form, source: e.target.value })}
                  placeholder="SNB, ECB, manual..."
                />
              </div>
              <Button onClick={handleCreate} className="w-full" disabled={createRate.isPending}>
                Create Exchange Rate
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
          ) : rates.length > 0 ? (
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>From</TableHead>
                  <TableHead>To</TableHead>
                  <TableHead className="text-right">Rate</TableHead>
                  <TableHead>Date</TableHead>
                  <TableHead className="hidden sm:table-cell">Source</TableHead>
                  <TableHead className="w-16" />
                </TableRow>
              </TableHeader>
              <TableBody>
                {rates.map((rate) => (
                  <TableRow key={rate.id}>
                    <TableCell className="font-mono font-medium">
                      {rate.from_currency_id}
                    </TableCell>
                    <TableCell className="font-mono font-medium">
                      {rate.to_currency_id}
                    </TableCell>
                    <TableCell className="text-right font-mono text-sm">
                      {rate.rate}
                    </TableCell>
                    <TableCell className="font-mono text-sm">{rate.valid_date}</TableCell>
                    <TableCell className="hidden sm:table-cell text-muted-foreground">
                      {rate.source ?? '—'}
                    </TableCell>
                    <TableCell>
                      <AlertDialog>
                        <AlertDialogTrigger asChild>
                          <Button variant="ghost" size="icon">
                            <Trash2 className="h-4 w-4" />
                          </Button>
                        </AlertDialogTrigger>
                        <AlertDialogContent>
                          <AlertDialogHeader>
                            <AlertDialogTitle>Delete Exchange Rate?</AlertDialogTitle>
                            <AlertDialogDescription>
                              This will permanently delete this exchange rate entry.
                            </AlertDialogDescription>
                          </AlertDialogHeader>
                          <AlertDialogFooter>
                            <AlertDialogCancel>Cancel</AlertDialogCancel>
                            <AlertDialogAction onClick={() => handleDelete(rate.id)}>
                              Delete
                            </AlertDialogAction>
                          </AlertDialogFooter>
                        </AlertDialogContent>
                      </AlertDialog>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          ) : (
            <p className="py-8 text-center text-sm text-muted-foreground">
              No exchange rates yet. Add your first rate.
            </p>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
