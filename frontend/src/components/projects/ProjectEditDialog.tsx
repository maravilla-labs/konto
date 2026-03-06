import { useState, useEffect } from 'react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { RichTextEditor } from '@/components/ui/rich-text-editor';
import {
  Dialog, DialogContent, DialogHeader, DialogTitle,
} from '@/components/ui/dialog';
import {
  Select, SelectContent, SelectItem, SelectTrigger, SelectValue,
} from '@/components/ui/select';
import { useContacts, useEmployees, useUpdateProject } from '@/hooks/useApi';
import { SUPPORTED_LANGUAGES } from '@/lib/language';
import { useI18n } from '@/i18n';
import { toast } from 'sonner';
import type { ProjectSummary } from '@/types/projects';

interface ProjectEditDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  summary: ProjectSummary;
}

export function ProjectEditDialog({ open, onOpenChange, summary }: ProjectEditDialogProps) {
  const { t } = useI18n();
  const updateProject = useUpdateProject();
  const { data: contactsData } = useContacts({ per_page: 200 });
  const contacts = contactsData?.data ?? [];
  const { data: employeesData } = useEmployees();
  const employees = employeesData ?? [];

  const [form, setForm] = useState({
    name: '',
    number: '',
    status: '',
    contact_id: '',
    owner_id: '',
    description: '',
    start_date: '',
    end_date: '',
    language: '',
  });

  useEffect(() => {
    if (open) {
      setForm({
        name: summary.name ?? '',
        number: summary.number ?? '',
        status: summary.status ?? 'active',
        contact_id: summary.contact_id ?? '',
        owner_id: summary.owner_id ?? '',
        description: summary.description ?? '',
        start_date: summary.start_date ?? '',
        end_date: summary.end_date ?? '',
        language: summary.language ?? '',
      });
    }
  }, [open, summary]);

  function handleSave() {
    updateProject.mutate(
      {
        id: summary.id,
        data: {
          name: form.name,
          number: form.number || undefined,
          status: form.status,
          contact_id: form.contact_id || undefined,
          owner_id: form.owner_id || undefined,
          description: form.description || undefined,
          start_date: form.start_date || undefined,
          end_date: form.end_date || undefined,
          language: form.language || undefined,
        },
      },
      {
        onSuccess: () => {
          toast.success(t('projects.project_updated', 'Project updated'));
          onOpenChange(false);
        },
        onError: () => toast.error(t('projects.project_update_failed', 'Failed to update project')),
      },
    );
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-lg max-h-[85vh] overflow-y-auto">
        <DialogHeader>
          <DialogTitle>{t('projects.edit_project', 'Edit Project')}</DialogTitle>
        </DialogHeader>
        <div className="space-y-4">
          <div className="grid grid-cols-2 gap-4">
            <div>
              <Label>{t('projects.name', 'Name')}</Label>
              <Input
                value={form.name}
                onChange={(e) => setForm({ ...form, name: e.target.value })}
              />
            </div>
            <div>
              <Label>{t('projects.number_field', 'Project Number')}</Label>
              <Input
                value={form.number}
                onChange={(e) => setForm({ ...form, number: e.target.value })}
                placeholder={t('common.optional', 'optional')}
              />
            </div>
          </div>

          <div>
            <Label>{t('common.status', 'Status')}</Label>
            <Select value={form.status} onValueChange={(v) => setForm({ ...form, status: v })}>
              <SelectTrigger><SelectValue /></SelectTrigger>
              <SelectContent>
                <SelectItem value="active">{t('projects.status.active', 'Active')}</SelectItem>
                <SelectItem value="completed">{t('projects.status.completed', 'Completed')}</SelectItem>
                <SelectItem value="on_hold">{t('projects.status.on_hold', 'On hold')}</SelectItem>
                <SelectItem value="cancelled">{t('projects.status.cancelled', 'Cancelled')}</SelectItem>
              </SelectContent>
            </Select>
          </div>

          <div>
            <Label>{t('projects.contact_field', 'Contact')}</Label>
            <Select
              value={form.contact_id || '__none__'}
              onValueChange={(v) => setForm({ ...form, contact_id: v === '__none__' ? '' : v })}
            >
              <SelectTrigger><SelectValue placeholder={t('common.none', 'None')} /></SelectTrigger>
              <SelectContent>
                <SelectItem value="__none__">{t('common.none', 'None')}</SelectItem>
                {contacts.map((c) => (
                  <SelectItem key={c.id} value={c.id}>{c.name1}</SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>

          <div>
            <Label>{t('projects.owner', 'Owner')}</Label>
            <Select
              value={form.owner_id || '__none__'}
              onValueChange={(v) => setForm({ ...form, owner_id: v === '__none__' ? '' : v })}
            >
              <SelectTrigger><SelectValue placeholder={t('common.none', 'None')} /></SelectTrigger>
              <SelectContent>
                <SelectItem value="__none__">{t('common.none', 'None')}</SelectItem>
                {employees.map((emp) => (
                  <SelectItem key={emp.id} value={emp.id}>
                    {emp.number ? `${emp.number} ` : ''}{emp.first_name} {emp.last_name}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>

          <div>
            <Label>{t('common.description', 'Description')}</Label>
            <RichTextEditor
              value={form.description}
              onChange={(md) => setForm({ ...form, description: md })}
            />
          </div>

          <div className="grid grid-cols-2 gap-4">
            <div>
              <Label>{t('projects.start_date', 'Start Date')}</Label>
              <Input
                type="date"
                value={form.start_date}
                onChange={(e) => setForm({ ...form, start_date: e.target.value })}
              />
            </div>
            <div>
              <Label>{t('projects.end_date', 'End Date')}</Label>
              <Input
                type="date"
                value={form.end_date}
                onChange={(e) => setForm({ ...form, end_date: e.target.value })}
              />
            </div>
          </div>

          <div>
            <Label>{t('common.language', 'Language')}</Label>
            <Select
              value={form.language || '__auto__'}
              onValueChange={(v) => setForm({ ...form, language: v === '__auto__' ? '' : v })}
            >
              <SelectTrigger><SelectValue /></SelectTrigger>
              <SelectContent>
                <SelectItem value="__auto__">{t('common.automatic', 'Automatic')}</SelectItem>
                {SUPPORTED_LANGUAGES.map((lang) => (
                  <SelectItem key={lang.code} value={lang.code}>{lang.label}</SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>

          <Button onClick={handleSave} className="w-full" disabled={updateProject.isPending}>
            {t('common.save_changes', 'Save Changes')}
          </Button>
        </div>
      </DialogContent>
    </Dialog>
  );
}
