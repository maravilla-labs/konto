import { useState } from 'react';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Switch } from '@/components/ui/switch';
import { Skeleton } from '@/components/ui/skeleton';
import {
  Dialog, DialogContent, DialogHeader, DialogTitle, DialogTrigger,
} from '@/components/ui/dialog';
import {
  AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent,
  AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger,
} from '@/components/ui/alert-dialog';
import {
  Table, TableBody, TableCell, TableHead, TableHeader, TableRow,
} from '@/components/ui/table';
import {
  useProjectSubStatuses, useCreateProjectSubStatus, useUpdateProjectSubStatus, useDeleteProjectSubStatus,
} from '@/hooks/useApi';
import { useI18n } from '@/i18n';
import { toast } from 'sonner';
import { Plus, Pencil, Trash2 } from 'lucide-react';
import type { ProjectSubStatus } from '@/types/project-sub-status';

export function ProjectSubStatusesPage() {
  const { t } = useI18n();
  const { data, isLoading } = useProjectSubStatuses();
  const createSubStatus = useCreateProjectSubStatus();
  const updateSubStatus = useUpdateProjectSubStatus();
  const deleteSubStatus = useDeleteProjectSubStatus();

  const [createOpen, setCreateOpen] = useState(false);
  const [editItem, setEditItem] = useState<ProjectSubStatus | null>(null);
  const [form, setForm] = useState({ name: '', sort_order: '0', color: '#3b82f6' });

  const items = data ?? [];

  function handleCreate() {
    createSubStatus.mutate(
      { name: form.name, sort_order: parseInt(form.sort_order) || 0, color: form.color },
      {
        onSuccess: () => {
          toast.success(t('project_sub_statuses.created', 'Sub-status created'));
          setCreateOpen(false);
          setForm({ name: '', sort_order: '0', color: '#3b82f6' });
        },
        onError: () => toast.error(t('project_sub_statuses.create_failed', 'Failed to create sub-status')),
      },
    );
  }

  function openEdit(item: ProjectSubStatus) {
    setEditItem(item);
    setForm({ name: item.name, sort_order: String(item.sort_order), color: item.color });
  }

  function handleUpdate() {
    if (!editItem) return;
    updateSubStatus.mutate(
      { id: editItem.id, data: { name: form.name, sort_order: parseInt(form.sort_order) || 0, color: form.color } },
      {
        onSuccess: () => {
          toast.success(t('project_sub_statuses.updated', 'Sub-status updated'));
          setEditItem(null);
        },
        onError: () => toast.error(t('project_sub_statuses.update_failed', 'Failed to update sub-status')),
      },
    );
  }

  function handleToggleActive(item: ProjectSubStatus) {
    updateSubStatus.mutate(
      { id: item.id, data: { is_active: !item.is_active } },
      {
        onSuccess: () => toast.success(t('project_sub_statuses.updated', 'Sub-status updated')),
        onError: () => toast.error(t('project_sub_statuses.update_failed', 'Failed to update sub-status')),
      },
    );
  }

  function handleDelete(id: string) {
    deleteSubStatus.mutate(id, {
      onSuccess: () => toast.success(t('project_sub_statuses.deleted', 'Sub-status deleted')),
      onError: () => toast.error(t('project_sub_statuses.delete_failed', 'Failed to delete sub-status')),
    });
  }

  return (
    <div className="space-y-4">
      <div className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h2 className="text-lg font-semibold">{t('project_sub_statuses.title', 'Project Sub-Statuses')}</h2>
          <p className="text-sm text-muted-foreground">{t('project_sub_statuses.subtitle', 'Configure project sub-status categories')}</p>
        </div>
        <Dialog open={createOpen} onOpenChange={setCreateOpen}>
          <DialogTrigger asChild>
            <Button size="sm">
              <Plus className="mr-1 h-4 w-4" /> {t('project_sub_statuses.add', 'Add Sub-Status')}
            </Button>
          </DialogTrigger>
          <DialogContent className="max-w-sm">
            <DialogHeader>
              <DialogTitle>{t('project_sub_statuses.add', 'Add Sub-Status')}</DialogTitle>
            </DialogHeader>
            <div className="space-y-4">
              <div>
                <Label>{t('project_sub_statuses.name', 'Name')}</Label>
                <Input value={form.name} onChange={(e) => setForm({ ...form, name: e.target.value })} />
              </div>
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <Label>{t('project_sub_statuses.sort_order', 'Sort Order')}</Label>
                  <Input type="number" value={form.sort_order} onChange={(e) => setForm({ ...form, sort_order: e.target.value })} />
                </div>
                <div>
                  <Label>{t('project_sub_statuses.color', 'Color')}</Label>
                  <div className="flex gap-2 items-center">
                    <input type="color" value={form.color} onChange={(e) => setForm({ ...form, color: e.target.value })} className="h-9 w-12 rounded border cursor-pointer" />
                    <Input value={form.color} onChange={(e) => setForm({ ...form, color: e.target.value })} className="flex-1" />
                  </div>
                </div>
              </div>
              <Button onClick={handleCreate} className="w-full" disabled={!form.name || createSubStatus.isPending}>
                {t('common.create', 'Create')}
              </Button>
            </div>
          </DialogContent>
        </Dialog>
      </div>

      <Card>
        <CardContent className="p-0">
          {isLoading ? (
            <div className="space-y-2 p-4">
              {Array.from({ length: 5 }).map((_, i) => <Skeleton key={i} className="h-10 w-full" />)}
            </div>
          ) : items.length > 0 ? (
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead className="w-12">{t('project_sub_statuses.color', 'Color')}</TableHead>
                  <TableHead>{t('project_sub_statuses.name', 'Name')}</TableHead>
                  <TableHead>{t('project_sub_statuses.sort_order', 'Sort Order')}</TableHead>
                  <TableHead>{t('project_sub_statuses.is_active', 'Active')}</TableHead>
                  <TableHead className="w-24">{t('common.actions', 'Actions')}</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {items.map((item) => (
                  <TableRow key={item.id} className={!item.is_active ? 'opacity-50' : ''}>
                    <TableCell>
                      <span className="inline-block h-4 w-4 rounded-full border" style={{ backgroundColor: item.color }} />
                    </TableCell>
                    <TableCell className="font-medium">{item.name}</TableCell>
                    <TableCell className="font-mono">{item.sort_order}</TableCell>
                    <TableCell>
                      <Switch checked={item.is_active} onCheckedChange={() => handleToggleActive(item)} />
                    </TableCell>
                    <TableCell>
                      <div className="flex gap-1">
                        <Button variant="ghost" size="icon" className="h-8 w-8" onClick={() => openEdit(item)}>
                          <Pencil className="h-3.5 w-3.5" />
                        </Button>
                        <AlertDialog>
                          <AlertDialogTrigger asChild>
                            <Button variant="ghost" size="icon" className="h-8 w-8">
                              <Trash2 className="h-3.5 w-3.5" />
                            </Button>
                          </AlertDialogTrigger>
                          <AlertDialogContent>
                            <AlertDialogHeader>
                              <AlertDialogTitle>{t('common.delete', 'Delete')}?</AlertDialogTitle>
                              <AlertDialogDescription>{t('project_sub_statuses.confirm_delete', 'Delete this sub-status?')}</AlertDialogDescription>
                            </AlertDialogHeader>
                            <AlertDialogFooter>
                              <AlertDialogCancel>{t('common.cancel', 'Cancel')}</AlertDialogCancel>
                              <AlertDialogAction onClick={() => handleDelete(item.id)}>{t('common.delete', 'Delete')}</AlertDialogAction>
                            </AlertDialogFooter>
                          </AlertDialogContent>
                        </AlertDialog>
                      </div>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          ) : (
            <p className="py-8 text-center text-sm text-muted-foreground">
              {t('project_sub_statuses.no_results', 'No sub-statuses configured.')}
            </p>
          )}
        </CardContent>
      </Card>

      {/* Edit Dialog */}
      <Dialog open={!!editItem} onOpenChange={(open) => !open && setEditItem(null)}>
        <DialogContent className="max-w-sm">
          <DialogHeader>
            <DialogTitle>{t('project_sub_statuses.edit', 'Edit Sub-Status')}</DialogTitle>
          </DialogHeader>
          <div className="space-y-4">
            <div>
              <Label>{t('project_sub_statuses.name', 'Name')}</Label>
              <Input value={form.name} onChange={(e) => setForm({ ...form, name: e.target.value })} />
            </div>
            <div className="grid grid-cols-2 gap-4">
              <div>
                <Label>{t('project_sub_statuses.sort_order', 'Sort Order')}</Label>
                <Input type="number" value={form.sort_order} onChange={(e) => setForm({ ...form, sort_order: e.target.value })} />
              </div>
              <div>
                <Label>{t('project_sub_statuses.color', 'Color')}</Label>
                <div className="flex gap-2 items-center">
                  <input type="color" value={form.color} onChange={(e) => setForm({ ...form, color: e.target.value })} className="h-9 w-12 rounded border cursor-pointer" />
                  <Input value={form.color} onChange={(e) => setForm({ ...form, color: e.target.value })} className="flex-1" />
                </div>
              </div>
            </div>
            <Button onClick={handleUpdate} className="w-full" disabled={!form.name || updateSubStatus.isPending}>
              {t('common.save_changes', 'Save Changes')}
            </Button>
          </div>
        </DialogContent>
      </Dialog>
    </div>
  );
}
