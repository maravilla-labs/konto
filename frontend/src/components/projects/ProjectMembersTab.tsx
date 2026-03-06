import { useState } from 'react';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import {
  Table, TableBody, TableCell, TableHead, TableHeader, TableRow,
} from '@/components/ui/table';
import { useProjectMembers, useRemoveProjectMember, useUpdateProjectMember, useRateFunctions } from '@/hooks/useApi';
import { AddMemberDialog } from './AddMemberDialog';
import {
  Dialog, DialogContent, DialogHeader, DialogTitle,
} from '@/components/ui/dialog';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import {
  Select, SelectContent, SelectItem, SelectTrigger, SelectValue,
} from '@/components/ui/select';
import { Plus, Pencil, Trash2, DollarSign } from 'lucide-react';
import { toast } from 'sonner';
import { useI18n } from '@/i18n';
import type { ProjectMember } from '@/types/project-member';

interface ProjectMembersTabProps {
  projectId: string;
}

export function ProjectMembersTab({ projectId }: ProjectMembersTabProps) {
  const { t } = useI18n();
  const { data: members, isLoading } = useProjectMembers(projectId);
  const { data: rateFunctions } = useRateFunctions();
  const removeMember = useRemoveProjectMember();
  const updateMember = useUpdateProjectMember();

  const [addOpen, setAddOpen] = useState(false);
  const [editMember, setEditMember] = useState<ProjectMember | null>(null);
  const [editForm, setEditForm] = useState({
    rate_function_id: '',
    hourly_rate: '',
    role_label: '',
    budget_hours: '',
  });

  const functions = rateFunctions ?? [];
  const list = members ?? [];

  function openEdit(m: ProjectMember) {
    setEditMember(m);
    setEditForm({
      rate_function_id: m.rate_function_id ?? '',
      hourly_rate: m.hourly_rate != null ? String(m.hourly_rate) : '',
      role_label: m.role_label ?? '',
      budget_hours: m.budget_hours != null ? String(m.budget_hours) : '',
    });
  }

  function handleUpdate() {
    if (!editMember) return;
    updateMember.mutate(
      {
        projectId,
        memberId: editMember.id,
        data: {
          rate_function_id: editForm.rate_function_id || undefined,
          hourly_rate: editForm.hourly_rate ? Number(editForm.hourly_rate) : undefined,
          role_label: editForm.role_label || undefined,
        },
      },
      {
        onSuccess: () => { toast.success(t('projects.member_updated', 'Member updated')); setEditMember(null); },
        onError: () => toast.error(t('projects.member_update_failed', 'Failed to update member')),
      },
    );
  }

  function handleRemove(memberId: string) {
    if (!confirm(t('projects.confirm_remove_member', 'Remove this team member?'))) return;
    removeMember.mutate(
      { projectId, memberId },
      {
        onSuccess: () => toast.success(t('projects.member_removed', 'Member removed')),
        onError: () => toast.error(t('projects.member_remove_failed', 'Failed to remove member')),
      },
    );
  }

  function getRateIndicator(m: ProjectMember) {
    if (m.hourly_rate != null) return { label: t('projects.rate_override', 'Override'), variant: 'default' as const };
    if (m.rate_function_name) return { label: m.rate_function_name, variant: 'secondary' as const };
    return { label: t('projects.rate_project', 'Project'), variant: 'outline' as const };
  }

  if (isLoading) return <p className="text-sm text-muted-foreground py-4">{t('common.loading', 'Loading...')}</p>;

  return (
    <div className="space-y-4">
      <div className="flex justify-end">
        <Button size="sm" onClick={() => setAddOpen(true)}>
          <Plus className="mr-1 h-4 w-4" /> {t('projects.add_member', 'Add Team Member')}
        </Button>
      </div>

      <Card>
        <CardContent className="p-0">
          {list.length > 0 ? (
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>{t('projects.member_name', 'Name')}</TableHead>
                  <TableHead>{t('projects.role_label', 'Role')}</TableHead>
                  <TableHead>{t('projects.rate_function', 'Function')}</TableHead>
                  <TableHead className="text-right">{t('projects.resolved_rate', 'Rate (CHF/h)')}</TableHead>
                  <TableHead className="text-right">{t('project_budget.budget_hours', 'Budget Hours')}</TableHead>
                  <TableHead className="w-24">{t('common.actions', 'Actions')}</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {list.map((m) => {
                  const rate = getRateIndicator(m);
                  return (
                    <TableRow key={m.id}>
                      <TableCell className="font-medium">{m.user_name ?? m.user_id}</TableCell>
                      <TableCell>{m.role_label ?? '—'}</TableCell>
                      <TableCell>
                        <Badge variant={rate.variant}>{rate.label}</Badge>
                      </TableCell>
                      <TableCell className="text-right font-mono">
                        {m.resolved_rate != null ? (
                          <span className="flex items-center justify-end gap-1">
                            <DollarSign className="h-3 w-3 text-muted-foreground" />
                            {Number(m.resolved_rate).toFixed(2)}
                          </span>
                        ) : '—'}
                      </TableCell>
                      <TableCell className="text-right font-mono">
                        {m.budget_hours != null ? `${m.budget_hours}h` : '—'}
                      </TableCell>
                      <TableCell>
                        <div className="flex gap-1">
                          <Button variant="ghost" size="icon" className="h-8 w-8" onClick={() => openEdit(m)} title={t('common.edit', 'Edit')}>
                            <Pencil className="h-3.5 w-3.5" />
                          </Button>
                          <Button variant="ghost" size="icon" className="h-8 w-8" onClick={() => handleRemove(m.id)} title={t('common.delete', 'Remove')}>
                            <Trash2 className="h-3.5 w-3.5" />
                          </Button>
                        </div>
                      </TableCell>
                    </TableRow>
                  );
                })}
              </TableBody>
            </Table>
          ) : (
            <p className="py-6 text-center text-sm text-muted-foreground">
              {t('projects.no_members', 'No team members assigned. Add your first member.')}
            </p>
          )}
        </CardContent>
      </Card>

      <AddMemberDialog projectId={projectId} open={addOpen} onOpenChange={setAddOpen} />

      {/* Edit Member Dialog */}
      <Dialog open={!!editMember} onOpenChange={(open) => !open && setEditMember(null)}>
        <DialogContent className="max-w-md">
          <DialogHeader>
            <DialogTitle>{t('projects.edit_member', 'Edit Member')}</DialogTitle>
          </DialogHeader>
          <div className="space-y-4">
            <div>
              <Label>{t('projects.rate_function', 'Rate Function')}</Label>
              <Select value={editForm.rate_function_id || '__none__'} onValueChange={(v) => setEditForm({ ...editForm, rate_function_id: v === '__none__' ? '' : v })}>
                <SelectTrigger><SelectValue /></SelectTrigger>
                <SelectContent>
                  <SelectItem value="__none__">{t('common.none', 'None')}</SelectItem>
                  {functions.map((rf) => (
                    <SelectItem key={rf.id} value={rf.id}>{rf.name} (CHF {rf.hourly_rate}/h)</SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
            <div>
              <Label>{t('projects.hourly_rate_override', 'Hourly Rate Override')}</Label>
              <Input type="number" step="0.01" value={editForm.hourly_rate} onChange={(e) => setEditForm({ ...editForm, hourly_rate: e.target.value })} />
            </div>
            <div>
              <Label>{t('projects.role_label', 'Role Label')}</Label>
              <Input value={editForm.role_label} onChange={(e) => setEditForm({ ...editForm, role_label: e.target.value })} />
            </div>
            <div>
              <Label>{t('project_budget.budget_hours', 'Budget Hours')}</Label>
              <Input type="number" step="0.5" value={editForm.budget_hours} onChange={(e) => setEditForm({ ...editForm, budget_hours: e.target.value })} placeholder={t('common.optional', 'optional')} />
            </div>
            <Button onClick={handleUpdate} className="w-full" disabled={updateMember.isPending}>
              {t('common.update', 'Update')}
            </Button>
          </div>
        </DialogContent>
      </Dialog>
    </div>
  );
}
