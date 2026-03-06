import { useState } from 'react';
import { Link, useParams, useNavigate } from 'react-router-dom';
import {
  useProjectSummary,
  useUpdateProject,
  useProjectSubStatuses,
  useProjectBudgetAnalytics,
} from '@/hooks/useApi';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { MarkdownPreview } from '@/components/ui/markdown-preview';
import { Skeleton } from '@/components/ui/skeleton';
import { Tabs, TabsList, TabsTrigger, TabsContent } from '@/components/ui/tabs';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { ArrowLeft, Pencil, ReceiptText } from 'lucide-react';
import { toast } from 'sonner';
import { useI18n } from '@/i18n';
import { ProjectOverviewTab } from '@/components/projects/ProjectOverviewTab';
import { ProjectItemsTree } from '@/components/projects/ProjectItemsTree';
import { ProjectMembersTab } from '@/components/projects/ProjectMembersTab';
import { ProjectMilestonesTab } from '@/components/projects/ProjectMilestonesTab';
import { ProjectDocumentsTab } from '@/components/projects/ProjectDocumentsTab';
import { ProjectBillingTab } from '@/components/projects/ProjectBillingTab';
import { ProjectTimeTrackingTab } from '@/components/projects/ProjectTimeTrackingTab';
import { ProjectSettingsTab } from '@/components/projects/ProjectSettingsTab';
import { ProjectEditDialog } from '@/components/projects/ProjectEditDialog';
import { BudgetAnalyticsChart } from '@/components/projects/BudgetAnalyticsChart';
import { BudgetBreakdownTable } from '@/components/projects/BudgetBreakdownTable';

const VALID_TABS = ['overview', 'wbs', 'members', 'milestones', 'documents', 'time-tracking', 'billing', 'settings'] as const;

const statusVariant: Record<string, 'default' | 'secondary' | 'destructive' | 'outline'> = {
  active: 'default',
  completed: 'secondary',
  on_hold: 'outline',
  cancelled: 'destructive',
};

