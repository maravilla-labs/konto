import { useState, useRef } from 'react';
import { useNavigate } from 'react-router-dom';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Badge } from '@/components/ui/badge';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table';
import { Upload, Wand2, ArrowRightLeft } from 'lucide-react';
import { toast } from 'sonner';
import { useBankTransactions, useImportCamt053, useAutoMatch } from '@/hooks/useBankingApi';
import { useBankAccounts } from '@/hooks/useSettingsApi';

const statusColors: Record<string, string> = {
  unmatched: 'bg-yellow-100 text-yellow-800',
  matched: 'bg-green-100 text-green-800',
  ignored: 'bg-gray-100 text-gray-800',
};

export function BankingPage() {
  const navigate = useNavigate();
  const [bankAccountId, setBankAccountId] = useState<string>('');
  const [statusFilter, setStatusFilter] = useState<string>('');
  const [page, setPage] = useState(1);
  const fileRef = useRef<HTMLInputElement>(null);

  const { data: bankAccounts } = useBankAccounts();
  const { data: txData, isLoading } = useBankTransactions({
    page,
    per_page: 25,
    bank_account_id: bankAccountId || undefined,
    status: statusFilter || undefined,
  });
  const importMut = useImportCamt053();
  const autoMatchMut = useAutoMatch();

  const transactions = txData?.data ?? [];
  const total = txData?.total ?? 0;
  const totalPages = Math.ceil(total / 25);

  const handleImport = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file || !bankAccountId) {
      toast.error('Please select a bank account first');
      return;
    }
    try {
      const result = await importMut.mutateAsync({ bankAccountId, file });
      toast.success(`Imported ${result.imported_count} transactions`);
    } catch {
      toast.error('Import failed');
    }
    if (fileRef.current) fileRef.current.value = '';
  };

  const handleAutoMatch = async () => {
    if (!bankAccountId) {
      toast.error('Please select a bank account first');
      return;
    }
    try {
      const result = await autoMatchMut.mutateAsync(bankAccountId);
      toast.success(`Matched ${result.matched_count}, ${result.unmatched_count} remaining`);
    } catch {
      toast.error('Auto-match failed');
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-bold">Banking</h1>
        <div className="flex gap-2">
          <input type="file" ref={fileRef} accept=".xml" className="hidden" onChange={handleImport} />
          <Button variant="outline" onClick={() => fileRef.current?.click()} disabled={!bankAccountId}>
            <Upload className="mr-2 h-4 w-4" /> Import CAMT.053
          </Button>
          <Button variant="outline" onClick={handleAutoMatch} disabled={!bankAccountId || autoMatchMut.isPending}>
            <Wand2 className="mr-2 h-4 w-4" /> Auto-Match
          </Button>
          <Button onClick={() => navigate('/banking/reconcile')}>
            <ArrowRightLeft className="mr-2 h-4 w-4" /> Reconcile
          </Button>
        </div>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>Filters</CardTitle>
        </CardHeader>
        <CardContent className="flex gap-4">
          <Select value={bankAccountId || 'all'} onValueChange={(v) => setBankAccountId(v === 'all' ? '' : v)}>
            <SelectTrigger className="w-[250px]">
              <SelectValue placeholder="All bank accounts" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="all">All bank accounts</SelectItem>
              {(bankAccounts ?? []).map((a: { id: string; name: string; iban: string }) => (
                <SelectItem key={a.id} value={a.id}>{a.name} ({a.iban})</SelectItem>
              ))}
            </SelectContent>
          </Select>
          <Select value={statusFilter || 'all'} onValueChange={(v) => setStatusFilter(v === 'all' ? '' : v)}>
            <SelectTrigger className="w-[180px]">
              <SelectValue placeholder="All statuses" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="all">All statuses</SelectItem>
              <SelectItem value="unmatched">Unmatched</SelectItem>
              <SelectItem value="matched">Matched</SelectItem>
              <SelectItem value="ignored">Ignored</SelectItem>
            </SelectContent>
          </Select>
        </CardContent>
      </Card>

      <Card>
        <CardContent className="p-0">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Date</TableHead>
                <TableHead>Description</TableHead>
                <TableHead>Counterparty</TableHead>
                <TableHead>Reference</TableHead>
                <TableHead className="text-right">Amount</TableHead>
                <TableHead>Status</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {isLoading ? (
                <TableRow><TableCell colSpan={6} className="text-center py-8">Loading...</TableCell></TableRow>
              ) : transactions.length === 0 ? (
                <TableRow><TableCell colSpan={6} className="text-center py-8">No transactions found</TableCell></TableRow>
              ) : (
                transactions.map((tx) => (
                  <TableRow key={tx.id}>
                    <TableCell className="whitespace-nowrap">{tx.transaction_date}</TableCell>
                    <TableCell className="max-w-[250px] truncate">{tx.description}</TableCell>
                    <TableCell>{tx.counterparty_name ?? '-'}</TableCell>
                    <TableCell className="font-mono text-xs">{tx.reference ?? tx.bank_reference ?? '-'}</TableCell>
                    <TableCell className={`text-right font-mono ${parseFloat(tx.amount) >= 0 ? 'text-green-600' : 'text-red-600'}`}>
                      {parseFloat(tx.amount).toLocaleString('de-CH', { minimumFractionDigits: 2 })} {tx.currency}
                    </TableCell>
                    <TableCell>
                      <Badge className={statusColors[tx.status] ?? ''}>{tx.status}</Badge>
                    </TableCell>
                  </TableRow>
                ))
              )}
            </TableBody>
          </Table>
        </CardContent>
      </Card>

      {totalPages > 1 && (
        <div className="flex justify-center gap-2">
          <Button variant="outline" size="sm" disabled={page <= 1} onClick={() => setPage(page - 1)}>Previous</Button>
          <span className="flex items-center px-3 text-sm">Page {page} of {totalPages}</span>
          <Button variant="outline" size="sm" disabled={page >= totalPages} onClick={() => setPage(page + 1)}>Next</Button>
        </div>
      )}
    </div>
  );
}
