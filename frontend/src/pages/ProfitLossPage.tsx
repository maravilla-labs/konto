import { useState, useMemo } from 'react';
import { useProfitLoss } from '@/hooks/useApi';
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

function firstOfMonth(): string {
  const d = new Date();
  return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-01`;
}

function lastOfMonth(): string {
  const d = new Date(new Date().getFullYear(), new Date().getMonth() + 1, 0);
  return d.toISOString().split('T')[0];
}

export function ProfitLossPage() {
  const [from, setFrom] = useState(firstOfMonth());
  const [to, setTo] = useState(lastOfMonth());
  const [compFrom, setCompFrom] = useState('');
  const [compTo, setCompTo] = useState('');

  const { data, isLoading, refetch } = useProfitLoss({ from_date: from, to_date: to });
  const { data: compData, refetch: compRefetch } = useProfitLoss(
    compFrom && compTo ? { from_date: compFrom, to_date: compTo } : undefined,
  );

  const comparing = !!(compFrom && compTo && compData);
  const netNum = parseFloat(data?.net_income || '0');

  const handleGenerate = () => {
    refetch();
    if (compFrom && compTo) compRefetch();
  };

  const actions = useMemo<ToolbarAction[]>(() => [
    { icon: <RefreshCw className="h-4 w-4" />, label: 'Generate', onClick: handleGenerate, disabled: isLoading, loading: isLoading, primary: true },
  ], [isLoading]);

  const overflow = useMemo<ToolbarOverflowItem[]>(() => [
    { icon: <Download className="h-4 w-4" />, label: 'Export CSV', onClick: () => downloadCsv('/reports/profit-loss', { from_date: from, to_date: to }) },
  ], [from, to]);

  return (
    <div className="space-y-4">
      <StickyToolbar actions={actions} overflow={overflow}>
        <div className="flex items-center gap-1">
          <Label className="text-xs text-muted-foreground">From</Label>
          <Input type="date" value={from} onChange={(e) => setFrom(e.target.value)} className="h-8 text-sm w-36" />
        </div>
        <div className="flex items-center gap-1">
          <Label className="text-xs text-muted-foreground">To</Label>
          <Input type="date" value={to} onChange={(e) => setTo(e.target.value)} className="h-8 text-sm w-36" />
        </div>
        <div className="flex items-center gap-1">
          <Label className="text-xs text-muted-foreground">Compare From</Label>
          <Input type="date" value={compFrom} onChange={(e) => setCompFrom(e.target.value)} className="h-8 text-sm w-36" />
        </div>
        <div className="flex items-center gap-1">
          <Label className="text-xs text-muted-foreground">Compare To</Label>
          <Input type="date" value={compTo} onChange={(e) => setCompTo(e.target.value)} className="h-8 text-sm w-36" />
        </div>
      </StickyToolbar>

      {isLoading ? (
        <div className="space-y-2">
          {Array.from({ length: 2 }).map((_, i) => <Skeleton key={i} className="h-32 w-full" />)}
        </div>
      ) : data ? (
        <div className="space-y-4">
          <PLSection title="Revenue" accounts={data.revenue} total={data.total_revenue}
            compAccounts={compData?.revenue} compTotal={compData?.total_revenue} comparing={comparing} />
          <PLSection title="Expenses" accounts={data.expenses} total={data.total_expenses}
            compAccounts={compData?.expenses} compTotal={compData?.total_expenses} comparing={comparing} />
          <NetResultCard current={data.net_income} comparison={compData?.net_income} comparing={comparing} netNum={netNum} />
        </div>
      ) : (
        <Card><CardContent className="py-8"><p className="text-center text-sm text-muted-foreground">Click &quot;Generate&quot; to view profit &amp; loss.</p></CardContent></Card>
      )}
    </div>
  );
}

type Acct = { account_id: string; account_number: number; account_name: string; balance: string };

function PLSection({ title, accounts, total, compAccounts, compTotal, comparing }: {
  title: string; accounts: Acct[]; total: string;
  compAccounts?: Acct[]; compTotal?: string; comparing: boolean;
}) {
  const compMap = new Map((compAccounts ?? []).map((a) => [a.account_id, a]));
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
              const v = comp ? parseFloat(a.balance) - parseFloat(comp.balance) : 0;
              const pct = comp && parseFloat(comp.balance) !== 0 ? (v / Math.abs(parseFloat(comp.balance))) * 100 : 0;
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
            <TotalRow label={`Total ${title}`} current={total} comp={compTotal} comparing={comparing} />
          </TableBody>
        </Table>
      </CardContent>
    </Card>
  );
}

function TotalRow({ label, current, comp, comparing }: { label: string; current: string; comp?: string; comparing: boolean }) {
  const v = comp ? parseFloat(current) - parseFloat(comp) : 0;
  const pct = comp && parseFloat(comp) !== 0 ? (v / Math.abs(parseFloat(comp))) * 100 : 0;
  return (
    <TableRow className="bg-muted/50 font-bold">
      <TableCell colSpan={2}>{label}</TableCell>
      <TableCell className="text-right font-mono text-sm">{formatAmount(current)}</TableCell>
      {comparing && <TableCell className="text-right font-mono text-sm">{formatAmount(comp ?? '0')}</TableCell>}
      {comparing && <TableCell className={`text-right font-mono text-sm ${v >= 0 ? 'text-green-600' : 'text-red-600'}`}>{formatAmount(v.toFixed(2))}</TableCell>}
      {comparing && <TableCell className={`text-right font-mono text-sm ${v >= 0 ? 'text-green-600' : 'text-red-600'}`}>{pct.toFixed(1)}%</TableCell>}
    </TableRow>
  );
}

function NetResultCard({ current, comparison, comparing, netNum }: { current: string; comparison?: string; comparing: boolean; netNum: number }) {
  const compNum = parseFloat(comparison ?? '0');
  const variance = netNum - compNum;
  return (
    <Card>
      <CardContent className="flex items-center justify-between py-4">
        <span className="text-lg font-semibold">Net Result</span>
        <div className="flex items-center gap-6">
          <span className={`font-mono text-lg font-bold ${netNum >= 0 ? 'text-green-600' : 'text-red-600'}`}>{formatAmount(current)}</span>
          {comparing && (
            <>
              <span className="text-sm text-muted-foreground">vs</span>
              <span className={`font-mono text-sm ${compNum >= 0 ? 'text-green-600' : 'text-red-600'}`}>{formatAmount(comparison ?? '0')}</span>
              <span className={`font-mono text-sm font-bold ${variance >= 0 ? 'text-green-600' : 'text-red-600'}`}>({variance >= 0 ? '+' : ''}{formatAmount(variance.toFixed(2))})</span>
            </>
          )}
        </div>
      </CardContent>
    </Card>
  );
}