export function ProjectDetailPage() {
  const { t } = useI18n();
  const navigate = useNavigate();
  const { id, tab } = useParams<{ id: string; tab?: string }>();
  const activeTab = VALID_TABS.includes(tab as typeof VALID_TABS[number]) ? tab! : 'overview';
  const { data: summary, isLoading } = useProjectSummary(id);
  const updateProject = useUpdateProject();
  const { data: subStatuses } = useProjectSubStatuses();
  const { data: budgetAnalytics, isLoading: budgetLoading } = useProjectBudgetAnalytics(id);
  const [editOpen, setEditOpen] = useState(false);

  function handleTabChange(value: string) {
    const path = value === 'overview' ? `/projects/${id}` : `/projects/${id}/${value}`;
    navigate(path, { replace: true });
  }

  if (isLoading) {
    return (
      <div className="space-y-4">
        <Skeleton className="h-8 w-48" />
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
          {Array.from({ length: 4 }).map((_, i) => <Skeleton key={i} className="h-24" />)}
        </div>
      </div>
    );
  }

  if (!summary) {
    return <p className="text-center text-muted-foreground py-8">{t('projects.not_found', 'Project not found.')}</p>;
  }

  const totalHours = Number(summary.total_hours) || 0;
  const billableHours = Number(summary.billable_hours) || 0;
  const budgetHours = budgetAnalytics?.budget_hours
    ? Number(budgetAnalytics.budget_hours)
    : (summary.budget_hours ? Number(summary.budget_hours) : null);
  const totalInvoiced = Number(summary.total_invoiced) || 0;

  function handleLanguageChange(next: string) {
    if (!id) return;
    updateProject.mutate(
      { id, data: { language: next || undefined } },
      {
        onSuccess: () => toast.success(t('projects.language_updated', 'Project language updated')),
        onError: () => toast.error(t('projects.language_update_failed', 'Failed to update project language')),
      },
    );
  }

  return (
    <div className="space-y-4">
      <div className="flex items-center gap-3">
        <Link to="/projects">
          <Button variant="ghost" size="icon"><ArrowLeft className="h-4 w-4" /></Button>
        </Link>
        <div>
          <div className="flex items-center gap-2">
            {summary.number && <span className="font-mono text-muted-foreground">{summary.number}</span>}
            <h2 className="text-lg font-semibold">{summary.name}</h2>
          </div>
          {summary.contact_name && <p className="text-sm text-muted-foreground">{summary.contact_name}</p>}
        </div>
        <Badge variant={statusVariant[summary.status] ?? 'outline'} className="ml-2">
          {t(`projects.status.${summary.status}`, summary.status.replace('_', ' '))}
        </Badge>
        {(() => {
          const currentSub = (subStatuses ?? []).find(s => s.id === summary.sub_status_id);
          return (
            <Select
              value={summary.sub_status_id ?? '__none__'}
              onValueChange={(v) => {
                if (!id) return;
                updateProject.mutate(
                  { id, data: { sub_status_id: v === '__none__' ? undefined : v } as Record<string, unknown> },
                  { onSuccess: () => toast.success(t('projects.project_updated', 'Project updated')) },
                );
              }}
            >
              <SelectTrigger className="h-7 w-auto min-w-[120px] text-xs">
                <SelectValue>
                  {currentSub ? (
                    <span className="flex items-center gap-1.5">
                      <span className="h-2 w-2 rounded-full" style={{ backgroundColor: currentSub.color }} />
                      {currentSub.name}
                    </span>
                  ) : (
                    t('project_conditions.sub_status', 'Sub-Status')
                  )}
                </SelectValue>
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="__none__">{t('common.none', 'None')}</SelectItem>
                {(subStatuses ?? []).filter(s => s.is_active).map((s) => (
                  <SelectItem key={s.id} value={s.id}>
                    <span className="flex items-center gap-1.5">
                      <span className="h-2 w-2 rounded-full" style={{ backgroundColor: s.color }} />
                      {s.name}
                    </span>
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          );
        })()}
        <div className="ml-auto flex items-center gap-2">
          <Button size="sm" variant="outline" onClick={() => setEditOpen(true)}>
            <Pencil className="mr-1 h-4 w-4" /> {t('projects.edit_project', 'Edit Project')}
          </Button>
          <Link to={`/invoices/new?contact_id=${summary.contact_id ?? ''}&project_id=${id}`}>
            <Button size="sm" variant="outline">
              <ReceiptText className="mr-1 h-4 w-4" /> {t('projects.create_invoice', 'Create Invoice')}
            </Button>
          </Link>
        </div>
      </div>

      {summary.description && (
        <MarkdownPreview content={summary.description} className="text-sm text-muted-foreground" />
      )}

      <ProjectEditDialog open={editOpen} onOpenChange={setEditOpen} summary={summary} />

      <Tabs value={activeTab} onValueChange={handleTabChange}>
        <TabsList variant="line">
          <TabsTrigger value="overview">{t('projects.tab_overview', 'Overview')}</TabsTrigger>
          <TabsTrigger value="wbs">{t('projects.tab_wbs', 'WBS')}</TabsTrigger>
          <TabsTrigger value="members">{t('projects.tab_members', 'Members')}</TabsTrigger>
          <TabsTrigger value="milestones">{t('projects.tab_milestones', 'Milestones')}</TabsTrigger>
          <TabsTrigger value="documents">{t('projects.tab_documents', 'Documents')}</TabsTrigger>
          <TabsTrigger value="time-tracking">{t('projects.tab_time_tracking', 'Time Tracking')}</TabsTrigger>
          <TabsTrigger value="billing">{t('projects.tab_billing', 'Billing')}</TabsTrigger>
          <TabsTrigger value="settings">{t('projects.tab_settings', 'Settings')}</TabsTrigger>
        </TabsList>

        <TabsContent value="overview">
          <ProjectOverviewTab
            summary={summary}
            totalHours={totalHours}
            billableHours={billableHours}
            budgetHours={budgetHours}
            totalInvoiced={totalInvoiced}
            nonBillableHours={budgetAnalytics?.non_billable_hours != null ? Number(budgetAnalytics.non_billable_hours) : (totalHours - billableHours)}
            unbilledAmount={budgetAnalytics ? (Number(budgetAnalytics.actual_amount ?? 0) - Number(budgetAnalytics.invoiced_amount ?? 0)) : 0}
            actualAmount={budgetAnalytics?.actual_amount != null ? Number(budgetAnalytics.actual_amount) : 0}
            onLanguageChange={handleLanguageChange}
          />
          <div className="mt-4 space-y-4">
            <BudgetAnalyticsChart
              timeline={budgetAnalytics?.timeline}
              isLoading={budgetLoading}
            />
            <BudgetBreakdownTable
              memberBreakdown={(budgetAnalytics?.per_member ?? []).map((m: Record<string, unknown>) => ({
                name: m.user_name as string,
                budget_hours: m.budget_hours != null ? Number(m.budget_hours) : null,
                actual_hours: Number(m.actual_hours) || 0,
                rate: m.rate != null ? Number(m.rate) : null,
                amount: Number(m.actual_amount) || 0,
              }))}
              activityBreakdown={(budgetAnalytics?.per_activity ?? []).map((a: Record<string, unknown>) => ({
                name: a.activity_name as string,
                budget_hours: a.budget_hours != null ? Number(a.budget_hours) : null,
                actual_hours: Number(a.actual_hours) || 0,
                rate: a.rate != null ? Number(a.rate) : null,
                amount: Number(a.actual_amount) || 0,
              }))}
            />
          </div>
        </TabsContent>

        <TabsContent value="wbs">
          <ProjectItemsTree projectId={id!} />
        </TabsContent>

        <TabsContent value="members">
          <ProjectMembersTab projectId={id!} />
        </TabsContent>

        <TabsContent value="milestones">
          <ProjectMilestonesTab projectId={id!} />
        </TabsContent>

        <TabsContent value="documents">
          <ProjectDocumentsTab projectId={id!} />
        </TabsContent>

        <TabsContent value="time-tracking">
          <ProjectTimeTrackingTab projectId={id!} summary={summary} />
        </TabsContent>

        <TabsContent value="billing">
          <ProjectBillingTab projectId={id!} />
        </TabsContent>

        <TabsContent value="settings">
          <ProjectSettingsTab projectId={id!} summary={summary} />
        </TabsContent>
      </Tabs>
    </div>
  );
}
