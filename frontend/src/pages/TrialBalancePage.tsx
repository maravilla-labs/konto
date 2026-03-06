import { useState, useMemo } from 'react';
import { useTrialBalance } from '@/hooks/useApi';
import { Card, CardContent } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Skeleton } from '@/components/ui/skeleton';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import { StickyToolbar, type ToolbarAction, type ToolbarOverflowItem } from '@/components/ui/sticky-toolbar';
import { formatAmount } from '@/lib/format';
import { downloadCsv } from '@/lib/export';
import { Download, RefreshCw } from 'lucide-react';

export function TrialBalancePage() {
  const today = new Date().toISOString().split('T')[0];
  const [asOf, setAsOf] = useState(today);
  const { data, isLoading, refetch } = useTrialBalance({ as_of: asOf });

  const totals = data?.reduce(
    (acc, row) => ({
      debit: acc.debit + parseFloat(row.total_debit || '0'),
      credit: acc.credit + parseFloat(row.total_credit || '0'),
    }),
    { debit: 0, credit: 0 }
  );

  const actions = useMemo<ToolbarAction[]>(() => [
    { icon: <RefreshCw className="h-4 w-4" />, label: 'Generate', onClick: () => refetch(), disabled: isLoading, loading: isLoading, primary: true },
  ], [isLoading]);

  const overflow = useMemo<ToolbarOverflowItem[]>(() => [
    { icon: <Download className="h-4 w-4" />, label: 'Export CSV', onClick: () => downloadCsv('/reports/trial-balance', { as_of: asOf }) },
  ], [asOf]);

  return (
    <div className="space-y-4">
      <StickyToolbar actions={actions} overflow={overflow}>
        <div className="flex items-center gap-1">
          <Label className="text-xs text-muted-foreground">As of</Label>
          <Input type="date" value={asOf} onChange={(e) => setAsOf(e.target.value)} className="h-8 text-sm w-36" />
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
                  <TableHead className="w-20">Acct #</TableHead>
                  <TableHead>Account Name</TableHead>
                  <TableHead className="hidden sm:table-cell">Type</TableHead>
                  <TableHead className="text-right">Debit</TableHead>
                  <TableHead className="text-right">Credit</TableHead>
                  <TableHead className="text-right">Balance</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {data.map((row) => (
                  <TableRow key={row.account_id}>
                    <TableCell className="font-mono text-sm text-muted-foreground">
                      {row.account_number}
                    </TableCell>
                    <TableCell className="font-medium">{row.account_name}</TableCell>
                    <TableCell className="hidden sm:table-cell text-muted-foreground">
                      {row.account_type}
                    </TableCell>
                    <TableCell className="text-right font-mono text-sm">
                      {formatAmount(row.total_debit)}
                    </TableCell>
                    <TableCell className="text-right font-mono text-sm">
                      {formatAmount(row.total_credit)}
                    </TableCell>
                    <TableCell className="text-right font-mono text-sm font-medium">
                      {formatAmount(row.balance)}
                    </TableCell>
                  </TableRow>
                ))}
                {totals && (
                  <TableRow className="bg-muted/50 font-bold">
                    <TableCell colSpan={3}>Totals</TableCell>
                    <TableCell className="text-right font-mono text-sm">
                      {formatAmount(totals.debit.toFixed(2))}
                    </TableCell>
                    <TableCell className="text-right font-mono text-sm">
                      {formatAmount(totals.credit.toFixed(2))}
                    </TableCell>
                    <TableCell />
                  </TableRow>
                )}
              </TableBody>
            </Table>
          ) : (
            <p className="py-8 text-center text-sm text-muted-foreground">
              Click &quot;Generate&quot; to view the trial balance.
            </p>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
