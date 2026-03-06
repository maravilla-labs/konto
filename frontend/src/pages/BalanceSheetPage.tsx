import { useState, useMemo } from 'react';
import { useBalanceSheet } from '@/hooks/useApi';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Skeleton } from '@/components/ui/skeleton';
import {
  Table, TableBody, TableCell, TableHead, TableHeader, TableRow,
} from '@/components/ui/table';
import { StickyToolbar, type ToolbarAction, type ToolbarOverflowItem } from '@/components/ui/sticky-toolbar';
import { formatAmount } from '@/lib/format';
import { downloadCsv } from '@/lib/export';
import { Download, RefreshCw } from 'lucide-react';

export function BalanceSheetPage() {
  const today = new Date().toISOString().split('T')[0];
  const [asOf, setAsOf] = useState(today);
  const [compAsOf, setCompAsOf] = useState('');
  const { data, isLoading, refetch } = useBalanceSheet({ as_of: asOf });
  const { data: compData, refetch: compRefetch } = useBalanceSheet(
    compAsOf ? { as_of: compAsOf } : undefined,
  );

  const comparing = !!(compAsOf && compData);

  const handleGenerate = () => {
    refetch();
    if (compAsOf) compRefetch();
  };

  const actions = useMemo<ToolbarAction[]>(() => [
    { icon: <RefreshCw className="h-4 w-4" />, label: 'Generate', onClick: handleGenerate, disabled: isLoading, loading: isLoading, primary: true },
  ], [isLoading]);

  const overflow = useMemo<ToolbarOverflowItem[]>(() => [
    { icon: <Download className="h-4 w-4" />, label: 'Export CSV', onClick: () => downloadCsv('/reports/balance-sheet', { as_of: asOf }) },
  ], [asOf]);

  return (
    <div className="space-y-4">
      <StickyToolbar actions={actions} overflow={overflow}>
        <div className="flex items-center gap-1">
          <Label className="text-xs text-muted-foreground">As of</Label>
          <Input type="date" value={asOf} onChange={(e) => setAsOf(e.target.value)} className="h-8 text-sm w-36" />
        </div>
        <div className="flex items-center gap-1">
          <Label className="text-xs text-muted-foreground">Compare to</Label>
          <Input type="date" value={compAsOf} onChange={(e) => setCompAsOf(e.target.value)} className="h-8 text-sm w-36" />
        </div>
      </StickyToolbar>

      {isLoading ? (
        <div className="space-y-2">
          {Array.from({ length: 3 }).map((_, i) => <Skeleton key={i} className="h-32 w-full" />)}
        </div>
      ) : data ? (
        <div className="space-y-4">
          <BSSection title="Assets" accounts={data.assets} compAccounts={compData?.assets} comparing={comparing} />
          <BSSection title="Liabilities" accounts={data.liabilities} compAccounts={compData?.liabilities} comparing={comparing} />
          <BSSection title="Equity" accounts={data.equity} compAccounts={compData?.equity} comparing={comparing} />

          <TotalCard label="Total Assets" current={data.total_assets} comp={compData?.total_assets} comparing={comparing} />
          <TotalCard label="Total Liabilities + Equity" current={data.total_liabilities_equity}
            comp={compData?.total_liabilities_equity} comparing={comparing} />
        </div>
      ) : (
        <Card><CardContent className="py-8"><p className="text-center text-sm text-muted-foreground">Click &quot;Generate&quot; to view the balance sheet.</p></CardContent></Card>
      )}
    </div>
  );
}

type Acct = { account_id: string; account_number: number; account_name: string; balance: string };

