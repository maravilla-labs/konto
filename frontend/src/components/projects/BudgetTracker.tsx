import { Card, CardContent } from '@/components/ui/card';
import { Progress } from '@/components/ui/progress';
import { useI18n } from '@/i18n';

interface BudgetTrackerProps {
  currentHours?: number;
  softBudgetHours?: number;
  hardBudgetHours?: number;
  currentAmount?: number;
  softBudgetAmount?: number;
  hardBudgetAmount?: number;
}

function getBarColor(current: number, soft?: number, hard?: number): string {
  if (hard && current >= hard) return '[&>div]:bg-red-500';
  if (soft && current >= soft) return '[&>div]:bg-yellow-500';
  return '[&>div]:bg-green-500';
}

function getPercent(current: number, max?: number): number {
  if (!max || max === 0) return 0;
  return Math.min(100, (current / max) * 100);
}

export function BudgetTracker({
  currentHours = 0,
  softBudgetHours,
  hardBudgetHours,
  currentAmount = 0,
  softBudgetAmount,
  hardBudgetAmount,
}: BudgetTrackerProps) {
  const { t } = useI18n();

  const softHours = Number(softBudgetHours) || 0;
  const hardHours = Number(hardBudgetHours) || 0;
  const softAmount = Number(softBudgetAmount) || 0;
  const hardAmount = Number(hardBudgetAmount) || 0;

  const hasHoursBudget = softHours > 0 || hardHours > 0;
  const hasAmountBudget = softAmount > 0 || hardAmount > 0;

  if (!hasHoursBudget && !hasAmountBudget) return null;

  const hoursMax = hardHours || softHours;
  const hoursPct = getPercent(currentHours, hoursMax);
  const hoursColor = getBarColor(currentHours, softHours || undefined, hardHours || undefined);

  const amountMax = hardAmount || softAmount;
  const amountPct = getPercent(currentAmount, amountMax);
  const amountColor = getBarColor(currentAmount, softAmount || undefined, hardAmount || undefined);

  return (
    <Card>
      <CardContent className="pt-4 space-y-4">
        <h3 className="text-sm font-semibold">
          {t('projects.budget_tracker', 'Budget Tracker')}
        </h3>

        {hasHoursBudget && (
          <div className="space-y-1">
            <div className="flex justify-between text-sm">
              <span className="text-muted-foreground">
                {t('projects.hours_budget', 'Hours Budget')}
              </span>
              <span className="font-medium">
                {currentHours.toFixed(1)}h / {hoursMax.toFixed(0)}h ({hoursPct.toFixed(0)}%)
              </span>
            </div>
            <Progress value={hoursPct} className={hoursColor} />
            {softHours > 0 && hardHours > 0 && (
              <div className="flex justify-between text-xs text-muted-foreground">
                <span>{t('projects.soft_limit', 'Soft')}: {softHours}h</span>
                <span>{t('projects.hard_limit', 'Hard')}: {hardHours}h</span>
              </div>
            )}
          </div>
        )}

        {hasAmountBudget && (
          <div className="space-y-1">
            <div className="flex justify-between text-sm">
              <span className="text-muted-foreground">
                {t('projects.amount_budget', 'Amount Budget')}
              </span>
              <span className="font-medium">
                CHF {currentAmount.toFixed(0)} / CHF {amountMax.toFixed(0)} ({amountPct.toFixed(0)}%)
              </span>
            </div>
            <Progress value={amountPct} className={amountColor} />
            {softAmount > 0 && hardAmount > 0 && (
              <div className="flex justify-between text-xs text-muted-foreground">
                <span>{t('projects.soft_limit', 'Soft')}: CHF {softAmount}</span>
                <span>{t('projects.hard_limit', 'Hard')}: CHF {hardAmount}</span>
              </div>
            )}
          </div>
        )}
      </CardContent>
    </Card>
  );
}
