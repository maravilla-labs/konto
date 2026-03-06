import { useState } from 'react';
import { useProjects, useCreateProject, useContacts } from '@/hooks/useApi';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Skeleton } from '@/components/ui/skeleton';
import { Badge } from '@/components/ui/badge';
import { StickyToolbar, type ToolbarAction } from '@/components/ui/sticky-toolbar';
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import { Plus } from 'lucide-react';
import { Link } from 'react-router-dom';
import { RichTextEditor } from '@/components/ui/rich-text-editor';
import { toast } from 'sonner';
import { SUPPORTED_LANGUAGES } from '@/lib/language';
import { useI18n } from '@/i18n';
import { useSettings } from '@/hooks/useSettingsApi';
import { formatDate } from '@/lib/locale';

const statusVariant: Record<string, 'default' | 'secondary' | 'destructive' | 'outline'> = {
  active: 'default',
  completed: 'secondary',
  on_hold: 'outline',
  cancelled: 'destructive',
  archived: 'secondary',
};

export function ProjectsPage() {
  const { t } = useI18n();
  const { data: settings } = useSettings();
  const [page, setPage] = useState(1);
  const { data, isLoading } = useProjects({ page });
  const createProject = useCreateProject();
  const { data: contactsData } = useContacts({ per_page: 200 });
  const contacts = contactsData?.data ?? [];
  const [open, setOpen] = useState(false);
  const [form, setForm] = useState({
    name: '',
    number: '',
    description: '',
    start_date: '',
    language: '',
    contact_id: '',
    hourly_rate: '',
    soft_budget_hours: '',
    hard_budget_hours: '',
    soft_budget_amount: '',
    hard_budget_amount: '',
    invoicing_method: 'hourly',
    currency: 'CHF',
  });

  const defaultForm = {
    name: '', number: '', description: '', start_date: '', language: '',
    contact_id: '', hourly_rate: '', soft_budget_hours: '', hard_budget_hours: '',
    soft_budget_amount: '', hard_budget_amount: '', invoicing_method: 'hourly', currency: 'CHF',
  };

  function handleCreate() {
    createProject.mutate(
      {
        name: form.name,
        number: form.number || undefined,
        description: form.description || undefined,
        start_date: form.start_date || undefined,
        language: form.language || undefined,
        contact_id: form.contact_id || undefined,
        hourly_rate: form.hourly_rate || undefined,
        soft_budget_hours: form.soft_budget_hours ? Number(form.soft_budget_hours) : undefined,
        hard_budget_hours: form.hard_budget_hours ? Number(form.hard_budget_hours) : undefined,
        soft_budget_amount: form.soft_budget_amount ? Number(form.soft_budget_amount) : undefined,
        hard_budget_amount: form.hard_budget_amount ? Number(form.hard_budget_amount) : undefined,
        invoicing_method: form.invoicing_method || undefined,
        currency: form.currency || undefined,
      },
      {
        onSuccess: () => {
          toast.success(t('projects.created', 'Project created'));
          setOpen(false);
          setForm(defaultForm);
        },
        onError: () => toast.error(t('projects.create_failed', 'Failed to create project')),
      }
    );
  }

  const projects = data?.data ?? [];
  const dateFormat = settings?.date_format ?? 'dd.MM.yyyy';

  const actions: ToolbarAction[] = [
    {
      icon: <Plus className="h-4 w-4" />,
      label: t('projects.new_project', 'New Project'),
      onClick: () => setOpen(true),
      primary: true,
    },
  ];

  return (
    <div className="space-y-4">
      <StickyToolbar actions={actions}>
        <Badge variant="secondary">
          {t('projects.subtitle', 'Manage projects and track time')}
        </Badge>
      </StickyToolbar>

      <Dialog open={open} onOpenChange={setOpen}>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>{t('projects.create_project', 'Create Project')}</DialogTitle>
            </DialogHeader>
            <div className="space-y-4">
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <Label>{t('projects.name', 'Name')}</Label>
                  <Input
                    value={form.name}
                    onChange={(e) => setForm({ ...form, name: e.target.value })}
                    placeholder={t('projects.name_placeholder', 'Project name')}
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
                <Label>{t('common.description', 'Description')}</Label>
                <RichTextEditor
                  value={form.description}
                  onChange={(md) => setForm({ ...form, description: md })}
                  placeholder={t('projects.description_placeholder', 'Optional description')}
                />
              </div>
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <Label>{t('project_conditions.invoicing_method', 'Invoicing Method')}</Label>
                  <Select value={form.invoicing_method} onValueChange={(v) => setForm({ ...form, invoicing_method: v })}>
                    <SelectTrigger><SelectValue /></SelectTrigger>
                    <SelectContent>
                      <SelectItem value="hourly">{t('project_conditions.invoicing_method.hourly', 'Hourly')}</SelectItem>
                      <SelectItem value="fixed_price">{t('project_conditions.invoicing_method.fixed_price', 'Fixed Price')}</SelectItem>
                      <SelectItem value="flat_rate">{t('project_conditions.invoicing_method.flat_rate', 'Flat Rate')}</SelectItem>
                      <SelectItem value="non_billable">{t('project_conditions.invoicing_method.non_billable', 'Non-Billable')}</SelectItem>
                    </SelectContent>
                  </Select>
                </div>
                <div>
                  <Label>{t('project_conditions.currency', 'Currency')}</Label>
                  <Input value={form.currency} onChange={(e) => setForm({ ...form, currency: e.target.value })} placeholder="CHF" />
                </div>
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
                  <Label>{t('projects.hourly_rate_field', 'Hourly Rate (CHF)')}</Label>
                  <Input
                    type="number"
                    step="0.01"
                    value={form.hourly_rate}
                    onChange={(e) => setForm({ ...form, hourly_rate: e.target.value })}
                  />
                </div>
              </div>
              <div>
                <Label>{t('contacts.preferred_language', 'Preferred Language')}</Label>
                <Select
                  value={form.language || '__auto__'}
                  onValueChange={(v) => setForm({ ...form, language: v === '__auto__' ? '' : v })}
                >
                  <SelectTrigger><SelectValue placeholder={t('common.automatic', 'Automatic')} /></SelectTrigger>
                  <SelectContent>
                    <SelectItem value="__auto__">{t('common.automatic', 'Automatic')}</SelectItem>
                    {SUPPORTED_LANGUAGES.map((lang) => (
                      <SelectItem key={lang.code} value={lang.code}>
                        {lang.label}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <Label>{t('projects.soft_budget_hours_field', 'Soft Budget Hours')}</Label>
                  <Input
                    type="number"
                    step="0.5"
                    value={form.soft_budget_hours}
                    onChange={(e) => setForm({ ...form, soft_budget_hours: e.target.value })}
                  />
                </div>
                <div>
                  <Label>{t('projects.hard_budget_hours_field', 'Hard Budget Hours')}</Label>
                  <Input
                    type="number"
                    step="0.5"
                    value={form.hard_budget_hours}
                    onChange={(e) => setForm({ ...form, hard_budget_hours: e.target.value })}
                  />
                </div>
              </div>
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <Label>{t('projects.soft_budget_amount_field', 'Soft Budget (CHF)')}</Label>
                  <Input
                    type="number"
                    step="0.01"
                    value={form.soft_budget_amount}
                    onChange={(e) => setForm({ ...form, soft_budget_amount: e.target.value })}
                  />
                </div>
                <div>
                  <Label>{t('projects.hard_budget_amount_field', 'Hard Budget (CHF)')}</Label>
                  <Input
                    type="number"
                    step="0.01"
                    value={form.hard_budget_amount}
                    onChange={(e) => setForm({ ...form, hard_budget_amount: e.target.value })}
                  />
                </div>
              </div>
              <Button onClick={handleCreate} className="w-full" disabled={createProject.isPending}>
                {t('projects.create_project', 'Create Project')}
              </Button>
            </div>
          </DialogContent>
      </Dialog>

      <Card>
        <CardContent className="p-0">
          {isLoading ? (
            <div className="space-y-2 p-4">
              {Array.from({ length: 5 }).map((_, i) => (
                <Skeleton key={i} className="h-10 w-full" />
              ))}
            </div>
          ) : projects.length > 0 ? (
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>{t('projects.number', 'Nr')}</TableHead>
                  <TableHead>{t('projects.name', 'Name')}</TableHead>
                  <TableHead>{t('common.status', 'Status')}</TableHead>
                  <TableHead className="hidden sm:table-cell">{t('project_conditions.invoicing_method', 'Invoicing')}</TableHead>
                  <TableHead className="hidden md:table-cell">{t('projects.budget', 'Budget')}</TableHead>
                  <TableHead className="hidden lg:table-cell">{t('projects.start_date', 'Start Date')}</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {projects.map((project) => (
                  <TableRow key={project.id}>
                    <TableCell className="font-mono text-muted-foreground">
                      {project.number ?? '—'}
                    </TableCell>
                    <TableCell>
                      <Link to={`/projects/${project.id}`} className="text-primary hover:underline font-medium">
                        {project.name}
                      </Link>
                      {project.description && (
                        <span className="block text-xs text-muted-foreground">
                          {project.description}
                        </span>
                      )}
                    </TableCell>
                    <TableCell>
                      <Badge variant={statusVariant[project.status] ?? 'outline'}>
                        {t(`projects.status.${project.status}`, project.status.replace('_', ' '))}
                      </Badge>
                    </TableCell>
                    <TableCell className="hidden sm:table-cell">
                      <Badge variant="outline" className="text-xs">
                        {t(`project_conditions.invoicing_method.${project.invoicing_method}`, project.invoicing_method)}
                      </Badge>
                    </TableCell>
                    <TableCell className="hidden md:table-cell">
                      {(project.hard_budget_hours || project.soft_budget_hours || project.budget_hours) ? (
                        <Badge variant="outline" className="text-xs">
                          {project.hard_budget_hours
                            ? `${project.hard_budget_hours}h`
                            : project.soft_budget_hours
                            ? `${project.soft_budget_hours}h`
                            : project.budget_hours
                            ? `${project.budget_hours}h`
                            : ''}
                        </Badge>
                      ) : '—'}
                    </TableCell>
                    <TableCell className="hidden lg:table-cell">
                      {project.start_date ? formatDate(project.start_date, dateFormat) : '—'}
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          ) : (
            <p className="py-8 text-center text-sm text-muted-foreground">
              {t('projects.no_results', 'No projects found. Create your first project.')}
            </p>
          )}
        </CardContent>
      </Card>

      {data?.total_pages && data.total_pages > 1 && (
        <div className="flex items-center justify-center gap-2">
          <Button
            variant="outline"
            size="sm"
            onClick={() => setPage((p) => Math.max(1, p - 1))}
            disabled={page <= 1}
          >
            {t('common.previous', 'Previous')}
          </Button>
          <span className="text-sm text-muted-foreground">
            {t('common.page', 'Page')} {data.page} {t('common.of', 'of')} {data.total_pages}
          </span>
          <Button
            variant="outline"
            size="sm"
            onClick={() => setPage((p) => p + 1)}
            disabled={page >= data.total_pages}
          >
            {t('common.next', 'Next')}
          </Button>
        </div>
      )}
    </div>
  );
}
