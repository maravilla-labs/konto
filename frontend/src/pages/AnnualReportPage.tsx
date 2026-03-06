import { useState } from 'react';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Skeleton } from '@/components/ui/skeleton';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { useQuery } from '@tanstack/react-query';
import { fiscalYearsApi } from '@/api/fiscal-years';
import {
  useAnnualReport,
  useSwissBalanceSheet,
  useSwissIncomeStatement,
  useGenerateAnnualReport,
  useFinalizeAnnualReport,
} from '@/hooks/useAnnualReportApi';
import { annualReportApi } from '@/api/annual-report';
import { SwissBalanceSheetPreview } from '@/components/reports/SwissBalanceSheetPreview';
import { SwissIncomeStatementPreview } from '@/components/reports/SwissIncomeStatementPreview';
import { AnnualReportNotesEditor } from '@/components/reports/AnnualReportNotesEditor';
import { toast } from 'sonner';
import { FileText, Download, CheckCircle, Loader2 } from 'lucide-react';
import type { FiscalYear } from '@/types/fiscal-year';

export function AnnualReportPage() {
  const { data: fiscalYears, isLoading: fyLoading } = useQuery({
    queryKey: ['fiscal-years'],
    queryFn: () => fiscalYearsApi.list({ per_page: 100 }).then((r) => r.data.data),
  });

  const [selectedFyId, setSelectedFyId] = useState<string>('');
  const selectedFy = fiscalYears?.find((fy: FiscalYear) => fy.id === selectedFyId);

  const { data: report } = useAnnualReport(selectedFyId || undefined);
  const { data: balanceSheet } = useSwissBalanceSheet(
    selectedFy ? selectedFy.end_date : undefined,
  );
  const { data: incomeStatement } = useSwissIncomeStatement(
    selectedFy ? selectedFy.start_date : undefined,
    selectedFy ? selectedFy.end_date : undefined,
  );

  const generatePdf = useGenerateAnnualReport();
  const finalize = useFinalizeAnnualReport();

  function handleGenerate() {
    if (!selectedFyId) return;
    generatePdf.mutate(selectedFyId, {
      onSuccess: () => toast.success('PDF generated'),
      onError: () => toast.error('Failed to generate PDF'),
    });
  }

  async function handleDownload() {
    if (!selectedFyId) return;
    try {
      const res = await annualReportApi.downloadPdf(selectedFyId);
      const blob = new Blob([res.data], { type: 'application/pdf' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `jahresrechnung_${selectedFy?.name ?? selectedFyId}.pdf`;
      a.click();
      URL.revokeObjectURL(url);
    } catch {
      toast.error('Failed to download PDF');
    }
  }

  function handleFinalize() {
    if (!selectedFyId) return;
    finalize.mutate(selectedFyId, {
      onSuccess: () => toast.success('Report finalized'),
      onError: () => toast.error('Failed to finalize'),
    });
  }

  return (
    <div className="space-y-4">
      <div className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h2 className="text-lg font-semibold">Jahresrechnung</h2>
          <p className="text-sm text-muted-foreground">
            Annual Financial Statements (OR Art. 957-962)
          </p>
        </div>
      </div>

      {/* Fiscal Year Selector */}
      <Card>
        <CardContent className="flex flex-col gap-4 p-4 sm:flex-row sm:items-center sm:justify-between">
          <div className="flex items-center gap-3">
            <label className="text-sm font-medium">Fiscal Year:</label>
            {fyLoading ? (
              <Skeleton className="h-9 w-48" />
            ) : (
              <Select value={selectedFyId} onValueChange={setSelectedFyId}>
                <SelectTrigger className="w-60">
                  <SelectValue placeholder="Select fiscal year" />
                </SelectTrigger>
                <SelectContent>
                  {(fiscalYears ?? []).map((fy: FiscalYear) => (
                    <SelectItem key={fy.id} value={fy.id}>
                      {fy.name} ({fy.start_date} - {fy.end_date})
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            )}
          </div>

          {report && (
            <div className="flex items-center gap-2">
              <Badge variant={report.status === 'finalized' ? 'default' : 'secondary'}>
                {report.status === 'finalized' ? 'Finalized' : 'Draft'}
              </Badge>
              {report.generated_at && (
                <span className="text-xs text-muted-foreground">
                  Generated: {new Date(report.generated_at).toLocaleDateString()}
                </span>
              )}
            </div>
          )}
        </CardContent>
      </Card>

      {selectedFyId && (
        <>
          {/* Action Buttons */}
          <div className="flex flex-wrap gap-2">
            <Button onClick={handleDownload} variant="outline" size="sm">
              <Download className="mr-1 h-4 w-4" /> Download PDF
            </Button>
            <Button
              onClick={handleGenerate}
              size="sm"
              disabled={generatePdf.isPending}
            >
              {generatePdf.isPending ? (
                <Loader2 className="mr-1 h-4 w-4 animate-spin" />
              ) : (
                <FileText className="mr-1 h-4 w-4" />
              )}
              Generate PDF
            </Button>
            {report?.status !== 'finalized' && (
              <Button
                onClick={handleFinalize}
                variant="default"
                size="sm"
                disabled={finalize.isPending}
              >
                <CheckCircle className="mr-1 h-4 w-4" /> Finalize
              </Button>
            )}
          </div>

          {/* Tabbed Content */}
          <Tabs defaultValue="balance-sheet">
            <TabsList>
              <TabsTrigger value="balance-sheet">Bilanz</TabsTrigger>
              <TabsTrigger value="income-statement">Erfolgsrechnung</TabsTrigger>
              <TabsTrigger value="notes">Anhang</TabsTrigger>
            </TabsList>

            <TabsContent value="balance-sheet" className="mt-4">
              {balanceSheet ? (
                <SwissBalanceSheetPreview data={balanceSheet} />
              ) : (
                <Skeleton className="h-96 w-full" />
              )}
            </TabsContent>

            <TabsContent value="income-statement" className="mt-4">
              {incomeStatement ? (
                <SwissIncomeStatementPreview data={incomeStatement} />
              ) : (
                <Skeleton className="h-96 w-full" />
              )}
            </TabsContent>

            <TabsContent value="notes" className="mt-4">
              <AnnualReportNotesEditor fiscalYearId={selectedFyId} />
            </TabsContent>
          </Tabs>
        </>
      )}
    </div>
  );
}
