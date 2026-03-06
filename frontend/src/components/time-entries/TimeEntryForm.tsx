import { useEffect, useState, useMemo } from 'react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { Switch } from '@/components/ui/switch';
import { useI18n } from '@/i18n';
import { useActivityTypes, useProjectActivityTypes } from '@/hooks/useApi';
import { projectsApi } from '@/api/projects';
import type { ActivityType } from '@/types/activity-type';

export interface TimeEntryFormData {
  project_id: string;
  task_id: string;
  activity_type_id: string;
  quantity: string;
  date: string;
  hours: string;
  description: string;
  billable?: boolean;
  start_time?: string;
  end_time?: string;
}

interface TimeEntryFormProps {
  form: TimeEntryFormData;
  setForm: (f: TimeEntryFormData) => void;
  projects: { id: string; name: string }[];
  onSubmit: () => void;
  isPending: boolean;
  submitLabel: string;
}

function flattenTasks(items: { item_type: string; id: string; name: string; children?: unknown[] }[]): { id: string; name: string }[] {
  const tasks: { id: string; name: string }[] = [];
  for (const item of items) {
    if (item.item_type === 'task') {
      tasks.push({ id: item.id, name: item.name });
    }
    if (Array.isArray(item.children)) {
      tasks.push(...flattenTasks(item.children as typeof items));
    }
  }
  return tasks;
}

export function TimeEntryForm({
  form,
  setForm,
  projects,
  onSubmit,
  isPending,
  submitLabel,
}: TimeEntryFormProps) {
  const { t } = useI18n();
  const [tasks, setTasks] = useState<{ id: string; name: string }[]>([]);
  const { data: allActivityTypes } = useActivityTypes();
  const { data: projectActivityTypes } = useProjectActivityTypes(form.project_id || undefined);

  useEffect(() => {
    if (!form.project_id) {
      setTasks([]);
      return;
    }
    projectsApi.listItems(form.project_id).then((res) => {
      setTasks(flattenTasks(res.data));
    }).catch(() => setTasks([]));
  }, [form.project_id]);

  const activeTypes = useMemo(() => (allActivityTypes ?? []).filter((at: ActivityType) => at.is_active), [allActivityTypes]);
  const patIds = useMemo(() => new Set((projectActivityTypes ?? []).map((p) => p.activity_type_id)), [projectActivityTypes]);
  const availableTypes = useMemo(
    () => patIds.size > 0 ? activeTypes.filter((at: ActivityType) => patIds.has(at.id)) : activeTypes,
    [activeTypes, patIds],
  );
  const selectedAT = useMemo(
    () => activeTypes.find((at: ActivityType) => at.id === form.activity_type_id),
    [activeTypes, form.activity_type_id],
  );
  const showQuantity = selectedAT && selectedAT.unit_type !== 'hour' && selectedAT.unit_type !== 'fixed';

  return (
    <div className="space-y-4">
      <div>
        <Label>{t('common.project', 'Project')}</Label>
        <Select
          value={form.project_id}
          onValueChange={(v) => setForm({ ...form, project_id: v, task_id: '' })}
        >
          <SelectTrigger>
            <SelectValue placeholder={`${t('common.search', 'Search')} ${t('common.project', 'Project').toLowerCase()}`} />
          </SelectTrigger>
          <SelectContent>
            {projects.map((p) => (
              <SelectItem key={p.id} value={p.id}>{p.name}</SelectItem>
            ))}
          </SelectContent>
        </Select>
      </div>
      {tasks.length > 0 && (
        <div>
          <Label>{t('time_entries.task', 'Task')} ({t('common.optional', 'optional')})</Label>
          <Select
            value={form.task_id || '__none__'}
            onValueChange={(v) => setForm({ ...form, task_id: v === '__none__' ? '' : v })}
          >
            <SelectTrigger>
              <SelectValue placeholder={t('time_entries.select_task', 'Select task')} />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="__none__">{t('common.none', 'None')}</SelectItem>
              {tasks.map((task) => (
                <SelectItem key={task.id} value={task.id}>{task.name}</SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>
      )}
      {availableTypes.length > 0 && (
        <div>
          <Label>{t('time_entries.activity_type', 'Activity Type')}</Label>
          <Select
            value={form.activity_type_id || '__none__'}
            onValueChange={(v) => setForm({ ...form, activity_type_id: v === '__none__' ? '' : v, quantity: '' })}
          >
            <SelectTrigger>
              <SelectValue placeholder={t('time_entries.select_activity_type', 'Select activity type')} />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="__none__">{t('common.none', 'None')}</SelectItem>
              {availableTypes.map((at: ActivityType) => (
                <SelectItem key={at.id} value={at.id}>
                  {at.name} ({at.unit_type}{at.default_rate != null ? ` — ${Number(at.default_rate).toFixed(2)}` : ''})
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>
      )}
      {showQuantity && (
        <div>
          <Label>{t('time_entries.quantity', 'Quantity')} ({selectedAT?.unit_type})</Label>
          <Input
            type="number"
            step="0.01"
            value={form.quantity}
            onChange={(e) => setForm({ ...form, quantity: e.target.value })}
            placeholder="0.00"
          />
        </div>
      )}
      <div className="grid grid-cols-2 gap-4">
        <div>
          <Label>{t('common.date', 'Date')}</Label>
          <Input
            type="date"
            value={form.date}
            onChange={(e) => setForm({ ...form, date: e.target.value })}
          />
        </div>
        <div>
          <Label>{t('common.hours', 'Hours')}</Label>
          <Input
            type="number"
            step="0.1"
            value={form.hours}
            onChange={(e) => setForm({ ...form, hours: e.target.value })}
            placeholder="1.5"
          />
        </div>
      </div>
      <div className="grid grid-cols-2 gap-4">
        <div>
          <Label>{t('time_entry_status.start_time', 'Start Time')}</Label>
          <Input
            type="time"
            value={form.start_time ?? ''}
            onChange={(e) => {
              const startTime = e.target.value;
              const updated = { ...form, start_time: startTime };
              if (startTime && form.end_time) {
                const [sh, sm] = startTime.split(':').map(Number);
                const [eh, em] = form.end_time.split(':').map(Number);
                const diff = (eh * 60 + em) - (sh * 60 + sm);
                if (diff > 0) updated.hours = (diff / 60).toFixed(1);
              }
              setForm(updated);
            }}
          />
        </div>
        <div>
          <Label>{t('time_entry_status.end_time', 'End Time')}</Label>
          <Input
            type="time"
            value={form.end_time ?? ''}
            onChange={(e) => {
              const endTime = e.target.value;
              const updated = { ...form, end_time: endTime };
              if (form.start_time && endTime) {
                const [sh, sm] = form.start_time.split(':').map(Number);
                const [eh, em] = endTime.split(':').map(Number);
                const diff = (eh * 60 + em) - (sh * 60 + sm);
                if (diff > 0) updated.hours = (diff / 60).toFixed(1);
              }
              setForm(updated);
            }}
          />
        </div>
      </div>
      <div className="flex items-center gap-3">
        <Switch
          checked={form.billable ?? true}
          onCheckedChange={(checked) => setForm({ ...form, billable: checked })}
        />
        <Label>{t('time_entry_status.billable', 'Billable')}</Label>
      </div>
      <div>
        <Label>{t('common.description', 'Description')}</Label>
        <Input
          value={form.description}
          onChange={(e) => setForm({ ...form, description: e.target.value })}
          placeholder={t('common.description', 'Description')}
        />
      </div>
      <Button onClick={onSubmit} className="w-full" disabled={isPending}>
        {submitLabel}
      </Button>
    </div>
  );
}
