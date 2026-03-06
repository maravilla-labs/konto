import { useState, useMemo, useEffect } from 'react';
import { Card, CardContent } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Checkbox } from '@/components/ui/checkbox';
import {
  Dialog, DialogContent, DialogHeader, DialogTitle, DialogTrigger, DialogFooter,
} from '@/components/ui/dialog';
import {
  Select, SelectContent, SelectItem, SelectTrigger, SelectValue,
} from '@/components/ui/select';
import {
  Table, TableBody, TableCell, TableHead, TableHeader, TableRow,
} from '@/components/ui/table';
import {
  useTimeEntries, useCreateTimeEntry, useUpdateTimeEntry, useUsers,
  useCreateInvoiceFromTimeEntries, useDefaultAccounts,
  useTransitionTimeEntry,
} from '@/hooks/useApi';
import { useAuthStore } from '@/stores/authStore';
import { useI18n } from '@/i18n';
import { toast } from 'sonner';
import { useNavigate } from 'react-router-dom';
import { Plus, ReceiptText, Pencil } from 'lucide-react';
import type { ProjectSummary, TimeEntry } from '@/types/projects';
import type { ActivityType } from '@/types/activity-type';
import { useActivityTypes, useProjectActivityTypes } from '@/hooks/useApi';
import { projectsApi } from '@/api/projects';

type GroupBy = 'none' | 'user' | 'task';

const STATUS_COLORS: Record<string, string> = {
  pending: 'bg-gray-100 text-gray-700',
  in_progress: 'bg-blue-100 text-blue-700',
  done: 'bg-green-100 text-green-700',
  invoiced: 'bg-purple-100 text-purple-700',
  closed: 'bg-slate-100 text-slate-700',
};

const NEXT_STATUSES: Record<string, string[]> = {
  pending: ['in_progress', 'done'],
  in_progress: ['done', 'pending'],
  done: ['in_progress', 'pending'],
  invoiced: ['closed', 'done'],
  closed: ['done'],
};

function flattenTasks(items: { item_type: string; id: string; name: string; children?: unknown[] }[]): { id: string; name: string }[] {
  const tasks: { id: string; name: string }[] = [];
  for (const item of items) {
    if (item.item_type === 'task') tasks.push({ id: item.id, name: item.name });
    if (Array.isArray(item.children)) tasks.push(...flattenTasks(item.children as typeof items));
  }
  return tasks;
}

interface ProjectTimeTrackingTabProps {
  projectId: string;
  summary: ProjectSummary;
}

