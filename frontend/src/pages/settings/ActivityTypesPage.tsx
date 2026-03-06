import { useState } from 'react';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Switch } from '@/components/ui/switch';
import { Badge } from '@/components/ui/badge';
import { Skeleton } from '@/components/ui/skeleton';
import {
  Dialog, DialogContent, DialogHeader, DialogTitle,
} from '@/components/ui/dialog';
import {
  Select, SelectContent, SelectItem, SelectTrigger, SelectValue,
} from '@/components/ui/select';
import {
  Table, TableBody, TableCell, TableHead, TableHeader, TableRow,
} from '@/components/ui/table';
import { useActivityTypes, useCreateActivityType, useUpdateActivityType, useDeleteActivityType } from '@/hooks/useApi';
import { useI18n } from '@/i18n';
import { toast } from 'sonner';
import { Plus, Pencil, Trash2 } from 'lucide-react';
import { StickyToolbar, type ToolbarAction } from '@/components/ui/sticky-toolbar';
import type { ActivityType } from '@/types/activity-type';

const UNIT_PRESETS = ['hour', 'fixed', 'day', 'sqm', 'piece', 'km'];

export function ActivityTypesPage() {
  const { t } = useI18n();
  const { data: types, isLoading } = useActivityTypes();
  const createType = useCreateActivityType();
  const updateType = useUpdateActivityType();
  const deleteType = useDeleteActivityType();

  const [createOpen, setCreateOpen] = useState(false);
  const [createForm, setCreateForm] = useState({ name: '', unit_type: 'hour', default_rate: '' });

  const [editType, setEditType] = useState<ActivityType | null>(null);
  const [editForm, setEditForm] = useState({ name: '', is_active: true, unit_type: 'hour', default_rate: '' });

  function openEdit(at: ActivityType) {
    setEditType(at);
    setEditForm({
      name: at.name,
      is_active: at.is_active,
      unit_type: at.unit_type || 'hour',
      default_rate: at.default_rate != null ? String(at.default_rate) : '',
    });
  }

  function handleCreate() {
    const rate = createForm.default_rate ? parseFloat(createForm.default_rate) : undefined;
    createType.mutate(
      { name: createForm.name, unit_type: createForm.unit_type || 'hour', default_rate: rate },
      {
        onSuccess: () => {
          toast.success(t('activity_types.created', 'Activity type created'));
          setCreateOpen(false);
          setCreateForm({ name: '', unit_type: 'hour', default_rate: '' });
        },
        onError: () => toast.error(t('activity_types.create_failed', 'Failed to create')),
      },
    );
  }

  function handleUpdate() {
    if (!editType) return;
    const rate = editForm.default_rate ? parseFloat(editForm.default_rate) : null;
    updateType.mutate(
      {
        id: editType.id,
        data: {
          name: editForm.name,
          is_active: editForm.is_active,
          unit_type: editForm.unit_type || 'hour',
          default_rate: rate,
        },
      },
      {
        onSuccess: () => {
          toast.success(t('activity_types.updated', 'Activity type updated'));
          setEditType(null);
        },
        onError: () => toast.error(t('activity_types.update_failed', 'Failed to update')),
      },
    );
  }

  function handleDelete(id: string) {
    if (!confirm(t('activity_types.confirm_delete', 'Delete this activity type?'))) return;
    deleteType.mutate(id, {
      onSuccess: () => toast.success(t('activity_types.deleted', 'Activity type deleted')),
      onError: () => toast.error(t('activity_types.delete_failed', 'Failed to delete')),
    });
  }

  function unitLabel(unit: string): string {
    const key = `activity_types.unit_${unit}`;
    return t(key, unit);
  }

  const list = types ?? [];

  return (
    <div className="space-y-4">
      <StickyToolbar
        actions={[
          { icon: <Plus className="h-4 w-4" />, label: t('activity_types.add', 'Add Activity Type'), onClick: () => setCreateOpen(true), primary: true },
        ] satisfies ToolbarAction[]}
      >
        <Badge variant="secondary">{list.length} {t('activity_types.title', 'Activity Types')}</Badge>
      </StickyToolbar>

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
                  <TableHead>{t('common.name', 'Name')}</TableHead>
                  <TableHead>{t('activity_types.unit_type', 'Unit Type')}</TableHead>
                  <TableHead>{t('activity_types.default_rate', 'Default Rate')}</TableHead>
                  <TableHead>{t('common.status', 'Status')}</TableHead>
                  <TableHead className="w-24">{t('common.actions', 'Actions')}</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {list.map((at) => (
                  <TableRow key={at.id}>
                    <TableCell className="font-medium">{at.name}</TableCell>
                    <TableCell>{unitLabel(at.unit_type)}</TableCell>
                    <TableCell className="font-mono">
                      {at.default_rate != null ? Number(at.default_rate).toFixed(2) : '—'}
                    </TableCell>
                    <TableCell>
                      <Badge variant={at.is_active ? 'default' : 'secondary'}>
                        {at.is_active ? t('common.active', 'Active') : t('common.inactive', 'Inactive')}
                      </Badge>
                    </TableCell>
                    <TableCell>
                      <div className="flex gap-1">
                        <Button variant="ghost" size="icon" className="h-8 w-8" onClick={() => openEdit(at)} title={t('common.edit', 'Edit')}>
                          <Pencil className="h-3.5 w-3.5" />
                        </Button>
                        <Button variant="ghost" size="icon" className="h-8 w-8" onClick={() => handleDelete(at.id)} title={t('common.delete', 'Delete')}>
                          <Trash2 className="h-3.5 w-3.5" />
                        </Button>
                      </div>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          ) : (
            <p className="py-8 text-center text-sm text-muted-foreground">{t('activity_types.no_results', 'No activity types configured.')}</p>
          )}
        </CardContent>
      </Card>

      {/* Create Dialog */}
      <Dialog open={createOpen} onOpenChange={setCreateOpen}>
        <DialogContent className="max-w-sm">
          <DialogHeader><DialogTitle>{t('activity_types.add', 'New Activity Type')}</DialogTitle></DialogHeader>
          <div className="space-y-4">
            <div>
              <Label>{t('common.name', 'Name')}</Label>
              <Input value={createForm.name} onChange={(e) => setCreateForm({ ...createForm, name: e.target.value })} placeholder="e.g. Development" />
            </div>
            <div>
              <Label>{t('activity_types.unit_type', 'Unit Type')}</Label>
              <Select value={createForm.unit_type} onValueChange={(v) => setCreateForm({ ...createForm, unit_type: v })}>
                <SelectTrigger><SelectValue /></SelectTrigger>
                <SelectContent>
                  {UNIT_PRESETS.map((u) => (
                    <SelectItem key={u} value={u}>{unitLabel(u)}</SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
            <div>
              <Label>{t('activity_types.default_rate', 'Default Rate')}</Label>
              <Input
                type="number"
                step="0.01"
                value={createForm.default_rate}
                onChange={(e) => setCreateForm({ ...createForm, default_rate: e.target.value })}
                placeholder="0.00"
              />
            </div>
            <Button onClick={handleCreate} className="w-full" disabled={createType.isPending || !createForm.name.trim()}>
              {t('common.create', 'Create')}
            </Button>
          </div>
        </DialogContent>
      </Dialog>

      {/* Edit Dialog */}
      <Dialog open={!!editType} onOpenChange={(open) => !open && setEditType(null)}>
        <DialogContent className="max-w-sm">
          <DialogHeader><DialogTitle>{t('activity_types.edit', 'Edit Activity Type')}</DialogTitle></DialogHeader>
          <div className="space-y-4">
            <div>
              <Label>{t('common.name', 'Name')}</Label>
              <Input value={editForm.name} onChange={(e) => setEditForm({ ...editForm, name: e.target.value })} />
            </div>
            <div>
              <Label>{t('activity_types.unit_type', 'Unit Type')}</Label>
              <Select value={editForm.unit_type} onValueChange={(v) => setEditForm({ ...editForm, unit_type: v })}>
                <SelectTrigger><SelectValue /></SelectTrigger>
                <SelectContent>
                  {UNIT_PRESETS.map((u) => (
                    <SelectItem key={u} value={u}>{unitLabel(u)}</SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
            <div>
              <Label>{t('activity_types.default_rate', 'Default Rate')}</Label>
              <Input
                type="number"
                step="0.01"
                value={editForm.default_rate}
                onChange={(e) => setEditForm({ ...editForm, default_rate: e.target.value })}
                placeholder="0.00"
              />
            </div>
            <div className="flex items-center gap-2">
              <Switch checked={editForm.is_active} onCheckedChange={(v) => setEditForm({ ...editForm, is_active: v })} />
              <Label>{t('common.active', 'Active')}</Label>
            </div>
            <Button onClick={handleUpdate} className="w-full" disabled={updateType.isPending}>
              {t('common.update', 'Update')}
            </Button>
          </div>
        </DialogContent>
      </Dialog>
    </div>
  );
}
