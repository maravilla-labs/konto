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
import { useCreateProjectMilestone, useUpdateProjectMilestone, useProjectItems } from '@/hooks/useApi';
import { useI18n } from '@/i18n';
import { toast } from 'sonner';
import type { ProjectMilestone } from '@/types/project-milestone';

interface MilestoneDialogProps {
  projectId: string;
  milestone?: ProjectMilestone | null;
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

interface FormState {
  name: string;
  description: string;
  target_date: string;
  project_item_id: string;
}

export function MilestoneDialog({ projectId, milestone, open, onOpenChange }: MilestoneDialogProps) {
  const { t } = useI18n();
  const { data: items } = useProjectItems(projectId);
  const createMilestone = useCreateProjectMilestone();
  const updateMilestone = useUpdateProjectMilestone();

  const isEdit = !!milestone;
  const [form, setForm] = useState<FormState>({
    name: '',
    description: '',
    target_date: '',
    project_item_id: '',
  });

  useEffect(() => {
    if (milestone) {
      setForm({
        name: milestone.name,
        description: milestone.description ?? '',
        target_date: milestone.target_date,
        project_item_id: milestone.project_item_id ?? '',
      });
    } else {
      setForm({ name: '', description: '', target_date: '', project_item_id: '' });
    }
  }, [milestone]);

  // Flatten items for linking
  const flatItems: { id: string; label: string }[] = [];
  (items ?? []).forEach((item) => {
    flatItems.push({ id: item.id, label: `${item.item_type}: ${item.name}` });
    item.children?.forEach((child) => {
      flatItems.push({ id: child.id, label: `  ${child.item_type}: ${child.name}` });
      child.children?.forEach((gc) => {
        flatItems.push({ id: gc.id, label: `    ${gc.item_type}: ${gc.name}` });
      });
    });
  });

  function handleSubmit() {
    if (!form.name.trim() || !form.target_date) return;

    const payload = {
      name: form.name,
      description: form.description || undefined,
      target_date: form.target_date,
      project_item_id: form.project_item_id || undefined,
    };

    if (isEdit) {
      updateMilestone.mutate(
        { milestoneId: milestone!.id, data: payload },
        {
          onSuccess: () => { toast.success(t('projects.milestone_updated', 'Milestone updated')); onOpenChange(false); },
          onError: () => toast.error(t('projects.milestone_update_failed', 'Failed to update milestone')),
        },
      );
    } else {
      createMilestone.mutate(
        { projectId, data: payload },
        {
          onSuccess: () => { toast.success(t('projects.milestone_created', 'Milestone created')); onOpenChange(false); },
          onError: () => toast.error(t('projects.milestone_create_failed', 'Failed to create milestone')),
        },
      );
    }
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-md">
        <DialogHeader>
          <DialogTitle>
            {isEdit ? t('projects.edit_milestone', 'Edit Milestone') : t('projects.create_milestone', 'Create Milestone')}
          </DialogTitle>
        </DialogHeader>
        <div className="space-y-4">
          <div>
            <Label>{t('projects.name', 'Name')}</Label>
            <Input value={form.name} onChange={(e) => setForm({ ...form, name: e.target.value })} placeholder={t('projects.milestone_name_placeholder', 'Milestone name')} />
          </div>

          <div>
            <Label>{t('common.description', 'Description')}</Label>
            <RichTextEditor value={form.description} onChange={(md) => setForm({ ...form, description: md })} />
          </div>

          <div>
            <Label>{t('projects.target_date', 'Target Date')}</Label>
            <Input type="date" value={form.target_date} onChange={(e) => setForm({ ...form, target_date: e.target.value })} />
          </div>

          {flatItems.length > 0 && (
            <div>
              <Label>{t('projects.linked_item', 'Linked Item')}</Label>
              <Select value={form.project_item_id || '__none__'} onValueChange={(v) => setForm({ ...form, project_item_id: v === '__none__' ? '' : v })}>
                <SelectTrigger><SelectValue placeholder={t('common.none', 'None')} /></SelectTrigger>
                <SelectContent>
                  <SelectItem value="__none__">{t('common.none', 'None')}</SelectItem>
                  {flatItems.map((fi) => (
                    <SelectItem key={fi.id} value={fi.id}>{fi.label}</SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
          )}

          <Button onClick={handleSubmit} className="w-full" disabled={createMilestone.isPending || updateMilestone.isPending || !form.name.trim() || !form.target_date}>
            {isEdit ? t('common.update', 'Update') : t('common.create', 'Create')}
          </Button>
        </div>
      </DialogContent>
    </Dialog>
  );
}
