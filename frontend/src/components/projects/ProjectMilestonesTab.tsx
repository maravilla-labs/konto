import { useState } from 'react';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import {
  Table, TableBody, TableCell, TableHead, TableHeader, TableRow,
} from '@/components/ui/table';
import { useProjectMilestones, useReachProjectMilestone, useDeleteProjectMilestone } from '@/hooks/useApi';
import { MilestoneDialog } from './MilestoneDialog';
import { Plus, Pencil, Trash2, CheckCircle2 } from 'lucide-react';
import { toast } from 'sonner';
import { useI18n } from '@/i18n';
import type { ProjectMilestone } from '@/types/project-milestone';

interface ProjectMilestonesTabProps {
  projectId: string;
}

const statusVariant: Record<string, 'default' | 'secondary' | 'destructive' | 'outline'> = {
  pending: 'outline',
  reached: 'default',
  overdue: 'destructive',
};

export function ProjectMilestonesTab({ projectId }: ProjectMilestonesTabProps) {
  const { t } = useI18n();
  const { data: milestones, isLoading } = useProjectMilestones(projectId);
  const reachMilestone = useReachProjectMilestone();
  const deleteMilestone = useDeleteProjectMilestone();

  const [dialogOpen, setDialogOpen] = useState(false);
  const [editMilestone, setEditMilestone] = useState<ProjectMilestone | null>(null);

  const list = milestones ?? [];

  function handleCreate() {
    setEditMilestone(null);
    setDialogOpen(true);
  }

  function handleEdit(m: ProjectMilestone) {
    setEditMilestone(m);
    setDialogOpen(true);
  }

  function handleReach(milestoneId: string) {
    reachMilestone.mutate(milestoneId, {
      onSuccess: () => toast.success(t('projects.milestone_reached', 'Milestone marked as reached')),
      onError: () => toast.error(t('projects.milestone_reach_failed', 'Failed to mark milestone')),
    });
  }

  function handleDelete(milestoneId: string) {
    if (!confirm(t('projects.confirm_delete_milestone', 'Delete this milestone?'))) return;
    deleteMilestone.mutate(milestoneId, {
      onSuccess: () => toast.success(t('projects.milestone_deleted', 'Milestone deleted')),
      onError: () => toast.error(t('projects.milestone_delete_failed', 'Failed to delete milestone')),
    });
  }

  if (isLoading) return <p className="text-sm text-muted-foreground py-4">{t('common.loading', 'Loading...')}</p>;

  return (
    <div className="space-y-4">
      <div className="flex justify-end">
        <Button size="sm" onClick={handleCreate}>
          <Plus className="mr-1 h-4 w-4" /> {t('projects.add_milestone', 'Add Milestone')}
        </Button>
      </div>

      <Card>
        <CardContent className="p-0">
          {list.length > 0 ? (
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>{t('projects.name', 'Name')}</TableHead>
                  <TableHead>{t('projects.target_date', 'Target Date')}</TableHead>
                  <TableHead>{t('common.status', 'Status')}</TableHead>
                  <TableHead className="hidden md:table-cell">{t('projects.reached_at', 'Reached')}</TableHead>
                  <TableHead className="w-32">{t('common.actions', 'Actions')}</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {list.map((m) => (
                  <TableRow key={m.id}>
                    <TableCell>
                      <div>
                        <span className="font-medium">{m.name}</span>
                        {m.description && (
                          <span className="block text-xs text-muted-foreground truncate max-w-xs">
                            {m.description}
                          </span>
                        )}
                      </div>
                    </TableCell>
                    <TableCell className="font-mono">{m.target_date}</TableCell>
                    <TableCell>
                      <Badge variant={statusVariant[m.status] ?? 'outline'}>
                        {m.status}
                      </Badge>
                    </TableCell>
                    <TableCell className="hidden md:table-cell font-mono">
                      {m.reached_at ?? '—'}
                    </TableCell>
                    <TableCell>
                      <div className="flex gap-1">
                        {m.status !== 'reached' && (
                          <Button variant="ghost" size="icon" className="h-8 w-8" onClick={() => handleReach(m.id)} title={t('projects.mark_reached', 'Mark as Reached')}>
                            <CheckCircle2 className="h-3.5 w-3.5 text-green-600" />
                          </Button>
                        )}
                        <Button variant="ghost" size="icon" className="h-8 w-8" onClick={() => handleEdit(m)} title={t('common.edit', 'Edit')}>
                          <Pencil className="h-3.5 w-3.5" />
                        </Button>
                        <Button variant="ghost" size="icon" className="h-8 w-8" onClick={() => handleDelete(m.id)} title={t('common.delete', 'Delete')}>
                          <Trash2 className="h-3.5 w-3.5" />
                        </Button>
                      </div>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          ) : (
            <p className="py-6 text-center text-sm text-muted-foreground">
              {t('projects.no_milestones', 'No milestones. Add your first milestone.')}
            </p>
          )}
        </CardContent>
      </Card>

      <MilestoneDialog
        projectId={projectId}
        milestone={editMilestone}
        open={dialogOpen}
        onOpenChange={setDialogOpen}
      />
    </div>
  );
}
