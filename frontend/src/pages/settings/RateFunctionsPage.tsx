import { useState } from 'react';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Switch } from '@/components/ui/switch';
import { Badge } from '@/components/ui/badge';
import { Skeleton } from '@/components/ui/skeleton';
import { RichTextEditor } from '@/components/ui/rich-text-editor';
import {
  Dialog, DialogContent, DialogHeader, DialogTitle,
} from '@/components/ui/dialog';
import {
  Table, TableBody, TableCell, TableHead, TableHeader, TableRow,
} from '@/components/ui/table';
import { useRateFunctions, useCreateRateFunction, useUpdateRateFunction, useDeleteRateFunction } from '@/hooks/useApi';
import { toast } from 'sonner';
import { Plus, Pencil, Trash2 } from 'lucide-react';
import { useI18n } from '@/i18n';
import type { RateFunction } from '@/types/rate-function';

interface CreateForm {
  name: string;
  description: string;
  hourly_rate: string;
  sort_order: string;
}

interface EditForm {
  name: string;
  description: string;
  hourly_rate: string;
  is_active: boolean;
  sort_order: string;
}

export function RateFunctionsPage() {
  const { t } = useI18n();
  const { data: functions, isLoading } = useRateFunctions();
  const createFn = useCreateRateFunction();
  const updateFn = useUpdateRateFunction();
  const deleteFn = useDeleteRateFunction();

  const [createOpen, setCreateOpen] = useState(false);
  const [createForm, setCreateForm] = useState<CreateForm>({
    name: '', description: '', hourly_rate: '', sort_order: '0',
  });

  const [editItem, setEditItem] = useState<RateFunction | null>(null);
  const [editForm, setEditForm] = useState<EditForm>({
    name: '', description: '', hourly_rate: '', is_active: true, sort_order: '0',
  });

  function openEdit(rf: RateFunction) {
    setEditItem(rf);
    setEditForm({
      name: rf.name,
      description: rf.description ?? '',
      hourly_rate: String(rf.hourly_rate),
      is_active: rf.is_active,
      sort_order: String(rf.sort_order),
    });
  }

  function handleCreate() {
    if (!createForm.name.trim() || !createForm.hourly_rate) return;
    createFn.mutate(
      {
        name: createForm.name,
        description: createForm.description || undefined,
        hourly_rate: Number(createForm.hourly_rate),
        sort_order: createForm.sort_order ? Number(createForm.sort_order) : undefined,
      },
      {
        onSuccess: () => {
          toast.success(t('projects.rate_function_created', 'Rate function created'));
          setCreateOpen(false);
          setCreateForm({ name: '', description: '', hourly_rate: '', sort_order: '0' });
        },
        onError: () => toast.error(t('projects.rate_function_create_failed', 'Failed to create')),
      },
    );
  }

  function handleUpdate() {
    if (!editItem) return;
    updateFn.mutate(
      {
        id: editItem.id,
        data: {
          name: editForm.name,
          description: editForm.description || undefined,
          hourly_rate: Number(editForm.hourly_rate),
          is_active: editForm.is_active,
          sort_order: Number(editForm.sort_order),
        },
      },
      {
        onSuccess: () => {
          toast.success(t('projects.rate_function_updated', 'Rate function updated'));
          setEditItem(null);
        },
        onError: () => toast.error(t('projects.rate_function_update_failed', 'Failed to update')),
      },
    );
  }

  function handleDelete(id: string) {
    if (!confirm(t('projects.confirm_delete_rate_function', 'Delete this rate function?'))) return;
    deleteFn.mutate(id, {
      onSuccess: () => toast.success(t('projects.rate_function_deleted', 'Rate function deleted')),
      onError: () => toast.error(t('projects.rate_function_delete_failed', 'Failed to delete')),
    });
  }

  const list = functions ?? [];

  return (
    <div className="space-y-4">
      <div className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h2 className="text-lg font-semibold">{t('projects.rate_functions_title', 'Rate Functions')}</h2>
          <p className="text-sm text-muted-foreground">{t('projects.rate_functions_subtitle', 'Manage billing rate functions for project members')}</p>
        </div>
        <Button size="sm" onClick={() => setCreateOpen(true)}>
          <Plus className="mr-1 h-4 w-4" /> {t('projects.add_rate_function', 'Add Rate Function')}
        </Button>
      </div>

      <Card>
        <CardContent className="p-0">
          {isLoading ? (
            <div className="space-y-2 p-4">
              {Array.from({ length: 3 }).map((_, i) => <Skeleton key={i} className="h-10 w-full" />)}
            </div>
          ) : list.length > 0 ? (
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>{t('projects.name', 'Name')}</TableHead>
                  <TableHead className="hidden md:table-cell">{t('common.description', 'Description')}</TableHead>
                  <TableHead className="text-right">{t('projects.hourly_rate_col', 'Hourly Rate')}</TableHead>
                  <TableHead>{t('common.status', 'Status')}</TableHead>
                  <TableHead className="hidden sm:table-cell">{t('projects.sort_order', 'Order')}</TableHead>
                  <TableHead className="w-24">{t('common.actions', 'Actions')}</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {list.map((rf) => (
                  <TableRow key={rf.id}>
                    <TableCell className="font-medium">{rf.name}</TableCell>
                    <TableCell className="hidden md:table-cell text-muted-foreground truncate max-w-xs">
                      {rf.description ?? '—'}
                    </TableCell>
                    <TableCell className="text-right font-mono">CHF {rf.hourly_rate.toFixed(2)}</TableCell>
                    <TableCell>
                      <Badge variant={rf.is_active ? 'default' : 'secondary'}>
                        {rf.is_active ? t('common.active', 'Active') : t('common.inactive', 'Inactive')}
                      </Badge>
                    </TableCell>
                    <TableCell className="hidden sm:table-cell font-mono">{rf.sort_order}</TableCell>
                    <TableCell>
                      <div className="flex gap-1">
                        <Button variant="ghost" size="icon" className="h-8 w-8" onClick={() => openEdit(rf)} title={t('common.edit', 'Edit')}>
                          <Pencil className="h-3.5 w-3.5" />
                        </Button>
                        <Button variant="ghost" size="icon" className="h-8 w-8" onClick={() => handleDelete(rf.id)} title={t('common.delete', 'Delete')}>
                          <Trash2 className="h-3.5 w-3.5" />
                        </Button>
                      </div>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          ) : (
            <p className="py-8 text-center text-sm text-muted-foreground">
              {t('projects.no_rate_functions', 'No rate functions configured.')}
            </p>
          )}
        </CardContent>
      </Card>

      {/* Create Dialog */}
      <Dialog open={createOpen} onOpenChange={setCreateOpen}>
        <DialogContent className="max-w-md">
          <DialogHeader><DialogTitle>{t('projects.new_rate_function', 'New Rate Function')}</DialogTitle></DialogHeader>
          <div className="space-y-4">
            <div><Label>{t('projects.name', 'Name')}</Label><Input value={createForm.name} onChange={(e) => setCreateForm({ ...createForm, name: e.target.value })} placeholder="e.g. Senior Developer" /></div>
            <div><Label>{t('common.description', 'Description')}</Label><RichTextEditor value={createForm.description} onChange={(md) => setCreateForm({ ...createForm, description: md })} /></div>
            <div><Label>{t('projects.hourly_rate_col', 'Hourly Rate (CHF)')}</Label><Input type="number" step="0.01" value={createForm.hourly_rate} onChange={(e) => setCreateForm({ ...createForm, hourly_rate: e.target.value })} placeholder="0.00" /></div>
            <div><Label>{t('projects.sort_order', 'Sort Order')}</Label><Input type="number" value={createForm.sort_order} onChange={(e) => setCreateForm({ ...createForm, sort_order: e.target.value })} /></div>
            <Button onClick={handleCreate} className="w-full" disabled={createFn.isPending || !createForm.name.trim() || !createForm.hourly_rate}>
              {t('common.create', 'Create')}
            </Button>
          </div>
        </DialogContent>
      </Dialog>

      {/* Edit Dialog */}
      <Dialog open={!!editItem} onOpenChange={(open) => !open && setEditItem(null)}>
        <DialogContent className="max-w-md">
          <DialogHeader><DialogTitle>{t('projects.edit_rate_function', 'Edit Rate Function')}</DialogTitle></DialogHeader>
          <div className="space-y-4">
            <div><Label>{t('projects.name', 'Name')}</Label><Input value={editForm.name} onChange={(e) => setEditForm({ ...editForm, name: e.target.value })} /></div>
            <div><Label>{t('common.description', 'Description')}</Label><RichTextEditor value={editForm.description} onChange={(md) => setEditForm({ ...editForm, description: md })} /></div>
            <div><Label>{t('projects.hourly_rate_col', 'Hourly Rate (CHF)')}</Label><Input type="number" step="0.01" value={editForm.hourly_rate} onChange={(e) => setEditForm({ ...editForm, hourly_rate: e.target.value })} /></div>
            <div><Label>{t('projects.sort_order', 'Sort Order')}</Label><Input type="number" value={editForm.sort_order} onChange={(e) => setEditForm({ ...editForm, sort_order: e.target.value })} /></div>
            <div className="flex items-center gap-2">
              <Switch checked={editForm.is_active} onCheckedChange={(v) => setEditForm({ ...editForm, is_active: v })} />
              <Label>{t('common.active', 'Active')}</Label>
            </div>
            <Button onClick={handleUpdate} className="w-full" disabled={updateFn.isPending}>
              {t('common.update', 'Update')}
            </Button>
          </div>
        </DialogContent>
      </Dialog>
    </div>
  );
}
