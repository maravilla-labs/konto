import { useState, useMemo } from 'react';
import { useCashFlow, useMonthlyCashFlow } from '@/hooks/useApi';
import { useSettings } from '@/hooks/useSettingsApi';
import { useI18n } from '@/i18n';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Skeleton } from '@/components/ui/skeleton';
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from '@/components/ui/collapsible';
import { StickyToolbar, type ToolbarAction, type ToolbarOverflowItem } from '@/components/ui/sticky-toolbar';
import { formatAmount } from '@/lib/format';
import { formatCurrency } from '@/lib/locale';
import { downloadCsv } from '@/lib/export';
import { Download, ChevronDown, ArrowRight, Check, AlertTriangle, RefreshCw } from 'lucide-react';
import { SparklineCard } from '@/components/cash-flow/SparklineCard';
import { CashFlowChart } from '@/components/cash-flow/CashFlowChart';
import type { CashFlowSection } from '@/types/dashboard-charts';

function yearStart(): string {
  return `${new Date().getFullYear()}-01-01`;
}

function today(): string {
  return new Date().toISOString().split('T')[0];
}

export function CashFlowPage() {
  const { t } = useI18n();
  const { data: settings } = useSettings();
  const numberFormat = settings?.number_format ?? 'ch';
  const currency = settings?.default_currency_id ?? 'CHF';

  const [from, setFrom] = useState(yearStart());
  const [to, setTo] = useState(today());
  const { data, isLoading, refetch } = useCashFlow({ from_date: from, to_date: to });
  const {
    data: monthlyData,
    isLoading: monthlyLoading,
    refetch: refetchMonthly,
  } = useMonthlyCashFlow({ from_date: from, to_date: to });

  const handleGenerate = () => {
    refetch();
    refetchMonthly();
  };

  const hasData = data && monthlyData;
  const loading = isLoading || monthlyLoading;

  const actions = useMemo<ToolbarAction[]>(() => [
    {
      icon: <RefreshCw className="h-4 w-4" />,
      label: t('cash_flow.generate', 'Generate'),
      onClick: handleGenerate,
      primary: true,
      disabled: loading,
      loading,
    },
  ], [loading, t]);

  const overflow = useMemo<ToolbarOverflowItem[]>(() => [
    {
      icon: <Download className="h-4 w-4" />,
      label: t('cash_flow.export_csv', 'Export CSV'),
      onClick: () => downloadCsv('/reports/cash-flow', { from_date: from, to_date: to }),
    },
  ], [from, to, t]);

  const inflowsData = (monthlyData?.months ?? []).map((m) => ({
    value: parseFloat(m.inflows),
  }));
  const outflowsData = (monthlyData?.months ?? []).map((m) => ({
    value: Math.abs(parseFloat(m.outflows)),
  }));
  const variationData = (monthlyData?.months ?? []).map((m) => ({
    value: parseFloat(m.net),
  }));

  return (
    <div className="space-y-4">
      {/* Row 1: Sparkline KPI Cards */}
      {hasData && (
        <div className="grid grid-cols-1 gap-3 md:grid-cols-3">
          <SparklineCard
            label={t('cash_flow.inflows', 'Inflows')}
            value={formatCurrency(monthlyData.total_inflows, currency, numberFormat)}
            data={inflowsData}
            color="#22c55e"
            fillColor="#22c55e"
          />
          <SparklineCard
            label={t('cash_flow.outflows', 'Outflows')}
            value={formatCurrency(monthlyData.total_outflows, currency, numberFormat)}
            data={outflowsData}
            color="#ef4444"
            fillColor="#ef4444"
          />
          <SparklineCard
            label={t('cash_flow.variation', 'Variation')}
            value={formatCurrency(monthlyData.net_variation, currency, numberFormat)}
            data={variationData}
            color="#3b82f6"
            fillColor="#3b82f6"
          />
        </div>
      )}

      {/* Toolbar */}
      <StickyToolbar actions={actions} overflow={overflow}>
        <Input
          type="date"
          value={from}
          onChange={(e) => setFrom(e.target.value)}
          className="h-8 w-36 text-sm"
        />
        <span className="text-xs text-muted-foreground">&ndash;</span>
        <Input
          type="date"
          value={to}
          onChange={(e) => setTo(e.target.value)}
          className="h-8 w-36 text-sm"
        />
        {hasData && (
          <div className="flex items-center gap-1.5 text-sm">
            <span className="font-mono font-semibold text-muted-foreground">
              {formatCurrency(monthlyData.initial_balance, currency, numberFormat)}
            </span>
            <ArrowRight className="h-3.5 w-3.5 text-muted-foreground" />
            <span
              className={`font-mono font-semibold ${
                parseFloat(monthlyData.ending_balance) >= parseFloat(monthlyData.initial_balance)
                  ? 'text-green-600'
                  : 'text-red-600'
              }`}
            >
              {formatCurrency(monthlyData.ending_balance, currency, numberFormat)}
            </span>
          </div>
        )}
      </StickyToolbar>

      {/* Loading skeleton */}
      {loading && (
        <div className="space-y-2">
          {Array.from({ length: 3 }).map((_, i) => (
            <Skeleton key={i} className="h-32 w-full" />
          ))}
        </div>
      )}

      {/* Row 3: Monthly Chart */}
      {!loading && monthlyData && monthlyData.months.length > 0 && (
        <Card>
          <CardContent className="pt-6">
            <CashFlowChart months={monthlyData.months} />
          </CardContent>
        </Card>
      )}

      {/* Row 4: Detailed sections */}
      {!loading && data ? (
        <div className="space-y-4">
          {data.sections.map((section) => (
            <CashFlowSectionCard key={section.label} section={section} />
          ))}

          {/* Reconciliation block */}
          <Card>
            <CardContent className="space-y-2 py-4">
              {/* Total Explained (sum of 3 sections) */}
              <div className="flex items-center justify-between">
                <span className="text-lg font-semibold">
                  {t('cash_flow.total_explained', 'Total Explained')}
                </span>
                <span
                  className={`font-mono text-lg font-bold ${
                    parseFloat(data.net_change) >= 0 ? 'text-green-600' : 'text-red-600'
                  }`}
                >
                  {formatAmount(data.net_change)}
                </span>
              </div>

              <div className="border-t pt-2 space-y-1 text-sm">
                {/* Opening */}
                <div className="flex justify-between">
                  <span className="text-muted-foreground">
                    {t('cash_flow.initial_balance', 'Opening Balance')}
                  </span>
                  <span className="font-mono">{formatAmount(data.opening_balance)}</span>
                </div>
                {/* + Total Explained = Expected Closing */}
                <div className="flex justify-between">
                  <span className="text-muted-foreground">
                    + {t('cash_flow.total_explained', 'Total Explained')}
                  </span>
                  <span className="font-mono">{formatAmount(data.net_change)}</span>
                </div>
                <div className="flex justify-between font-medium">
                  <span>
                    = {t('cash_flow.expected_closing', 'Expected Closing')}
                  </span>
                  <span className="font-mono">
                    {formatAmount(
                      (parseFloat(data.opening_balance) + parseFloat(data.net_change)).toFixed(2),
                    )}
                  </span>
                </div>
              </div>

              <div className="border-t pt-2 space-y-1 text-sm">
                {/* Actual Closing */}
                <div className="flex justify-between font-medium">
                  <span>{t('cash_flow.actual_closing', 'Actual Closing Balance')}</span>
                  <span className="font-mono">{formatAmount(data.closing_balance)}</span>
                </div>
              </div>

              {/* Reconciliation Difference */}
              <div className="border-t pt-2">
                <ReconciliationLine
                  difference={parseFloat(data.reconciliation_difference)}
                />
              </div>
            </CardContent>
          </Card>
        </div>
      ) : (
        !loading && (
          <Card>
            <CardContent className="py-8">
              <p className="text-center text-sm text-muted-foreground">
                {t('cash_flow.generate', 'Generate')}
              </p>
            </CardContent>
          </Card>
        )
      )}
    </div>
  );
}

