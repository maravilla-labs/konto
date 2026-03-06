import { useState, useMemo } from 'react';
import { useAccounts, useAccountLedger } from '@/hooks/useApi';
import { Card, CardContent } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Skeleton } from '@/components/ui/skeleton';
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
import { StickyToolbar, type ToolbarAction } from '@/components/ui/sticky-toolbar';
import { formatAmount } from '@/lib/format';
import { RefreshCw } from 'lucide-react';

function firstOfMonth(): string {
  const d = new Date();
  return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-01`;
}

export function AccountLedgerPage() {
  const [accountId, setAccountId] = useState('');
  const [from, setFrom] = useState(firstOfMonth());
  const [to, setTo] = useState(new Date().toISOString().split('T')[0]);
  const { data: accountsData } = useAccounts({ per_page: 500 });
  const accounts = accountsData?.data ?? [];

  const params = accountId ? { account_id: accountId, from_date: from, to_date: to } : null;
  const { data, isLoading, refetch } = useAccountLedger(params);

  const actions = useMemo<ToolbarAction[]>(() => [
    { icon: <RefreshCw className="h-4 w-4" />, label: 'Generate', onClick: () => refetch(), disabled: isLoading || !accountId, loading: isLoading, primary: true },
  ], [isLoading, accountId]);

  return (
    <div className="space-y-4">
      <StickyToolbar actions={actions}>
        <Select value={accountId} onValueChange={setAccountId}>
          <SelectTrigger className="h-8 text-sm w-52">
            <SelectValue placeholder="Select account" />
          </SelectTrigger>
          <SelectContent>
            {accounts.map((a) => (
              <SelectItem key={a.id} value={a.id}>
                {a.number} — {a.name}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
        <div className="flex items-center gap-1">
          <Label className="text-xs text-muted-foreground">From</Label>
          <Input type="date" value={from} onChange={(e) => setFrom(e.target.value)} className="h-8 text-sm w-36" />
        </div>
        <div className="flex items-center gap-1">
          <Label className="text-xs text-muted-foreground">To</Label>
          <Input type="date" value={to} onChange={(e) => setTo(e.target.value)} className="h-8 text-sm w-36" />
        </div>
      </StickyToolbar>

      <Card>
        <CardContent className="p-0">
          {isLoading ? (
            <div className="space-y-2 p-4">
              {Array.from({ length: 5 }).map((_, i) => (
                <Skeleton key={i} className="h-10 w-full" />
              ))}
            </div>
          ) : data && data.length > 0 ? (
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>Date</TableHead>
                  <TableHead>Description</TableHead>
                  <TableHead className="text-right">Debit</TableHead>
                  <TableHead className="text-right">Credit</TableHead>
                  <TableHead className="text-right">Balance</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {data.map((row, i) => (
                  <TableRow key={`${row.entry_id}-${i}`}>
                    <TableCell className="font-mono text-sm">{row.date}</TableCell>
                    <TableCell>{row.description}</TableCell>
                    <TableCell className="text-right font-mono text-sm">
                      {formatAmount(row.debit)}
                    </TableCell>
                    <TableCell className="text-right font-mono text-sm">
                      {formatAmount(row.credit)}
                    </TableCell>
                    <TableCell className="text-right font-mono text-sm font-medium">
                      {formatAmount(row.running_balance)}
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          ) : (
            <p className="py-8 text-center text-sm text-muted-foreground">
              {accountId
                ? 'Click "Generate" to view the ledger.'
                : 'Select an account to view its ledger.'}
            </p>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
