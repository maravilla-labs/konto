import { Card, CardContent } from '@/components/ui/card';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { Clock, DollarSign, FileText, TrendingUp, Ban, Receipt } from 'lucide-react';
import { SUPPORTED_LANGUAGES } from '@/lib/language';
import { useI18n } from '@/i18n';
import { BudgetTracker } from './BudgetTracker';

interface ProjectOverviewTabProps {
  summary: {
    language?: string;
    soft_budget_hours?: number;
    hard_budget_hours?: number;
    soft_budget_amount?: number;
    hard_budget_amount?: number;
  };
  totalHours: number;
  billableHours: number;
  budgetHours: number | null;
  totalInvoiced: number;
  nonBillableHours?: number;
  unbilledAmount?: number;
  actualAmount?: number;
  onLanguageChange: (v: string) => void;
}

export function ProjectOverviewTab({
  summary, totalHours: rawTotalHours, billableHours: rawBillableHours, budgetHours: rawBudgetHours, totalInvoiced: rawTotalInvoiced, nonBillableHours: rawNonBillableHours = 0, unbilledAmount: rawUnbilledAmount = 0, actualAmount: rawActualAmount = 0, onLanguageChange,
}: ProjectOverviewTabProps) {
  const { t } = useI18n();
  const totalHours = Number(rawTotalHours) || 0;
  const billableHours = Number(rawBillableHours) || 0;
  const budgetHours = Number(rawBudgetHours) || 0;
  const totalInvoiced = Number(rawTotalInvoiced) || 0;
  const nonBillableHours = Number(rawNonBillableHours) || 0;
  const unbilledAmount = Number(rawUnbilledAmount) || 0;
  const actualAmount = Number(rawActualAmount) || 0;

  return (
    <div className="space-y-4 pt-2">
      <Card>
        <CardContent className="pt-4">
          <div className="max-w-xs space-y-2">
            <span className="text-sm text-muted-foreground">{t('projects.preferred_language', 'Preferred Language')}</span>
            <Select
              value={(summary.language as string) || '__auto__'}
              onValueChange={(v) => onLanguageChange(v === '__auto__' ? '' : v)}
            >
              <SelectTrigger><SelectValue placeholder={t('common.automatic', 'Automatic')} /></SelectTrigger>
              <SelectContent>
                <SelectItem value="__auto__">{t('common.automatic', 'Automatic')}</SelectItem>
                {SUPPORTED_LANGUAGES.map((lang) => (
                  <SelectItem key={lang.code} value={lang.code}>{lang.label}</SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
        </CardContent>
      </Card>

      <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-4">
        <Card>
          <CardContent className="pt-4">
            <div className="flex items-center gap-2 text-sm text-muted-foreground mb-1">
              <Clock className="h-4 w-4" /> {t('projects.total_hours', 'Total Hours')}
            </div>
            <p className="text-2xl font-bold">{totalHours.toFixed(1)}</p>
          </CardContent>
        </Card>
        <Card>
          <CardContent className="pt-4">
            <div className="flex items-center gap-2 text-sm text-muted-foreground mb-1">
              <TrendingUp className="h-4 w-4" /> {t('projects.billable_hours', 'Billable Hours')}
            </div>
            <p className="text-2xl font-bold">{billableHours.toFixed(1)}</p>
          </CardContent>
        </Card>
        <Card>
          <CardContent className="pt-4">
            <div className="flex items-center gap-2 text-sm text-muted-foreground mb-1">
              <Ban className="h-4 w-4" /> {t('project_budget.non_billable_hours', 'Non-Billable Hours')}
            </div>
            <p className="text-2xl font-bold">{nonBillableHours.toFixed(1)}</p>
          </CardContent>
        </Card>
        <Card>
          <CardContent className="pt-4">
            <div className="flex items-center gap-2 text-sm text-muted-foreground mb-1">
              <DollarSign className="h-4 w-4" /> {t('projects.invoiced', 'Invoiced')}
            </div>
            <p className="text-2xl font-bold">CHF {totalInvoiced.toFixed(0)}</p>
          </CardContent>
        </Card>
        <Card>
          <CardContent className="pt-4">
            <div className="flex items-center gap-2 text-sm text-muted-foreground mb-1">
              <Receipt className="h-4 w-4" /> {t('project_budget.unbilled_amount', 'Unbilled Amount')}
            </div>
            <p className="text-2xl font-bold">CHF {unbilledAmount.toFixed(0)}</p>
          </CardContent>
        </Card>
        <Card>
          <CardContent className="pt-4">
            <div className="flex items-center gap-2 text-sm text-muted-foreground mb-1">
              <FileText className="h-4 w-4" /> {t('projects.budget', 'Budget')}
            </div>
            <p className="text-2xl font-bold">{budgetHours ? `${budgetHours.toFixed(0)}h` : '—'}</p>
          </CardContent>
        </Card>
      </div>

      <BudgetTracker
        currentHours={totalHours}
        softBudgetHours={summary.soft_budget_hours as number | undefined}
        hardBudgetHours={summary.hard_budget_hours as number | undefined}
        currentAmount={actualAmount}
        softBudgetAmount={summary.soft_budget_amount as number | undefined}
        hardBudgetAmount={summary.hard_budget_amount as number | undefined}
      />
    </div>
  );
}