function BSSection({ title, accounts, compAccounts, comparing }: {
  title: string; accounts: Acct[]; compAccounts?: Acct[]; comparing: boolean;
}) {
  const compMap = new Map((compAccounts ?? []).map((a) => [a.account_id, a]));
  const total = accounts.reduce((s, a) => s + parseFloat(a.balance || '0'), 0);
  const compTotal = (compAccounts ?? []).reduce((s, a) => s + parseFloat(a.balance || '0'), 0);

  return (
    <Card>
      <CardHeader className="pb-2"><CardTitle className="text-base">{title}</CardTitle></CardHeader>
      <CardContent className="p-0">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead className="w-20">Acct #</TableHead>
              <TableHead>Account</TableHead>
              <TableHead className="text-right">Current</TableHead>
              {comparing && <TableHead className="text-right">Comparison</TableHead>}
              {comparing && <TableHead className="text-right">Variance</TableHead>}
              {comparing && <TableHead className="text-right">%</TableHead>}
            </TableRow>
          </TableHeader>
          <TableBody>
            {accounts.map((a) => {
              const comp = compMap.get(a.account_id);
              const cur = parseFloat(a.balance || '0');
              const prev = parseFloat(comp?.balance || '0');
              const v = cur - prev;
              const pct = prev !== 0 ? (v / Math.abs(prev)) * 100 : 0;
              return (
                <TableRow key={a.account_id}>
                  <TableCell className="font-mono text-sm text-muted-foreground">{a.account_number}</TableCell>
                  <TableCell>{a.account_name}</TableCell>
                  <TableCell className="text-right font-mono text-sm">{formatAmount(a.balance)}</TableCell>
                  {comparing && <TableCell className="text-right font-mono text-sm">{formatAmount(comp?.balance ?? '0')}</TableCell>}
                  {comparing && <TableCell className={`text-right font-mono text-sm ${v >= 0 ? 'text-green-600' : 'text-red-600'}`}>{formatAmount(v.toFixed(2))}</TableCell>}
                  {comparing && <TableCell className={`text-right font-mono text-sm ${v >= 0 ? 'text-green-600' : 'text-red-600'}`}>{pct.toFixed(1)}%</TableCell>}
                </TableRow>
              );
            })}
            <TableRow className="bg-muted/50 font-bold">
              <TableCell colSpan={2}>Subtotal</TableCell>
              <TableCell className="text-right font-mono text-sm">{formatAmount(total.toFixed(2))}</TableCell>
              {comparing && <TableCell className="text-right font-mono text-sm">{formatAmount(compTotal.toFixed(2))}</TableCell>}
              {comparing && <VarianceCell current={total} comparison={compTotal} />}
              {comparing && <PctCell current={total} comparison={compTotal} />}
            </TableRow>
          </TableBody>
        </Table>
      </CardContent>
    </Card>
  );
}

function VarianceCell({ current, comparison }: { current: number; comparison: number }) {
  const v = current - comparison;
  return <TableCell className={`text-right font-mono text-sm ${v >= 0 ? 'text-green-600' : 'text-red-600'}`}>{formatAmount(v.toFixed(2))}</TableCell>;
}

function PctCell({ current, comparison }: { current: number; comparison: number }) {
  const v = current - comparison;
  const pct = comparison !== 0 ? (v / Math.abs(comparison)) * 100 : 0;
  return <TableCell className={`text-right font-mono text-sm ${v >= 0 ? 'text-green-600' : 'text-red-600'}`}>{pct.toFixed(1)}%</TableCell>;
}

function TotalCard({ label, current, comp, comparing }: { label: string; current: string; comp?: string; comparing: boolean }) {
  const curNum = parseFloat(current);
  const compNum = parseFloat(comp ?? '0');
  const v = curNum - compNum;
  return (
    <Card>
      <CardContent className="flex items-center justify-between py-4">
        <span className="font-semibold">{label}</span>
        <div className="flex items-center gap-4">
          <span className="font-mono font-bold">{formatAmount(current)}</span>
          {comparing && (
            <>
              <span className="text-sm text-muted-foreground">vs</span>
              <span className="font-mono text-sm">{formatAmount(comp ?? '0')}</span>
              <span className={`font-mono text-sm font-bold ${v >= 0 ? 'text-green-600' : 'text-red-600'}`}>
                ({v >= 0 ? '+' : ''}{formatAmount(v.toFixed(2))})
              </span>
            </>
          )}
        </div>
      </CardContent>
    </Card>
  );
}
