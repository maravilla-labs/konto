import { useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import {
  useTimesheets,
  useCreateTimesheet,
  useDeleteTimesheet,
  useSubmitTimesheet,
} from '@/hooks/useApi';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Skeleton } from '@/components/ui/skeleton';
import { Badge } from '@/components/ui/badge';
import { RichTextEditor } from '@/components/ui/rich-text-editor';
import { Pagination } from '@/components/ui/pagination';
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
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import { Plus, Eye, Trash2, Send } from 'lucide-react';
import { toast } from 'sonner';
import { useI18n } from '@/i18n';
import { StickyToolbar, type ToolbarAction } from '@/components/ui/sticky-toolbar';
import { useSettings } from '@/hooks/useSettingsApi';
import { formatDate } from '@/lib/locale';

const statusVariant: Record<string, 'default' | 'secondary' | 'destructive' | 'outline'> = {
  draft: 'secondary',
  submitted: 'default',
  approved: 'outline',
  locked: 'destructive',
};

export function TimesheetsPage() {
  const { t } = useI18n();
  const { data: settings } = useSettings();
  const navigate = useNavigate();
  const [page, setPage] = useState(1);
  const [statusFilter, setStatusFilter] = useState<string | undefined>();
  const [createOpen, setCreateOpen] = useState(false);
  const [createForm, setCreateForm] = useState({
    period_start: '',
    period_end: '',
    notes: '',
  });
  const dateFormat = settings?.date_format ?? 'dd.MM.yyyy';

  const statusTabs: { label: string; value: string | undefined }[] = [
    { label: t('common.all', 'All'), value: undefined },
    { label: t('timesheets.status.draft', 'Draft'), value: 'draft' },
    { label: t('timesheets.status.submitted', 'Submitted'), value: 'submitted' },
    { label: t('timesheets.status.approved', 'Approved'), value: 'approved' },
    { label: t('timesheets.status.locked', 'Locked'), value: 'locked' },
  ];

  const { data, isLoading } = useTimesheets({
    page,
    status: statusFilter,
  });
  const createTimesheet = useCreateTimesheet();
  const deleteTimesheet = useDeleteTimesheet();
  const submitTimesheet = useSubmitTimesheet();

  const timesheets = data?.data ?? [];

  function handleCreate() {
    if (!createForm.period_start || !createForm.period_end) {
      toast.error(t('timesheets.period_required', 'Period start and end dates are required'));
      return;
    }
    createTimesheet.mutate(
      {
        period_start: createForm.period_start,
        period_end: createForm.period_end,
        notes: createForm.notes || undefined,
      },
      {
        onSuccess: (res) => {
          toast.success(t('timesheets.created', 'Timesheet created'));
          setCreateOpen(false);
          setCreateForm({ period_start: '', period_end: '', notes: '' });
          navigate(`/timesheets/${res.data.id}`);
        },
        onError: () => toast.error(t('timesheets.create_failed', 'Failed to create timesheet')),
      }
    );
  }

  function handleDelete(id: string) {
    deleteTimesheet.mutate(id, {
      onSuccess: () => toast.success(t('timesheets.deleted', 'Timesheet deleted')),
      onError: () => toast.error(t('timesheets.delete_failed', 'Failed to delete timesheet')),
    });
  }

  function handleSubmit(id: string) {
    submitTimesheet.mutate(id, {
      onSuccess: () => toast.success(t('timesheets.submitted', 'Timesheet submitted for approval')),
      onError: () => toast.error(t('timesheets.submit_failed', 'Failed to submit timesheet')),
    });
  }

  return (
    <div className="space-y-4">
      <StickyToolbar
        actions={[
          { icon: <Plus className="h-4 w-4" />, label: t('timesheets.new_timesheet', 'New Timesheet'), onClick: () => setCreateOpen(true), primary: true },
        ] satisfies ToolbarAction[]}
      >
        <Badge variant="secondary">{timesheets.length} {t('timesheets.title', 'Timesheets')}</Badge>
      </StickyToolbar>

      <Dialog open={createOpen} onOpenChange={setCreateOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>{t('timesheets.create_timesheet', 'Create Timesheet')}</DialogTitle>
          </DialogHeader>
          <div className="space-y-4">
            <div className="grid grid-cols-2 gap-4">
              <div>
                <Label>{t('timesheets.period_start', 'Period Start')}</Label>
                <Input
                  type="date"
                  value={createForm.period_start}
                  onChange={(e) => setCreateForm({ ...createForm, period_start: e.target.value })}
                />
              </div>
              <div>
                <Label>{t('timesheets.period_end', 'Period End')}</Label>
                <Input
                  type="date"
                  value={createForm.period_end}
                  onChange={(e) => setCreateForm({ ...createForm, period_end: e.target.value })}
                />
              </div>
            </div>
            <div>
              <Label>{t('common.notes', 'Notes')} ({t('common.optional', 'optional')})</Label>
              <RichTextEditor
                value={createForm.notes}
                onChange={(md) => setCreateForm({ ...createForm, notes: md })}
                placeholder={t('timesheets.notes_placeholder', 'Optional notes for this timesheet...')}
              />
            </div>
            <Button onClick={handleCreate} className="w-full" disabled={createTimesheet.isPending}>
              {t('timesheets.create_timesheet', 'Create Timesheet')}
            </Button>
          </div>
        </DialogContent>
      </Dialog>

      <div className="flex flex-wrap gap-1">
        {statusTabs.map((tab) => (
          <Button
            key={tab.label}
            variant={statusFilter === tab.value ? 'default' : 'outline'}
            size="sm"
            onClick={() => { setStatusFilter(tab.value); setPage(1); }}
          >
            {tab.label}
          </Button>
        ))}
      </div>

      <Card>
        <CardContent className="p-0">
          {isLoading ? (
            <div className="space-y-2 p-4">
              {Array.from({ length: 5 }).map((_, i) => (
                <Skeleton key={i} className="h-10 w-full" />
              ))}
            </div>
          ) : timesheets.length > 0 ? (
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>{t('timesheets.period', 'Period')}</TableHead>
                  <TableHead className="hidden sm:table-cell">{t('common.status', 'Status')}</TableHead>
                  <TableHead className="hidden md:table-cell">{t('timesheets.total_hours', 'Total Hours')}</TableHead>
                  <TableHead className="hidden md:table-cell">{t('timesheets.entries', 'Entries')}</TableHead>
                  <TableHead className="hidden lg:table-cell">{t('timesheets.submitted_at', 'Submitted')}</TableHead>
                  <TableHead className="w-32" />
                </TableRow>
              </TableHeader>
              <TableBody>
                {timesheets.map((ts) => (
                  <TableRow key={ts.id}>
                    <TableCell>
                      <Link
                        to={`/timesheets/${ts.id}`}
                        className="font-mono text-sm font-medium text-primary hover:underline"
                      >
                        {formatDate(ts.period_start, dateFormat)} - {formatDate(ts.period_end, dateFormat)}
                      </Link>
                    </TableCell>
                    <TableCell className="hidden sm:table-cell">
                      <Badge variant={statusVariant[ts.status] ?? 'outline'}>
                        {t(`timesheets.status.${ts.status}`, ts.status)}
                      </Badge>
                    </TableCell>
                    <TableCell className="hidden md:table-cell font-mono text-sm">
                      {ts.total_hours != null ? `${ts.total_hours.toFixed(1)}h` : '-'}
                    </TableCell>
                    <TableCell className="hidden md:table-cell text-sm">
                      {ts.entry_count ?? 0}
                    </TableCell>
                    <TableCell className="hidden lg:table-cell font-mono text-sm">
                      {ts.submitted_at ? formatDate(ts.submitted_at.split('T')[0], dateFormat) : '-'}
                    </TableCell>
                    <TableCell>
                      <div className="flex gap-1">
                        <Button
                          variant="ghost"
                          size="icon"
                          onClick={() => navigate(`/timesheets/${ts.id}`)}
                          title={t('common.view', 'View')}
                        >
                          <Eye className="h-3.5 w-3.5" />
                        </Button>
                        {ts.status === 'draft' && (
                          <>
                            <Button
                              variant="ghost"
                              size="icon"
                              onClick={() => handleSubmit(ts.id)}
                              title={t('timesheets.submit', 'Submit')}
                            >
                              <Send className="h-3.5 w-3.5" />
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
                                    {t('timesheets.delete_confirm', 'This will permanently delete this timesheet.')}
                                  </AlertDialogDescription>
                                </AlertDialogHeader>
                                <AlertDialogFooter>
                                  <AlertDialogCancel>{t('common.cancel', 'Cancel')}</AlertDialogCancel>
                                  <AlertDialogAction onClick={() => handleDelete(ts.id)}>
                                    {t('common.delete', 'Delete')}
                                  </AlertDialogAction>
                                </AlertDialogFooter>
                              </AlertDialogContent>
                            </AlertDialog>
                          </>
                        )}
                      </div>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          ) : (
            <p className="py-8 text-center text-sm text-muted-foreground">
              {t('timesheets.no_results', 'No timesheets found. Create your first timesheet.')}
            </p>
          )}
        </CardContent>
      </Card>

      {data && (
        <Pagination
          page={page}
          totalPages={data.total_pages}
          onPageChange={setPage}
        />
      )}
    </div>
  );
}
