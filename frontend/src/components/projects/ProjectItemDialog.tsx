import { useState, useEffect } from 'react';
import {
  Dialog, DialogContent, DialogHeader, DialogTitle,
} from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { RichTextEditor } from '@/components/ui/rich-text-editor';
import {
  Select, SelectContent, SelectItem, SelectTrigger, SelectValue,
} from '@/components/ui/select';
import { useCreateProjectItem, useUpdateProjectItem, useUsers } from '@/hooks/useApi';
import { useI18n } from '@/i18n';
import { toast } from 'sonner';
import type { ProjectItem, ProjectItemType, ProjectItemStatus } from '@/types/project-item';

interface ProjectItemDialogProps {
  projectId: string;
  item?: ProjectItem | null;
  parentId?: string;
  defaultType?: ProjectItemType;
  phases?: ProjectItem[];
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

const ITEM_TYPES: ProjectItemType[] = ['phase', 'work_package', 'task'];
const STATUSES: ProjectItemStatus[] = ['pending', 'in_progress', 'completed', 'cancelled'];

const TYPE_LABELS: Record<ProjectItemType, string> = {
  phase: 'Phase',
  work_package: 'Work Package',
  task: 'Task',
};

const STATUS_LABELS: Record<ProjectItemStatus, string> = {
  pending: 'Pending',
  in_progress: 'In Progress',
  completed: 'Completed',
  cancelled: 'Cancelled',
};

interface FormState {
  item_type: ProjectItemType;
  name: string;
  description: string;
  parent_id: string;
  assignee_id: string;
  start_date: string;
  due_date: string;
  estimated_hours: string;
  budget_hours: string;
  budget_amount: string;
  status: ProjectItemStatus;
}

const emptyForm = (defaultType?: ProjectItemType, parentId?: string): FormState => ({
  item_type: defaultType ?? 'phase',
  name: '',
  description: '',
  parent_id: parentId ?? '',
  assignee_id: '',
  start_date: '',
  due_date: '',
  estimated_hours: '',
  budget_hours: '',
  budget_amount: '',
  status: 'pending',
});

export function ProjectItemDialog({
  projectId, item, parentId, defaultType, phases, open, onOpenChange,
}: ProjectItemDialogProps) {
  const { t } = useI18n();
  const { data: usersRaw } = useUsers();
  const createItem = useCreateProjectItem();
  const updateItem = useUpdateProjectItem();

  const users = usersRaw ?? [];
  const isEdit = !!item;

  const [form, setForm] = useState<FormState>(emptyForm(defaultType, parentId));

  useEffect(() => {
    if (item) {
      setForm({
        item_type: item.item_type,
        name: item.name,
        description: item.description ?? '',
        parent_id: item.parent_id ?? '',
        assignee_id: item.assignee_id ?? '',
        start_date: item.start_date ?? '',
        due_date: item.due_date ?? '',
        estimated_hours: item.estimated_hours != null ? String(item.estimated_hours) : '',
        budget_hours: item.budget_hours != null ? String(item.budget_hours) : '',
        budget_amount: item.budget_amount != null ? String(item.budget_amount) : '',
        status: item.status,
      });
    } else {
      setForm(emptyForm(defaultType, parentId));
    }
  }, [item, defaultType, parentId]);

  function handleSubmit() {
    if (!form.name.trim()) return;

    const payload = {
      item_type: form.item_type,
      name: form.name,
      description: form.description || undefined,
      parent_id: form.parent_id || undefined,
      assignee_id: form.assignee_id || undefined,
      start_date: form.start_date || undefined,
      due_date: form.due_date || undefined,
      estimated_hours: form.estimated_hours ? Number(form.estimated_hours) : undefined,
      budget_hours: form.budget_hours ? Number(form.budget_hours) : undefined,
      budget_amount: form.budget_amount ? Number(form.budget_amount) : undefined,
      status: form.status,
    };

    if (isEdit) {
      updateItem.mutate(
        { itemId: item!.id, data: payload },
        {
          onSuccess: () => { toast.success(t('projects.item_updated', 'Item updated')); onOpenChange(false); },
          onError: () => toast.error(t('projects.item_update_failed', 'Failed to update item')),
        },
      );
    } else {
      createItem.mutate(
        { projectId, data: payload },
        {
          onSuccess: () => { toast.success(t('projects.item_created', 'Item created')); onOpenChange(false); },
          onError: () => toast.error(t('projects.item_create_failed', 'Failed to create item')),
        },
      );
    }
  }

  const parentOptions = (phases ?? []).flatMap((p) => {
    const opts: { id: string; label: string }[] = [{ id: p.id, label: `Phase: ${p.name}` }];
    if (form.item_type === 'task' && p.children) {
      p.children.forEach((wp) => opts.push({ id: wp.id, label: `WP: ${wp.name}` }));
    }
    return opts;
  });

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-lg max-h-[90vh] overflow-y-auto">
        <DialogHeader>
          <DialogTitle>
            {isEdit ? t('projects.edit_item', 'Edit Item') : t('projects.create_item', 'Create Item')}
          </DialogTitle>
        </DialogHeader>
        <div className="space-y-4">
          <div className="grid grid-cols-2 gap-4">
            <div>
              <Label>{t('common.type', 'Type')}</Label>
              <Select value={form.item_type} onValueChange={(v) => setForm({ ...form, item_type: v as ProjectItemType })} disabled={isEdit}>
                <SelectTrigger><SelectValue /></SelectTrigger>
                <SelectContent>
                  {ITEM_TYPES.map((tp) => (
                    <SelectItem key={tp} value={tp}>{TYPE_LABELS[tp]}</SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
            <div>
              <Label>{t('common.status', 'Status')}</Label>
              <Select value={form.status} onValueChange={(v) => setForm({ ...form, status: v as ProjectItemStatus })}>
                <SelectTrigger><SelectValue /></SelectTrigger>
                <SelectContent>
                  {STATUSES.map((s) => (
                    <SelectItem key={s} value={s}>{STATUS_LABELS[s]}</SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
          </div>

          <div>
            <Label>{t('projects.name', 'Name')}</Label>
            <Input value={form.name} onChange={(e) => setForm({ ...form, name: e.target.value })} placeholder={t('projects.item_name_placeholder', 'Item name')} />
          </div>

          <div>
            <Label>{t('common.description', 'Description')}</Label>
            <RichTextEditor value={form.description} onChange={(md) => setForm({ ...form, description: md })} />
          </div>

          {form.item_type !== 'phase' && parentOptions.length > 0 && (
            <div>
              <Label>{t('projects.parent_item', 'Parent Item')}</Label>
              <Select value={form.parent_id || '__none__'} onValueChange={(v) => setForm({ ...form, parent_id: v === '__none__' ? '' : v })}>
                <SelectTrigger><SelectValue /></SelectTrigger>
                <SelectContent>
                  <SelectItem value="__none__">{t('common.none', 'None')}</SelectItem>
                  {parentOptions.map((o) => (
                    <SelectItem key={o.id} value={o.id}>{o.label}</SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
          )}

          <div>
            <Label>{t('projects.assignee', 'Assignee')}</Label>
            <Select value={form.assignee_id || '__none__'} onValueChange={(v) => setForm({ ...form, assignee_id: v === '__none__' ? '' : v })}>
              <SelectTrigger><SelectValue placeholder={t('common.none', 'None')} /></SelectTrigger>
              <SelectContent>
                <SelectItem value="__none__">{t('common.none', 'None')}</SelectItem>
                {users.map((u) => (
                  <SelectItem key={u.id} value={u.id}>{u.full_name}</SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>

          <div className="grid grid-cols-2 gap-4">
            <div>
              <Label>{t('projects.start_date', 'Start Date')}</Label>
              <Input type="date" value={form.start_date} onChange={(e) => setForm({ ...form, start_date: e.target.value })} />
            </div>
            <div>
              <Label>{t('common.due_date', 'Due Date')}</Label>
              <Input type="date" value={form.due_date} onChange={(e) => setForm({ ...form, due_date: e.target.value })} />
            </div>
          </div>

          <div className="grid grid-cols-3 gap-4">
            <div>
              <Label>{t('projects.estimated_hours', 'Est. Hours')}</Label>
              <Input type="number" step="0.5" value={form.estimated_hours} onChange={(e) => setForm({ ...form, estimated_hours: e.target.value })} />
            </div>
            <div>
              <Label>{t('projects.budget_hours_field', 'Budget Hours')}</Label>
              <Input type="number" step="0.5" value={form.budget_hours} onChange={(e) => setForm({ ...form, budget_hours: e.target.value })} />
            </div>
            <div>
              <Label>{t('projects.budget_amount_field', 'Budget CHF')}</Label>
              <Input type="number" step="0.01" value={form.budget_amount} onChange={(e) => setForm({ ...form, budget_amount: e.target.value })} />
            </div>
          </div>

          <Button onClick={handleSubmit} className="w-full" disabled={createItem.isPending || updateItem.isPending || !form.name.trim()}>
            {isEdit ? t('common.update', 'Update') : t('common.create', 'Create')}
          </Button>
        </div>
      </DialogContent>
    </Dialog>
  );
}