function ReconciliationLine({ difference }: { difference: number }) {
  const { t } = useI18n();
  const { data: settings } = useSettings();
  const numberFormat = settings?.number_format ?? 'ch';
  const currency = settings?.default_currency_id ?? 'CHF';
  const isZero = Math.abs(difference) < 0.005;

  if (isZero) {
    return (
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-2">
          <Check className="h-4 w-4 text-green-600" />
          <span className="text-sm font-medium text-green-600">
            {t('cash_flow.reconciled', 'Reconciled')}
          </span>
        </div>
        <span className="font-mono text-sm font-bold text-green-600">0.00</span>
      </div>
    );
  }

  return (
    <div className="flex items-center justify-between">
      <div className="flex items-center gap-2">
        <AlertTriangle className="h-4 w-4 text-red-600" />
        <span className="text-sm font-medium text-red-600">
          {t('cash_flow.reconciliation', 'Reconciliation Difference')}
        </span>
      </div>
      <span className="font-mono text-sm font-bold text-red-600" title={t('cash_flow.reconciliation_hint', 'May include system entries or unclassified movements')}>
        {formatCurrency(difference, currency, numberFormat)}
      </span>
    </div>
  );
}

function CashFlowSectionCard({ section }: { section: CashFlowSection }) {
  const { t } = useI18n();
  const [open, setOpen] = useState(true);
  const netNum = parseFloat(section.net);

  return (
    <Card>
      <Collapsible open={open} onOpenChange={setOpen}>
        <CollapsibleTrigger asChild>
          <CardHeader className="cursor-pointer pb-2">
            <div className="flex items-center justify-between">
              <CardTitle className="text-base">{section.label}</CardTitle>
              <div className="flex items-center gap-4">
                <span
                  className={`font-mono text-sm font-bold ${netNum >= 0 ? 'text-green-600' : 'text-red-600'}`}
                >
                  Net: {formatAmount(section.net)}
                </span>
                <ChevronDown
                  className={`h-4 w-4 transition-transform ${open ? 'rotate-180' : ''}`}
                />
              </div>
            </div>
          </CardHeader>
        </CollapsibleTrigger>
        <CollapsibleContent>
          <CardContent className="pt-0">
            {section.items.length === 0 ? (
              <p className="text-sm text-muted-foreground">
                {t('cash_flow.no_activity', 'No activity')}
              </p>
            ) : (
              <div className="space-y-1">
                {section.items.map((item, i) => (
                  <div
                    key={i}
                    className="flex items-center justify-between rounded px-2 py-1 text-sm"
                  >
                    <span className="text-muted-foreground">{item.description}</span>
                    <span
                      className={`font-mono ${parseFloat(item.amount) >= 0 ? 'text-green-600' : 'text-red-600'}`}
                    >
                      {formatAmount(item.amount)}
                    </span>
                  </div>
                ))}
              </div>
            )}
            <div className="mt-2 flex justify-between border-t pt-2 text-sm">
              <span className="text-muted-foreground">
                {t('cash_flow.inflows', 'Inflows')}
              </span>
              <span className="font-mono text-green-600">
                {formatAmount(section.inflows)}
              </span>
            </div>
            <div className="flex justify-between text-sm">
              <span className="text-muted-foreground">
                {t('cash_flow.outflows', 'Outflows')}
              </span>
              <span className="font-mono text-red-600">
                {formatAmount(section.outflows)}
              </span>
            </div>
          </CardContent>
        </CollapsibleContent>
      </Collapsible>
    </Card>
  );
}
