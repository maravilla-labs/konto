import { useState, useEffect } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import {
  Select, SelectContent, SelectItem, SelectTrigger, SelectValue,
} from '@/components/ui/select';
import { useUpdateProject, useProjectSubStatuses, useContacts } from '@/hooks/useApi';
import { useI18n } from '@/i18n';
import { toast } from 'sonner';
import type { ProjectSummary } from '@/types/projects';

interface ProjectSettingsTabProps {
  projectId: string;
  summary: ProjectSummary;
}

export function ProjectSettingsTab({ projectId, summary }: ProjectSettingsTabProps) {
  const { t } = useI18n();
  const updateProject = useUpdateProject();
  const { data: subStatuses } = useProjectSubStatuses();
  const { data: contactsData } = useContacts({ per_page: 200 });
  const contacts = contactsData?.data ?? [];

  const [form, setForm] = useState({
    contact_id: '',
    invoicing_method: 'hourly',
    currency: 'CHF',
    rounding_method: '',
    rounding_factor_minutes: '',
    flat_rate_total: '',
    sub_status_id: '',
    hourly_rate: '',
    soft_budget_hours: '',
    hard_budget_hours: '',
    soft_budget_amount: '',
    hard_budget_amount: '',
  });

  useEffect(() => {
    setForm({
      contact_id: summary.contact_id ?? '',
      invoicing_method: summary.invoicing_method ?? 'hourly',
      currency: summary.currency ?? 'CHF',
      rounding_method: summary.rounding_method ?? '',
      rounding_factor_minutes: summary.rounding_factor_minutes?.toString() ?? '',
      flat_rate_total: summary.flat_rate_total?.toString() ?? '',
      sub_status_id: summary.sub_status_id ?? '',
      hourly_rate: summary.hourly_rate ?? '',
      soft_budget_hours: summary.soft_budget_hours?.toString() ?? '',
      hard_budget_hours: summary.hard_budget_hours?.toString() ?? '',
      soft_budget_amount: summary.soft_budget_amount?.toString() ?? '',
      hard_budget_amount: summary.hard_budget_amount?.toString() ?? '',
    });
  }, [summary]);

  function handleSave() {
    updateProject.mutate(
      {
        id: projectId,
        data: {
          contact_id: form.contact_id || undefined,
          invoicing_method: form.invoicing_method || undefined,
          currency: form.currency || undefined,
          rounding_method: form.rounding_method || null,
          rounding_factor_minutes: form.rounding_factor_minutes ? Number(form.rounding_factor_minutes) : null,
          flat_rate_total: form.flat_rate_total ? Number(form.flat_rate_total) : null,
          sub_status_id: form.sub_status_id || undefined,
          hourly_rate: form.hourly_rate || undefined,
          soft_budget_hours: form.soft_budget_hours ? Number(form.soft_budget_hours) : undefined,
          hard_budget_hours: form.hard_budget_hours ? Number(form.hard_budget_hours) : undefined,
          soft_budget_amount: form.soft_budget_amount ? Number(form.soft_budget_amount) : undefined,
          hard_budget_amount: form.hard_budget_amount ? Number(form.hard_budget_amount) : undefined,
        },
      },
      {
        onSuccess: () => toast.success(t('projects.project_updated', 'Project updated')),
        onError: () => toast.error(t('projects.project_update_failed', 'Failed to update project')),
      },
    );
  }

  return (
    <div className="space-y-4 pt-2">
      <Card>
        <CardHeader>
          <CardTitle className="text-base">{t('project_conditions.client', 'Client')}</CardTitle>
        </CardHeader>
        <CardContent>
          <div>
            <Label>{t('projects.contact_field', 'Contact / Company')}</Label>
            <Select
              value={form.contact_id || '__none__'}
              onValueChange={(v) => setForm({ ...form, contact_id: v === '__none__' ? '' : v })}
            >
              <SelectTrigger><SelectValue placeholder={t('common.none', 'None')} /></SelectTrigger>
              <SelectContent>
                <SelectItem value="__none__">{t('common.none', 'None')}</SelectItem>
                {contacts.map((c) => (
                  <SelectItem key={c.id} value={c.id}>{c.name1}{c.name2 ? ` — ${c.name2}` : ''}</SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">{t('project_conditions.title', 'Conditions')}</CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="grid grid-cols-2 gap-4">
            <div>
              <Label>{t('project_conditions.invoicing_method', 'Invoicing Method')}</Label>
              <Select value={form.invoicing_method} onValueChange={(v) => setForm({ ...form, invoicing_method: v })}>
                <SelectTrigger><SelectValue /></SelectTrigger>
                <SelectContent>
                  <SelectItem value="hourly">{t('project_conditions.invoicing_method.hourly', 'Hourly')}</SelectItem>
                  <SelectItem value="fixed_price">{t('project_conditions.invoicing_method.fixed_price', 'Fixed Price')}</SelectItem>
                  <SelectItem value="flat_rate">{t('project_conditions.invoicing_method.flat_rate', 'Flat Rate')}</SelectItem>
                  <SelectItem value="non_billable">{t('project_conditions.invoicing_method.non_billable', 'Non-Billable')}</SelectItem>
                </SelectContent>
              </Select>
            </div>
            <div>
              <Label>{t('project_conditions.currency', 'Currency')}</Label>
              <Input value={form.currency} onChange={(e) => setForm({ ...form, currency: e.target.value })} placeholder="CHF" />
            </div>
          </div>
          <div className="grid grid-cols-2 gap-4">
            <div>
              <Label>{t('projects.hourly_rate_field', 'Hourly Rate (CHF)')}</Label>
              <Input type="number" step="0.01" value={form.hourly_rate} onChange={(e) => setForm({ ...form, hourly_rate: e.target.value })} />
            </div>
            {form.invoicing_method === 'flat_rate' && (
              <div>
                <Label>{t('project_conditions.flat_rate_total', 'Flat Rate Total')}</Label>
                <Input type="number" step="0.01" value={form.flat_rate_total} onChange={(e) => setForm({ ...form, flat_rate_total: e.target.value })} />
              </div>
            )}
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">{t('project_conditions.rounding', 'Time Rounding')}</CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="grid grid-cols-2 gap-4">
            <div>
              <Label>{t('project_conditions.rounding_method', 'Rounding Method')}</Label>
              <Select value={form.rounding_method || '__none__'} onValueChange={(v) => setForm({ ...form, rounding_method: v === '__none__' ? '' : v })}>
                <SelectTrigger><SelectValue /></SelectTrigger>
                <SelectContent>
                  <SelectItem value="__none__">{t('project_conditions.rounding_method.none', 'No rounding')}</SelectItem>
                  <SelectItem value="up">{t('project_conditions.rounding_method.up', 'Round up')}</SelectItem>
                  <SelectItem value="down">{t('project_conditions.rounding_method.down', 'Round down')}</SelectItem>
                  <SelectItem value="nearest">{t('project_conditions.rounding_method.nearest', 'Round nearest')}</SelectItem>
                </SelectContent>
              </Select>
            </div>
            <div>
              <Label>{t('project_conditions.rounding_factor', 'Rounding Factor')}</Label>
              <Select value={form.rounding_factor_minutes || '__none__'} onValueChange={(v) => setForm({ ...form, rounding_factor_minutes: v === '__none__' ? '' : v })}>
                <SelectTrigger><SelectValue /></SelectTrigger>
                <SelectContent>
                  <SelectItem value="__none__">{t('common.none', 'None')}</SelectItem>
                  <SelectItem value="5">5 min</SelectItem>
                  <SelectItem value="10">10 min</SelectItem>
                  <SelectItem value="15">15 min</SelectItem>
                  <SelectItem value="30">30 min</SelectItem>
                  <SelectItem value="60">60 min</SelectItem>
                </SelectContent>
              </Select>
            </div>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">{t('projects.budget', 'Budget')}</CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="grid grid-cols-2 gap-4">
            <div>
              <Label>{t('projects.soft_budget_hours_field', 'Soft Budget Hours')}</Label>
              <Input type="number" step="0.5" value={form.soft_budget_hours} onChange={(e) => setForm({ ...form, soft_budget_hours: e.target.value })} />
            </div>
            <div>
              <Label>{t('projects.hard_budget_hours_field', 'Hard Budget Hours')}</Label>
              <Input type="number" step="0.5" value={form.hard_budget_hours} onChange={(e) => setForm({ ...form, hard_budget_hours: e.target.value })} />
            </div>
          </div>
          <div className="grid grid-cols-2 gap-4">
            <div>
              <Label>{t('projects.soft_budget_amount_field', 'Soft Budget (CHF)')}</Label>
              <Input type="number" step="0.01" value={form.soft_budget_amount} onChange={(e) => setForm({ ...form, soft_budget_amount: e.target.value })} />
            </div>
            <div>
              <Label>{t('projects.hard_budget_amount_field', 'Hard Budget (CHF)')}</Label>
              <Input type="number" step="0.01" value={form.hard_budget_amount} onChange={(e) => setForm({ ...form, hard_budget_amount: e.target.value })} />
            </div>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">{t('project_conditions.sub_status', 'Sub-Status')}</CardTitle>
        </CardHeader>
        <CardContent>
          <Select value={form.sub_status_id || '__none__'} onValueChange={(v) => setForm({ ...form, sub_status_id: v === '__none__' ? '' : v })}>
            <SelectTrigger><SelectValue /></SelectTrigger>
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
        </CardContent>
      </Card>

      <div className="flex justify-end">
        <Button onClick={handleSave} disabled={updateProject.isPending}>
          {updateProject.isPending ? t('common.saving', 'Saving...') : t('common.save_changes', 'Save Changes')}
        </Button>
      </div>
    </div>
  );
}
