import { useState, useMemo } from 'react';
import { Link } from 'react-router-dom';
import { Card, CardContent } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import {
  Dialog, DialogContent, DialogHeader, DialogTitle,
} from '@/components/ui/dialog';
import {
  Select, SelectContent, SelectItem, SelectTrigger, SelectValue,
} from '@/components/ui/select';
import {
  Table, TableBody, TableCell, TableHead, TableHeader, TableRow,
} from '@/components/ui/table';
import {
  useInvoices, useActivityTypes, useProjectActivityTypes,
  useAddProjectActivityType, useRemoveProjectActivityType,
} from '@/hooks/useApi';
import { useI18n } from '@/i18n';
import { toast } from 'sonner';
import { Plus, Trash2 } from 'lucide-react';
import type { ActivityType } from '@/types/activity-type';

interface ProjectBillingTabProps {
  projectId: string;
}

export function ProjectBillingTab({ projectId }: ProjectBillingTabProps) {
  const { t } = useI18n();
  const { data: invoiceData } = useInvoices({ project_id: projectId });
  const { data: allActivityTypes } = useActivityTypes();
  const { data: projectActivityTypes } = useProjectActivityTypes(projectId);
  const addProjectAT = useAddProjectActivityType();
  const removeProjectAT = useRemoveProjectActivityType();

  const [addATOpen, setAddATOpen] = useState(false);
  const [addATId, setAddATId] = useState('');
  const [addATRate, setAddATRate] = useState('');

  const activeTypes = useMemo(() => (allActivityTypes ?? []).filter((at: ActivityType) => at.is_active), [allActivityTypes]);
  const patIds = useMemo(() => new Set((projectActivityTypes ?? []).map((p) => p.activity_type_id)), [projectActivityTypes]);
  const invoices = invoiceData?.data ?? [];

  return (
    <div className="space-y-6 pt-2">
      {/* Project Activity Types */}
      <div>
        <div className="flex items-center justify-between mb-2">
          <h3 className="text-sm font-semibold">{t('projects.activity_types', 'Activity Types')}</h3>
          <Button size="sm" variant="outline" onClick={() => { setAddATOpen(true); setAddATId(''); setAddATRate(''); }}>
            <Plus className="mr-1 h-4 w-4" /> {t('projects.add_activity_type', 'Add')}
          </Button>
        </div>
        {(projectActivityTypes ?? []).length > 0 ? (
          <Card>
            <CardContent className="p-0">
              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead>{t('common.name', 'Name')}</TableHead>
                    <TableHead>{t('activity_types.unit_type', 'Unit')}</TableHead>
                    <TableHead>{t('activity_types.default_rate', 'Default Rate')}</TableHead>
                    <TableHead>{t('projects.project_rate', 'Project Rate')}</TableHead>
                    <TableHead>{t('projects.effective_rate', 'Effective')}</TableHead>
                    <TableHead className="text-right">{t('project_budget.budget_hours', 'Budget Hours')}</TableHead>
                    <TableHead>{t('project_budget.chargeable', 'Chargeable')}</TableHead>
                    <TableHead className="w-16">{t('common.actions', 'Actions')}</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {(projectActivityTypes ?? []).map((pat) => (
                    <TableRow key={pat.id}>
                      <TableCell className="font-medium">{pat.activity_type_name ?? '—'}</TableCell>
                      <TableCell>{pat.unit_type ?? '—'}</TableCell>
                      <TableCell className="font-mono">{pat.default_rate != null ? Number(pat.default_rate).toFixed(2) : '—'}</TableCell>
                      <TableCell className="font-mono">{pat.rate != null ? Number(pat.rate).toFixed(2) : '—'}</TableCell>
                      <TableCell className="font-mono font-semibold">{pat.effective_rate != null ? Number(pat.effective_rate).toFixed(2) : '—'}</TableCell>
                      <TableCell className="text-right font-mono">{pat.budget_hours != null ? `${pat.budget_hours}h` : '—'}</TableCell>
                      <TableCell>
                        <Badge variant={pat.chargeable ? 'default' : 'outline'}>
                          {pat.chargeable ? t('time_entry_status.billable', 'Billable') : t('time_entry_status.non_billable', 'Non-Billable')}
                        </Badge>
                      </TableCell>
                      <TableCell>
                        <Button variant="ghost" size="icon" className="h-8 w-8" onClick={() => {
                          removeProjectAT.mutate({ projectId, patId: pat.id }, {
                            onSuccess: () => toast.success(t('common.deleted', 'Removed')),
                          });
                        }}>
                          <Trash2 className="h-3.5 w-3.5" />
                        </Button>
                      </TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </CardContent>
          </Card>
        ) : (
          <p className="text-xs text-muted-foreground">{t('projects.all_types_available', 'All activity types available (none restricted)')}</p>
        )}

        <Dialog open={addATOpen} onOpenChange={setAddATOpen}>
          <DialogContent className="max-w-sm">
            <DialogHeader><DialogTitle>{t('projects.add_activity_type', 'Add Activity Type')}</DialogTitle></DialogHeader>
            <div className="space-y-4">
              <div>
                <Label>{t('time_entries.activity_type', 'Activity Type')}</Label>
                <Select value={addATId} onValueChange={setAddATId}>
                  <SelectTrigger><SelectValue placeholder={t('common.select', 'Select...')} /></SelectTrigger>
                  <SelectContent>
                    {activeTypes.filter((at: ActivityType) => !patIds.has(at.id)).map((at: ActivityType) => (
                      <SelectItem key={at.id} value={at.id}>{at.name} ({at.unit_type})</SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
              <div>
                <Label>{t('projects.project_rate', 'Project Rate Override')} ({t('common.optional', 'optional')})</Label>
                <Input type="number" step="0.01" value={addATRate} onChange={(e) => setAddATRate(e.target.value)} placeholder="0.00" />
              </div>
              <Button className="w-full" disabled={!addATId || addProjectAT.isPending} onClick={() => {
                addProjectAT.mutate(
                  { projectId, data: { activity_type_id: addATId, rate: addATRate ? parseFloat(addATRate) : undefined } },
                  { onSuccess: () => { toast.success(t('common.created', 'Added')); setAddATOpen(false); } },
                );
              }}>
                {t('common.create', 'Add')}
              </Button>
            </div>
          </DialogContent>
        </Dialog>
      </div>

      {/* Project Invoices */}
      <div>
        <h3 className="text-sm font-semibold mb-2">{t('projects.tab_invoices', 'Invoices')}</h3>
        <Card>
          <CardContent className="p-0">
            {invoices.length > 0 ? (
              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead>{t('common.number', 'Number')}</TableHead>
                    <TableHead>{t('common.status', 'Status')}</TableHead>
                    <TableHead>{t('common.date', 'Date')}</TableHead>
                    <TableHead className="text-right">{t('common.total', 'Total')}</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {invoices.map((inv) => (
                    <TableRow key={inv.id}>
                      <TableCell>
                        <Link to={`/invoices/${inv.id}`} className="text-primary hover:underline font-medium">
                          {inv.invoice_number ?? t('invoices.draft', 'Draft')}
                        </Link>
                      </TableCell>
                      <TableCell><Badge variant="secondary">{inv.status}</Badge></TableCell>
                      <TableCell className="font-mono">{inv.issue_date}</TableCell>
                      <TableCell className="text-right font-mono">{Number(inv.total).toFixed(2)}</TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            ) : (
              <p className="py-6 text-center text-sm text-muted-foreground">
                {t('projects.no_invoices', 'No invoices for this project.')}
              </p>
            )}
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