export function ProjectTimeTrackingTab({ projectId, summary }: ProjectTimeTrackingTabProps) {
  const { t } = useI18n();
  const navigate = useNavigate();
  const currentUser = useAuthStore((s) => s.user);
  const isAdmin = currentUser?.role === 'admin';
  const { data: timeData } = useTimeEntries({ project_id: projectId, per_page: 200 });
  const { data: defaultAccounts } = useDefaultAccounts();
  const { data: usersData } = useUsers();
  const { data: allActivityTypes } = useActivityTypes();
  const { data: projectActivityTypes } = useProjectActivityTypes(projectId);
  const createEntry = useCreateTimeEntry();
  const updateEntry = useUpdateTimeEntry();
  const createInvoice = useCreateInvoiceFromTimeEntries();
  const transitionEntry = useTransitionTimeEntry();

  const [selected, setSelected] = useState<Set<string>>(new Set());
  const [addOpen, setAddOpen] = useState(false);
  const [editOpen, setEditOpen] = useState(false);
  const [invoicePreviewOpen, setInvoicePreviewOpen] = useState(false);
  const [editId, setEditId] = useState('');
  const [editForm, setEditForm] = useState({ date: '', hours: '', description: '' });
  const [groupBy, setGroupBy] = useState<GroupBy>('none');
  const [tasks, setTasks] = useState<{ id: string; name: string }[]>([]);
  const [form, setForm] = useState({
    task_id: '', date: '', hours: '', description: '', user_id: '',
    activity_type_id: '', quantity: '', billable: true, start_time: '', end_time: '',
  });

  useEffect(() => {
    projectsApi.listItems(projectId).then((res) => {
      setTasks(flattenTasks(res.data));
    }).catch(() => setTasks([]));
  }, [projectId]);

  const users = usersData ?? [];
  const userMap = useMemo(() => new Map(users.map((u) => [u.id, u.full_name])), [users]);
  const taskMap = useMemo(() => new Map(tasks.map((t) => [t.id, t.name])), [tasks]);
  const atMap = useMemo(() => new Map((allActivityTypes ?? []).map((at: ActivityType) => [at.id, at.name])), [allActivityTypes]);

  // Build rate lookup: activity_type_id → effective rate (project override > default)
  const rateMap = useMemo(() => {
    const map = new Map<string, number>();
    const pats = projectActivityTypes ?? [];
    const ats = allActivityTypes ?? [];
    for (const pat of pats) {
      const at = ats.find((a: ActivityType) => a.id === pat.activity_type_id);
      const effectiveRate = pat.rate != null ? Number(pat.rate) : (at?.default_rate != null ? Number(at.default_rate) : null);
      if (effectiveRate != null) map.set(pat.activity_type_id, effectiveRate);
    }
    // Also add rates for activity types not in project (fallback)
    for (const at of ats) {
      if (!map.has(at.id) && at.default_rate != null) {
        map.set(at.id, Number(at.default_rate));
      }
    }
    return map;
  }, [projectActivityTypes, allActivityTypes]);

  function resolveEntryRate(te: TimeEntry): number | null {
    if (te.activity_type_id && rateMap.has(te.activity_type_id)) {
      return rateMap.get(te.activity_type_id)!;
    }
    if (summary.hourly_rate) return Number(summary.hourly_rate);
    return null;
  }

  const entries = timeData?.data ?? [];
  const unbilled = useMemo(() => entries.filter((e) => !e.billed), [entries]);

  const activeTypes = useMemo(() => (allActivityTypes ?? []).filter((at: ActivityType) => at.is_active), [allActivityTypes]);
  const patIds = useMemo(() => new Set((projectActivityTypes ?? []).map((p) => p.activity_type_id)), [projectActivityTypes]);
  const entryActivityTypes = useMemo(
    () => patIds.size > 0 ? activeTypes.filter((at: ActivityType) => patIds.has(at.id)) : activeTypes,
    [activeTypes, patIds],
  );
  const selectedAT = useMemo(
    () => activeTypes.find((at: ActivityType) => at.id === form.activity_type_id),
    [activeTypes, form.activity_type_id],
  );
  const showQuantity = selectedAT && selectedAT.unit_type !== 'hour' && selectedAT.unit_type !== 'fixed';

  const revenueAccount = defaultAccounts?.find((a) => a.setting_key === 'revenue');
  const totalHours = useMemo(() => entries.reduce((sum, e) => sum + e.actual_minutes / 60, 0), [entries]);

  // Selected entries preview for invoice
  const selectedEntries = useMemo(() => entries.filter((e) => selected.has(e.id)), [entries, selected]);
  const invoicePreview = useMemo(() => {
    let totalAmount = 0;
    let totalHrs = 0;
    const lines = selectedEntries.map((e) => {
      const hours = e.actual_minutes / 60;
      const rate = resolveEntryRate(e);
      const amount = rate != null ? hours * rate : 0;
      totalHrs += hours;
      totalAmount += amount;
      return {
        id: e.id,
        date: e.date,
        description: e.description || `${e.date} — ${e.actual_minutes} min`,
        activity: e.activity_type_id ? (atMap.get(e.activity_type_id) ?? '—') : '—',
        hours,
        rate,
        amount,
      };
    });
    return { lines, totalHrs, totalAmount };
  }, [selectedEntries, atMap]);

  const grouped = useMemo(() => {
    if (groupBy === 'none') return null;
    const map = new Map<string, TimeEntry[]>();
    for (const e of entries) {
      const key = groupBy === 'user' ? (e.user_id ?? 'unknown') : (e.task_id ?? 'no-task');
      if (!map.has(key)) map.set(key, []);
      map.get(key)!.push(e);
    }
    return map;
  }, [entries, groupBy]);

  function getGroupLabel(key: string): string {
    if (groupBy === 'user') return userMap.get(key) ?? key;
    if (groupBy === 'task') return key === 'no-task' ? '—' : (taskMap.get(key) ?? key);
    return key;
  }

  function getGroupHours(items: TimeEntry[]): number {
    return items.reduce((sum, e) => sum + e.actual_minutes / 60, 0);
  }

  function toggleEntry(id: string) {
    setSelected((prev) => {
      const next = new Set(prev);
      if (next.has(id)) next.delete(id); else next.add(id);
      return next;
    });
  }

  function toggleAll() {
    if (selected.size === unbilled.length) setSelected(new Set());
    else setSelected(new Set(unbilled.map((e) => e.id)));
  }

  function openEdit(te: TimeEntry) {
    setEditId(te.id);
    setEditForm({
      date: te.date,
      hours: (te.actual_minutes / 60).toFixed(1),
      description: te.description ?? '',
    });
    setEditOpen(true);
  }

  function handleUpdate() {
    const minutes = Math.round(parseFloat(editForm.hours || '0') * 60);
    updateEntry.mutate(
      {
        id: editId,
        data: {
          date: editForm.date,
          actual_minutes: minutes,
          description: editForm.description || undefined,
        },
      },
      {
        onSuccess: () => { toast.success(t('time_entries.updated', 'Time entry updated')); setEditOpen(false); },
        onError: () => toast.error(t('time_entries.update_failed', 'Failed to update time entry')),
      },
    );
  }

  function handleTransition(entryId: string, newStatus: string) {
    transitionEntry.mutate(
      { id: entryId, status: newStatus },
      {
        onSuccess: () => toast.success(t('time_entry_status.transitioned', 'Status updated')),
        onError: () => toast.error(t('time_entry_status.transition_failed', 'Failed to update status')),
      },
    );
  }

  function handleAddEntry() {
    const minutes = Math.round(parseFloat(form.hours) * 60);
    if (!form.date || isNaN(minutes) || minutes <= 0) return;
    const qty = form.quantity ? parseFloat(form.quantity) : undefined;
    createEntry.mutate(
      {
        project_id: projectId,
        task_id: form.task_id || undefined,
        user_id: form.user_id || undefined,
        activity_type_id: form.activity_type_id || undefined,
        date: form.date,
        actual_minutes: minutes,
        description: form.description || undefined,
        quantity: qty,
      },
      {
        onSuccess: () => {
          toast.success(t('time_entries.created', 'Time entry created'));
          setAddOpen(false);
          setForm({ task_id: '', date: '', hours: '', description: '', user_id: '', activity_type_id: '', quantity: '', billable: true, start_time: '', end_time: '' });
        },
        onError: () => toast.error(t('time_entries.create_failed', 'Failed to create time entry')),
      },
    );
  }

  function handleCreateInvoice() {
    if (!summary.contact_id) {
      toast.error(t('projects.no_contact', 'No contact assigned — cannot create invoice.'));
      return;
    }
    if (!revenueAccount?.account_id) {
      toast.error('Revenue account not configured');
      return;
    }
    createInvoice.mutate(
      {
        time_entry_ids: Array.from(selected),
        contact_id: summary.contact_id,
        project_id: projectId,
        language: summary.language,
        hourly_rate: summary.hourly_rate ? String(summary.hourly_rate) : undefined,
        account_id: revenueAccount.account_id,
      },
      {
        onSuccess: (resp) => {
          toast.success(t('projects.invoice_created_from_entries', 'Invoice created from time entries'));
          setSelected(new Set());
          setInvoicePreviewOpen(false);
          navigate(`/invoices/${resp.data.id}`);
        },
        onError: () => toast.error(t('projects.invoice_create_failed', 'Failed to create invoice from time entries')),
      },
    );
  }

  function renderEntryRow(te: TimeEntry) {
    const nextStatuses = NEXT_STATUSES[te.status] ?? [];
    const rate = resolveEntryRate(te);
    const hours = te.actual_minutes / 60;
    return (
      <TableRow key={te.id} className={te.billed ? 'opacity-60' : ''}>
        <TableCell>
          {!te.billed && (
            <Checkbox checked={selected.has(te.id)} onCheckedChange={() => toggleEntry(te.id)} />
          )}
        </TableCell>
        <TableCell className="font-mono">{te.date}</TableCell>
        <TableCell className="font-mono">{hours.toFixed(1)}h</TableCell>
        <TableCell className="hidden lg:table-cell text-muted-foreground">
          {te.user_id ? (userMap.get(te.user_id) ?? te.user_id.slice(0, 8)) : '—'}
        </TableCell>
        <TableCell className="hidden md:table-cell max-w-xs truncate">{te.description ?? '—'}</TableCell>
        <TableCell className="hidden sm:table-cell text-muted-foreground">
          {te.activity_type_id ? (atMap.get(te.activity_type_id) ?? '—') : '—'}
        </TableCell>
        <TableCell className="hidden sm:table-cell font-mono text-right">
          {rate != null ? rate.toFixed(2) : '—'}
        </TableCell>
        <TableCell className="hidden sm:table-cell font-mono text-right">
          {rate != null ? (hours * rate).toFixed(2) : '—'}
        </TableCell>
        <TableCell>
          {nextStatuses.length > 0 ? (
            <Select value={te.status} onValueChange={(v) => { if (v !== te.status) handleTransition(te.id, v); }}>
              <SelectTrigger className={`h-7 w-auto min-w-[110px] text-xs font-medium rounded-full ${STATUS_COLORS[te.status] ?? ''}`}>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value={te.status}>{t(`time_entry_status.${te.status}`, te.status)}</SelectItem>
                {nextStatuses.map((s) => (
                  <SelectItem key={s} value={s}>
                    <span className={`inline-flex items-center rounded-full px-1.5 py-0.5 text-xs ${STATUS_COLORS[s] ?? ''}`}>
                      {t(`time_entry_status.${s}`, s)}
                    </span>
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          ) : (
            <Badge variant="outline" className={STATUS_COLORS[te.status] ?? ''}>
              {t(`time_entry_status.${te.status}`, te.status)}
            </Badge>
          )}
        </TableCell>
        <TableCell>
          <Button variant="ghost" size="icon" className="h-7 w-7" onClick={() => openEdit(te)}>
            <Pencil className="h-3.5 w-3.5" />
          </Button>
        </TableCell>
      </TableRow>
    );
  }

  return (
    <div className="space-y-4 pt-2">
      {/* Edit Dialog */}
      <Dialog open={editOpen} onOpenChange={setEditOpen}>
        <DialogContent className="max-w-sm">
          <DialogHeader>
            <DialogTitle>{t('time_entries.edit_entry', 'Edit Time Entry')}</DialogTitle>
          </DialogHeader>
          <div className="space-y-4">
            <div className="grid grid-cols-2 gap-4">
              <div>
                <Label>{t('common.date', 'Date')}</Label>
                <Input type="date" value={editForm.date} onChange={(e) => setEditForm({ ...editForm, date: e.target.value })} />
              </div>
              <div>
                <Label>{t('common.hours', 'Hours')}</Label>
                <Input type="number" step="0.1" value={editForm.hours} onChange={(e) => setEditForm({ ...editForm, hours: e.target.value })} />
              </div>
            </div>
            <div>
              <Label>{t('common.description', 'Description')}</Label>
              <Input value={editForm.description} onChange={(e) => setEditForm({ ...editForm, description: e.target.value })} />
            </div>
            <Button onClick={handleUpdate} className="w-full" disabled={updateEntry.isPending}>
              {t('common.save_changes', 'Save Changes')}
            </Button>
          </div>
        </DialogContent>
      </Dialog>

      {/* Invoice Preview Dialog */}
      <Dialog open={invoicePreviewOpen} onOpenChange={setInvoicePreviewOpen}>
        <DialogContent className="max-w-2xl">
          <DialogHeader>
            <DialogTitle>{t('projects.create_invoice_from_selected', 'Create Invoice from Selected')}</DialogTitle>
          </DialogHeader>
          <div className="space-y-4">
            <div className="grid grid-cols-2 gap-4 text-sm">
              <div>
                <span className="text-muted-foreground">{t('common.project', 'Project')}:</span>{' '}
                <span className="font-medium">{summary.name}</span>
              </div>
              <div>
                <span className="text-muted-foreground">{t('common.contact', 'Contact')}:</span>{' '}
                <span className="font-medium">{summary.contact_name ?? '—'}</span>
              </div>
            </div>
            <div className="max-h-64 overflow-y-auto border rounded-md">
              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead>{t('common.date', 'Date')}</TableHead>
                    <TableHead>{t('common.description', 'Description')}</TableHead>
                    <TableHead>{t('time_entries.activity_type', 'Activity')}</TableHead>
                    <TableHead className="text-right">{t('common.hours', 'Hours')}</TableHead>
                    <TableHead className="text-right">{t('project_budget.rate', 'Rate')}</TableHead>
                    <TableHead className="text-right">{t('project_budget.amount', 'Amount')}</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {invoicePreview.lines.map((line) => (
                    <TableRow key={line.id}>
                      <TableCell className="font-mono text-xs">{line.date}</TableCell>
                      <TableCell className="max-w-xs truncate text-xs">{line.description}</TableCell>
                      <TableCell className="text-xs">{line.activity}</TableCell>
                      <TableCell className="text-right font-mono text-xs">{line.hours.toFixed(1)}h</TableCell>
                      <TableCell className="text-right font-mono text-xs">
                        {line.rate != null ? line.rate.toFixed(2) : <span className="text-destructive">—</span>}
                      </TableCell>
                      <TableCell className="text-right font-mono text-xs">{line.amount.toFixed(2)}</TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </div>
            <div className="flex justify-between text-sm font-semibold border-t pt-2">
              <span>{t('common.total', 'Total')}: {invoicePreview.totalHrs.toFixed(1)}h</span>
              <span>CHF {invoicePreview.totalAmount.toFixed(2)}</span>
            </div>
            {invoicePreview.lines.some((l) => l.rate == null) && (
              <p className="text-xs text-destructive">
                {t('projects.missing_rates_warning', 'Some entries have no rate. Set activity type rates or project hourly rate.')}
              </p>
            )}
          </div>
          <DialogFooter>
            <Button variant="outline" onClick={() => setInvoicePreviewOpen(false)}>
              {t('common.cancel', 'Cancel')}
            </Button>
            <Button onClick={handleCreateInvoice} disabled={createInvoice.isPending}>
              <ReceiptText className="mr-1 h-4 w-4" />
              {t('invoices.create_invoice', 'Create Invoice')}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      <div className="flex items-center justify-between">
        <h3 className="text-sm font-semibold">
          {t('projects.tab_time_entries', 'Time Entries')}
          {entries.length > 0 && (
            <span className="ml-2 text-muted-foreground font-normal">
              ({totalHours.toFixed(1)}h — {unbilled.length} {t('projects.unbilled', 'unbilled')})
            </span>
          )}
        </h3>
        <div className="flex items-center gap-2">
          {entries.length > 0 && (
            <Select value={groupBy} onValueChange={(v) => setGroupBy(v as GroupBy)}>
              <SelectTrigger className="w-[140px] h-8 text-xs">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="none">{t('common.no_grouping', 'No grouping')}</SelectItem>
                <SelectItem value="user">{t('common.group_by_user', 'Group by user')}</SelectItem>
                <SelectItem value="task">{t('common.group_by_task', 'Group by task')}</SelectItem>
              </SelectContent>
            </Select>
          )}
          {selected.size > 0 && (
            <>
              <span className="text-xs text-muted-foreground">
                {t('projects.entries_selected', '{count} entries selected').replace('{count}', String(selected.size))}
              </span>
              <Button size="sm" onClick={() => setInvoicePreviewOpen(true)}>
                <ReceiptText className="mr-1 h-4 w-4" />
                {t('projects.create_invoice_from_selected', 'Create Invoice from Selected')}
              </Button>
            </>
          )}
          <Dialog open={addOpen} onOpenChange={setAddOpen}>
            <DialogTrigger asChild>
              <Button size="sm" variant="outline">
                <Plus className="mr-1 h-4 w-4" /> {t('time_entries.new_entry', 'New Entry')}
              </Button>
            </DialogTrigger>
            <DialogContent>
              <DialogHeader>
                <DialogTitle>{t('time_entries.create_entry', 'Create Entry')}</DialogTitle>
              </DialogHeader>
              <div className="space-y-4">
                {isAdmin && users.length > 0 && (
                  <div>
                    <Label>{t('common.user', 'User')}</Label>
                    <Select value={form.user_id || currentUser?.id || ''} onValueChange={(v) => setForm({ ...form, user_id: v })}>
                      <SelectTrigger><SelectValue /></SelectTrigger>
                      <SelectContent>
                        {users.filter((u) => u.is_active).map((u) => (
                          <SelectItem key={u.id} value={u.id}>{u.full_name}</SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                  </div>
                )}
                {entryActivityTypes.length > 0 && (
                  <div>
                    <Label>{t('time_entries.activity_type', 'Activity Type')}</Label>
                    <Select value={form.activity_type_id || '__none__'} onValueChange={(v) => setForm({ ...form, activity_type_id: v === '__none__' ? '' : v, quantity: '' })}>
                      <SelectTrigger><SelectValue /></SelectTrigger>
                      <SelectContent>
                        <SelectItem value="__none__">{t('common.none', 'None')}</SelectItem>
                        {entryActivityTypes.map((at: ActivityType) => (
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
                    <Input type="number" step="0.01" value={form.quantity} onChange={(e) => setForm({ ...form, quantity: e.target.value })} placeholder="0.00" />
                  </div>
                )}
                {tasks.length > 0 && (
                  <div>
                    <Label>{t('time_entries.task', 'Task')} ({t('common.optional', 'optional')})</Label>
                    <select className="w-full rounded-md border border-input bg-background px-3 py-2 text-sm" value={form.task_id} onChange={(e) => setForm({ ...form, task_id: e.target.value })}>
                      <option value="">{t('common.none', 'None')}</option>
                      {tasks.map((task) => (<option key={task.id} value={task.id}>{task.name}</option>))}
                    </select>
                  </div>
                )}
                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <Label>{t('common.date', 'Date')}</Label>
                    <Input type="date" value={form.date} onChange={(e) => setForm({ ...form, date: e.target.value })} />
                  </div>
                  <div>
                    <Label>{t('common.hours', 'Hours')}</Label>
                    <Input type="number" step="0.1" value={form.hours} onChange={(e) => setForm({ ...form, hours: e.target.value })} placeholder="1.5" />
                  </div>
                </div>
                <div>
                  <Label>{t('common.description', 'Description')}</Label>
                  <Input value={form.description} onChange={(e) => setForm({ ...form, description: e.target.value })} placeholder={t('common.description', 'Description')} />
                </div>
                <Button onClick={handleAddEntry} className="w-full" disabled={createEntry.isPending}>
                  {t('time_entries.create_entry', 'Create Entry')}
                </Button>
              </div>
            </DialogContent>
          </Dialog>
        </div>
      </div>

      <Card>
        <CardContent className="p-0">
          {entries.length > 0 ? (
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead className="w-10">
                    <Checkbox checked={unbilled.length > 0 && selected.size === unbilled.length} onCheckedChange={toggleAll} />
                  </TableHead>
                  <TableHead>{t('common.date', 'Date')}</TableHead>
                  <TableHead>{t('common.hours', 'Hours')}</TableHead>
                  <TableHead className="hidden lg:table-cell">{t('common.user', 'User')}</TableHead>
                  <TableHead className="hidden md:table-cell">{t('common.description', 'Description')}</TableHead>
                  <TableHead className="hidden sm:table-cell">{t('time_entries.activity_type', 'Activity Type')}</TableHead>
                  <TableHead className="hidden sm:table-cell text-right">{t('project_budget.rate', 'Rate')}</TableHead>
                  <TableHead className="hidden sm:table-cell text-right">{t('project_budget.amount', 'Amount')}</TableHead>
                  <TableHead>{t('common.status', 'Status')}</TableHead>
                  <TableHead></TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {grouped ? (
                  Array.from(grouped.entries()).map(([key, items]) => (
                    <>
                      <TableRow key={`group-${key}`} className="bg-muted/50">
                        <TableCell colSpan={10} className="py-1.5">
                          <span className="text-xs font-semibold">{getGroupLabel(key)}</span>
                          <span className="ml-2 text-xs text-muted-foreground">{getGroupHours(items).toFixed(1)}h</span>
                        </TableCell>
                      </TableRow>
                      {items.map(renderEntryRow)}
                    </>
                  ))
                ) : (
                  entries.map(renderEntryRow)
                )}
              </TableBody>
            </Table>
          ) : (
            <p className="py-6 text-center text-sm text-muted-foreground">
              {t('projects.no_time_entries', 'No time entries for this project.')}
            </p>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
