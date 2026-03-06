import { useMemo } from 'react';
import { useApAging } from '@/hooks/useApi';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Skeleton } from '@/components/ui/skeleton';
import {
  Table, TableBody, TableCell, TableHead, TableHeader, TableRow,
} from '@/components/ui/table';
import { BarChart, Bar, XAxis, YAxis, Tooltip, ResponsiveContainer } from 'recharts';
import { StickyToolbar, type ToolbarAction, type ToolbarOverflowItem } from '@/components/ui/sticky-toolbar';
import { formatAmount } from '@/lib/format';
import { downloadCsv } from '@/lib/export';
import { Download, RefreshCw } from 'lucide-react';

export function ApAgingPage() {
  const { data, isLoading, refetch } = useApAging();

  const chartData = (data ?? []).map((b) => ({
    bucket: b.bucket,
    total: parseFloat(b.total),
    count: b.count,
  }));

  const grandTotal = (data ?? []).reduce((sum, b) => sum + parseFloat(b.total || '0'), 0);
  const grandCount = (data ?? []).reduce((sum, b) => sum + b.count, 0);
  const hasData = grandCount > 0;

  const actions = useMemo<ToolbarAction[]>(() => [
    { icon: <RefreshCw className="h-4 w-4" />, label: 'Generate', onClick: () => refetch(), disabled: isLoading, loading: isLoading, primary: true },
  ], [isLoading]);

  const overflow = useMemo<ToolbarOverflowItem[]>(() => [
    { icon: <Download className="h-4 w-4" />, label: 'Export CSV', onClick: () => downloadCsv('/reports/ap-aging', {}) },
  ], []);

  return (
    <div className="space-y-4">
      <StickyToolbar actions={actions} overflow={overflow}>
        <span className="text-sm text-muted-foreground">Accounts Payable Aging</span>
      </StickyToolbar>

      {isLoading ? (
        <Skeleton className="h-64 w-full" />
      ) : data ? (
        <div className="space-y-4">
          {hasData && (
            <Card>
              <CardHeader className="pb-2">
                <CardTitle className="text-base">Distribution</CardTitle>
              </CardHeader>
              <CardContent>
                <ResponsiveContainer width="100%" height={200}>
                  <BarChart data={chartData}>
                    <XAxis dataKey="bucket" tick={{ fontSize: 11 }} />
                    <YAxis tick={{ fontSize: 11 }} />
                    <Tooltip
                      formatter={(value: number | string | undefined) =>
                        new Intl.NumberFormat('de-CH', { minimumFractionDigits: 2 }).format(
                          typeof value === 'number' ? value : parseFloat(String(value ?? 0)),
                        )
                      }
                    />
                    <Bar dataKey="total" fill="#f97316" radius={[3, 3, 0, 0]} />
                  </BarChart>
                </ResponsiveContainer>
              </CardContent>
            </Card>
          )}

          <Card>
            <CardContent className="p-0">
              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead>Bucket</TableHead>
                    <TableHead className="text-right">Count</TableHead>
                    <TableHead className="text-right">Total Amount</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {data.map((b) => (
                    <TableRow key={b.bucket}>
                      <TableCell className="font-medium">{b.bucket}</TableCell>
                      <TableCell className="text-right">{b.count}</TableCell>
                      <TableCell className="text-right font-mono text-sm">
                        {formatAmount(b.total)}
                      </TableCell>
                    </TableRow>
                  ))}
                  <TableRow className="bg-muted/50 font-bold">
                    <TableCell>Total</TableCell>
                    <TableCell className="text-right">{grandCount}</TableCell>
                    <TableCell className="text-right font-mono text-sm">
                      {formatAmount(grandTotal.toFixed(2))}
                    </TableCell>
                  </TableRow>
                </TableBody>
              </Table>
            </CardContent>
          </Card>

          {!hasData && (
            <Card>
              <CardContent className="py-4">
                <p className="text-center text-sm text-muted-foreground">
                  No payable data available. AP aging requires the expenses module.
                </p>
              </CardContent>
            </Card>
          )}
        </div>
      ) : (
        <Card>
          <CardContent className="py-8">
            <p className="text-center text-sm text-muted-foreground">
              Click &quot;Generate&quot; to view AP aging.
            </p>
          </CardContent>
        </Card>
      )}
    </div>
  );
}
