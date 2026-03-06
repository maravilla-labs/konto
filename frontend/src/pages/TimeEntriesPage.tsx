import { useState } from 'react';
import {
  useTimeEntries,
  useCreateTimeEntry,
  useUpdateTimeEntry,
  useDeleteTimeEntry,
  useProjects,
  useContacts,
  useAccounts,
  useCreateInvoiceFromTimeEntries,
  useTransitionTimeEntry,
} from '@/hooks/useApi';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Skeleton } from '@/components/ui/skeleton';
import { Badge } from '@/components/ui/badge';
import { Checkbox } from '@/components/ui/checkbox';
import { StickyToolbar, type ToolbarAction } from '@/components/ui/sticky-toolbar';
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from '@/components/ui/alert-dialog';
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
import { Plus, Pencil, Trash2, FileText } from 'lucide-react';
import { toast } from 'sonner';
import { useNavigate } from 'react-router-dom';
import { TimeEntryForm } from '@/components/time-entries/TimeEntryForm';
import type { TimeEntry } from '@/types/projects';
import { SUPPORTED_LANGUAGES } from '@/lib/language';
import { useI18n } from '@/i18n';
import { useSettings } from '@/hooks/useSettingsApi';
import { formatDate } from '@/lib/locale';

export function TimeEntriesPage() {
  const { language, t } = useI18n();
  const { data: settings } = useSettings();
  const [page, setPage] = useState(1);
  const [billedFilter, setBilledFilter] = useState<string>('all');
  const [statusFilter, setStatusFilter] = useState<string>('all');
  const { data, isLoading } = useTimeEntries({ page });
  const { data: projectsData } = useProjects({ per_page: 200 });
  const { data: contactsData } = useContacts({ per_page: 200 });
  const { data: accountsData } = useAccounts({ per_page: 200 });
  const createEntry = useCreateTimeEntry();
  const updateEntry = useUpdateTimeEntry();
  const deleteEntry = useDeleteTimeEntry();
  const createInvoice = useCreateInvoiceFromTimeEntries();
  const transitionEntry = useTransitionTimeEntry();
  const navigate = useNavigate();
  const projects = projectsData?.data ?? [];
  const contacts = contactsData?.data ?? [];
  const accounts = accountsData?.data ?? [];
  const projectMap = new Map(projects.map(p => [p.id, p.name]));
  const dateFormat = settings?.date_format ?? 'dd.MM.yyyy';

  const [selected, setSelected] = useState<Set<string>>(new Set());
  const [createOpen, setCreateOpen] = useState(false);
  const [editOpen, setEditOpen] = useState(false);
  const [invoiceOpen, setInvoiceOpen] = useState(false);
  const [editId, setEditId] = useState('');
  const [createForm, setCreateForm] = useState({
    project_id: '',
    task_id: '',
    activity_type_id: '',
    quantity: '',
    date: new Date().toISOString().split('T')[0],
    hours: '',
    description: '',
  });
  const [editForm, setEditForm] = useState({
    project_id: '',
    task_id: '',
    activity_type_id: '',
    quantity: '',
    date: '',
    hours: '',
    description: '',
  });
  const [invoiceForm, setInvoiceForm] = useState<{
    contact_id: string;
    project_id: string;
    language: string;
    hourly_rate: string;
    account_id: string;
  }>({
    contact_id: '',
    project_id: '',
    language,
    hourly_rate: '150',
    account_id: '',
  });

  // Filter entries
  const allEntries = data?.data ?? [];
  const filteredByBilled = billedFilter === 'all'
    ? allEntries
    : billedFilter === 'billed'
      ? allEntries.filter(e => e.billed)
      : allEntries.filter(e => !e.billed);
  const entries = statusFilter === 'all'
    ? filteredByBilled
    : filteredByBilled.filter(e => e.status === statusFilter);

  function toggleSelect(id: string) {
    setSelected(prev => {
      const next = new Set(prev);
      if (next.has(id)) next.delete(id);
      else next.add(id);
      return next;
    });
  }

  function toggleAll() {
    const unbilled = entries.filter(e => !e.billed);
    if (selected.size === unbilled.length) {
      setSelected(new Set());
    } else {
      setSelected(new Set(unbilled.map(e => e.id)));
    }
  }

  function handleCreate() {
    const minutes = Math.round(parseFloat(createForm.hours || '0') * 60);
    createEntry.mutate(
      {
        project_id: createForm.project_id || undefined,
        task_id: createForm.task_id || undefined,
        activity_type_id: createForm.activity_type_id || undefined,
        quantity: createForm.quantity ? parseFloat(createForm.quantity) : undefined,
        date: createForm.date,
        actual_minutes: minutes,
        description: createForm.description || undefined,
      },
      {
        onSuccess: () => {
          toast.success(t('time_entries.created', 'Time entry created'));
          setCreateOpen(false);
          setCreateForm({ project_id: '', task_id: '', activity_type_id: '', quantity: '', date: new Date().toISOString().split('T')[0], hours: '', description: '' });
        },
        onError: () => toast.error(t('time_entries.create_failed', 'Failed to create time entry')),
      }
    );
  }

  function openEdit(entry: TimeEntry) {
    setEditId(entry.id);
    setEditForm({
      project_id: entry.project_id ?? '',
      task_id: entry.task_id ?? '',
      activity_type_id: entry.activity_type_id ?? '',
      quantity: entry.quantity != null ? String(entry.quantity) : '',
      date: entry.date,
      hours: (entry.actual_minutes / 60).toFixed(1),
      description: entry.description ?? '',
    });
    setEditOpen(true);
  }

  function handleUpdate() {
    const minutes = Math.round(parseFloat(editForm.hours || '0') * 60);
    updateEntry.mutate(
      {
        id: editId,
        data: {
          project_id: editForm.project_id || undefined,
          task_id: editForm.task_id || undefined,
          activity_type_id: editForm.activity_type_id || undefined,
          quantity: editForm.quantity ? parseFloat(editForm.quantity) : undefined,
          date: editForm.date,
          actual_minutes: minutes,
          description: editForm.description || undefined,
        },
      },
      {
        onSuccess: () => {
          toast.success(t('time_entries.updated', 'Time entry updated'));
          setEditOpen(false);
        },
        onError: () => toast.error(t('time_entries.update_failed', 'Failed to update time entry')),
      }
    );
  }

  const NEXT_STATUSES: Record<string, string[]> = {
    pending: ['in_progress', 'done'],
    in_progress: ['done', 'pending'],
    done: ['in_progress', 'pending'],
    invoiced: ['closed', 'done'],
    closed: ['done'],
  };

  function handleTransition(entryId: string, newStatus: string) {
    transitionEntry.mutate(
      { id: entryId, status: newStatus },
      {
        onSuccess: () => toast.success(t('time_entry_status.transitioned', 'Status updated')),
        onError: () => toast.error(t('time_entry_status.transition_failed', 'Failed to update status')),
      },
    );
  }

  function handleDelete(id: string) {
    deleteEntry.mutate(id, {
      onSuccess: () => toast.success(t('time_entries.deleted', 'Time entry deleted')),
      onError: () => toast.error(t('time_entries.delete_failed', 'Failed to delete time entry')),
    });
  }

  function handleCreateInvoice() {
    if (!invoiceForm.contact_id || !invoiceForm.account_id) {
      toast.error(t('time_entries.contact_revenue_required', 'Contact and revenue account are required'));
      return;
    }
    createInvoice.mutate(
      {
        time_entry_ids: Array.from(selected),
        contact_id: invoiceForm.contact_id,
        project_id: invoiceForm.project_id || undefined,
        language: invoiceForm.language || undefined,
        hourly_rate: invoiceForm.hourly_rate,
        account_id: invoiceForm.account_id,
      },
      {
        onSuccess: (res) => {
          toast.success(t('time_entries.invoice_created', 'Invoice created from time entries'));
          setInvoiceOpen(false);
          setSelected(new Set());
          navigate(`/invoices/${res.data.id}`);
        },
        onError: () => toast.error(t('time_entries.invoice_create_failed', 'Failed to create invoice')),
      }
    );
  }

  const selectedCount = selected.size;

  const actions: ToolbarAction[] = [
    ...(selectedCount > 0
      ? [{
          icon: <FileText className="h-4 w-4" />,
          label: `${t('time_entries.create_invoice', 'Create Invoice')} (${selectedCount})`,
          onClick: () => setInvoiceOpen(true),
        }]
      : []),
    {
      icon: <Plus className="h-4 w-4" />,
      label: t('time_entries.new_entry', 'New Entry'),
      onClick: () => setCreateOpen(true),
      primary: true,
    },
  ];

  return (
    <div className="space-y-4">
      <StickyToolbar actions={actions}>
        <Badge variant="secondary">
          {t('time_entries.subtitle', 'Track time across projects')}
        </Badge>
      </StickyToolbar>

      {/* Create Invoice Dialog */}
      <Dialog open={invoiceOpen} onOpenChange={setInvoiceOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>
              {t('time_entries.create_invoice_from_entries', 'Create Invoice from Time Entries')}
            </DialogTitle>
          </DialogHeader>
          <div className="space-y-4">
            <p className="text-sm text-muted-foreground">
              {selectedCount} {t('time_entries.selected_count', 'time entries selected')}
            </p>
            <div>
              <Label>{t('common.contact', 'Contact')}</Label>
              <Select value={invoiceForm.contact_id} onValueChange={(v) => setInvoiceForm({ ...invoiceForm, contact_id: v })}>
                <SelectTrigger><SelectValue placeholder={t('common.contact', 'Contact')} /></SelectTrigger>
                <SelectContent>
                  {contacts.map(c => <SelectItem key={c.id} value={c.id}>{c.name1}</SelectItem>)}
                </SelectContent>
              </Select>
            </div>
            <div>
              <Label>{t('common.project', 'Project')} ({t('common.optional', 'optional')})</Label>
              <Select value={invoiceForm.project_id} onValueChange={(v) => setInvoiceForm({ ...invoiceForm, project_id: v })}>
                <SelectTrigger><SelectValue placeholder={t('common.project', 'Project')} /></SelectTrigger>
                <SelectContent>
                  {projects.map(p => <SelectItem key={p.id} value={p.id}>{p.name}</SelectItem>)}
                </SelectContent>
              </Select>
            </div>
            <div>
              <Label>{t('common.language', 'Language')}</Label>
              <Select
                value={invoiceForm.language || '__auto__'}
                onValueChange={(v) => setInvoiceForm({ ...invoiceForm, language: v === '__auto__' ? '' : v })}
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
            <div>
              <Label>{t('time_entries.hourly_rate_chf', 'Hourly Rate (CHF)')}</Label>
              <Input
                type="number"
                value={invoiceForm.hourly_rate}
                onChange={(e) => setInvoiceForm({ ...invoiceForm, hourly_rate: e.target.value })}
              />
            </div>
            <div>
              <Label>{t('time_entries.revenue_account', 'Revenue Account')}</Label>
              <Select value={invoiceForm.account_id} onValueChange={(v) => setInvoiceForm({ ...invoiceForm, account_id: v })}>
                <SelectTrigger><SelectValue placeholder={t('common.account', 'Account')} /></SelectTrigger>
                <SelectContent>
                  {accounts.filter(a => a.number >= 3000 && a.number < 4000).map(a => (
                    <SelectItem key={a.id} value={a.id}>{a.number} — {a.name}</SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
            <Button onClick={handleCreateInvoice} className="w-full" disabled={createInvoice.isPending}>
              {t('time_entries.create_invoice', 'Create Invoice')}
            </Button>
          </div>
        </DialogContent>
      </Dialog>

      {/* Create Entry Dialog */}
      <Dialog open={createOpen} onOpenChange={setCreateOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>{t('time_entries.create_entry', 'Create Entry')}</DialogTitle>
          </DialogHeader>
          <TimeEntryForm
            form={createForm}
            setForm={setCreateForm}
            projects={projects}
            onSubmit={handleCreate}
            isPending={createEntry.isPending}
            submitLabel={t('time_entries.create_entry', 'Create Entry')}
          />
        </DialogContent>
      </Dialog>

      {/* Filters */}
      <div className="flex gap-2 flex-wrap">
        {['all', 'unbilled', 'billed'].map((f) => (
          <Button
            key={f}
            size="sm"
            variant={billedFilter === f ? 'default' : 'outline'}
            onClick={() => setBilledFilter(f)}
          >
            {f === 'all'
              ? t('common.all', 'All')
              : f === 'unbilled'
                ? t('time_entries.filter.unbilled', 'Unbilled')
                : t('time_entries.filter.billed', 'Billed')}
          </Button>
        ))}
        <Select value={statusFilter} onValueChange={setStatusFilter}>
          <SelectTrigger className="w-[160px] h-8 text-xs">
            <SelectValue placeholder={t('time_entry_status.filter_status', 'Filter by Status')} />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">{t('common.all', 'All')}</SelectItem>
            <SelectItem value="pending">{t('time_entry_status.pending', 'Pending')}</SelectItem>
            <SelectItem value="in_progress">{t('time_entry_status.in_progress', 'In Progress')}</SelectItem>
            <SelectItem value="done">{t('time_entry_status.done', 'Done')}</SelectItem>
            <SelectItem value="invoiced">{t('time_entry_status.invoiced', 'Invoiced')}</SelectItem>
            <SelectItem value="closed">{t('time_entry_status.closed', 'Closed')}</SelectItem>
          </SelectContent>
        </Select>
      </div>

      <Dialog open={editOpen} onOpenChange={setEditOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>{t('time_entries.edit_entry', 'Edit Time Entry')}</DialogTitle>
          </DialogHeader>
          <TimeEntryForm
            form={editForm}
            setForm={setEditForm}
            projects={projects}
            onSubmit={handleUpdate}
            isPending={updateEntry.isPending}
            submitLabel={t('common.save_changes', 'Save Changes')}
          />
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
          ) : entries.length > 0 ? (
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead className="w-10">
                    <Checkbox
                      checked={selected.size > 0 && selected.size === entries.filter(e => !e.billed).length}
                      onCheckedChange={toggleAll}
                    />
                  </TableHead>
                  <TableHead>{t('common.date', 'Date')}</TableHead>
                  <TableHead>{t('common.project', 'Project')}</TableHead>
                  <TableHead className="hidden lg:table-cell">{t('time_entries.task', 'Task')}</TableHead>
                  <TableHead>{t('common.hours', 'Hours')}</TableHead>
                  <TableHead className="hidden md:table-cell">{t('common.description', 'Description')}</TableHead>
                  <TableHead className="hidden sm:table-cell">{t('common.status', 'Status')}</TableHead>
                  <TableHead>{t('time_entries.filter.billed', 'Billed')}</TableHead>
                  <TableHead className="w-24" />
                </TableRow>
              </TableHeader>
              <TableBody>
                {entries.map((entry) => (
                  <TableRow key={entry.id} className={selected.has(entry.id) ? 'bg-muted/50' : ''}>
                    <TableCell>
                      <Checkbox
                        checked={selected.has(entry.id)}
                        onCheckedChange={() => toggleSelect(entry.id)}
                        disabled={entry.billed}
                      />
                    </TableCell>
                    <TableCell className="font-mono text-sm">{formatDate(entry.date, dateFormat)}</TableCell>
                    <TableCell>{projectMap.get(entry.project_id ?? '') ?? '—'}</TableCell>
                    <TableCell className="hidden lg:table-cell text-sm">
                      {entry.task_name ?? '—'}
                    </TableCell>
                    <TableCell className="font-mono">
                      {(entry.actual_minutes / 60).toFixed(1)}h
                    </TableCell>
                    <TableCell className="hidden md:table-cell max-w-xs truncate">
                      {entry.description ?? '—'}
                    </TableCell>
                    <TableCell className="hidden sm:table-cell">
                      {(() => {
                        const statusColors: Record<string, string> = {
                          pending: 'bg-gray-100 text-gray-700',
                          in_progress: 'bg-blue-100 text-blue-700',
                          done: 'bg-green-100 text-green-700',
                          invoiced: 'bg-purple-100 text-purple-700',
                          closed: 'bg-slate-100 text-slate-700',
                        };
                        const cls = statusColors[entry.status] ?? '';
                        const nextStatuses = NEXT_STATUSES[entry.status] ?? [];
                        if (nextStatuses.length > 0) {
                          return (
                            <Select value={entry.status} onValueChange={(v) => { if (v !== entry.status) handleTransition(entry.id, v); }}>
                              <SelectTrigger className={`h-7 w-auto min-w-[110px] text-xs font-medium rounded-full ${cls}`}>
                                <SelectValue />
                              </SelectTrigger>
                              <SelectContent>
                                <SelectItem value={entry.status}>{t(`time_entry_status.${entry.status}`, entry.status)}</SelectItem>
                                {nextStatuses.map((s) => (
                                  <SelectItem key={s} value={s}>
                                    <span className={`inline-flex items-center rounded-full px-1.5 py-0.5 text-xs ${statusColors[s] ?? ''}`}>
                                      {t(`time_entry_status.${s}`, s)}
                                    </span>
                                  </SelectItem>
                                ))}
                              </SelectContent>
                            </Select>
                          );
                        }
                        return (
                          <Badge variant="outline" className={cls}>
                            {t(`time_entry_status.${entry.status}`, entry.status)}
                          </Badge>
                        );
                      })()}
                    </TableCell>
                    <TableCell>
                      {entry.billed
                        ? <Badge>{t('time_entries.filter.billed', 'Billed')}</Badge>
                        : <Badge variant="outline">{t('time_entries.filter.unbilled', 'Unbilled')}</Badge>
                      }
                    </TableCell>
                    <TableCell>
                      <div className="flex gap-1">
                        <Button variant="ghost" size="icon" onClick={() => openEdit(entry)} title={t('common.update', 'Update')}>
                          <Pencil className="h-3.5 w-3.5" />
                        </Button>
                        <AlertDialog>
                          <AlertDialogTrigger asChild>
                            <Button variant="ghost" size="icon" title={t('common.delete', 'Delete')}>
                              <Trash2 className="h-3.5 w-3.5" />
                            </Button>
                          </AlertDialogTrigger>
                          <AlertDialogContent>
                            <AlertDialogHeader>
                              <AlertDialogTitle>{t('common.delete', 'Delete')}?</AlertDialogTitle>
                              <AlertDialogDescription>
                                {t('time_entries.delete_confirm', 'This will permanently delete this time entry.')}
                              </AlertDialogDescription>
                            </AlertDialogHeader>
                            <AlertDialogFooter>
                              <AlertDialogCancel>{t('common.cancel', 'Cancel')}</AlertDialogCancel>
                              <AlertDialogAction onClick={() => handleDelete(entry.id)}>
                                {t('common.delete', 'Delete')}
                              </AlertDialogAction>
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
              {t('time_entries.no_results', 'No time entries found. Create your first entry or import data.')}
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
