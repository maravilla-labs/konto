import { useState } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import {
  useTimesheet,
  useUpdateTimesheet,
  useDeleteTimesheet,
  useSubmitTimesheet,
  useApproveTimesheet,
  useRejectTimesheet,
  useTimeEntries,
} from '@/hooks/useApi';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Skeleton } from '@/components/ui/skeleton';
import { RichTextEditor } from '@/components/ui/rich-text-editor';
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
import { Send, CheckCircle, XCircle, Trash2, ArrowLeft, Calendar, Clock, FileText } from 'lucide-react';
import { toast } from 'sonner';
import { useI18n } from '@/i18n';
import { useSettings } from '@/hooks/useSettingsApi';
import { formatDate } from '@/lib/locale';
import { useAuthStore } from '@/stores/authStore';

const statusVariant: Record<string, 'default' | 'secondary' | 'destructive' | 'outline'> = {
  draft: 'secondary',
  submitted: 'default',
  approved: 'outline',
  locked: 'destructive',
};

export function TimesheetDetailPage() {
  const { id } = useParams<{ id: string }>();
  const { t } = useI18n();
  const { data: settings } = useSettings();
  const navigate = useNavigate();
  const user = useAuthStore((s) => s.user);
  const dateFormat = settings?.date_format ?? 'dd.MM.yyyy';

  const { data: timesheet, isLoading } = useTimesheet(id);
  const { data: entriesData } = useTimeEntries({ per_page: 200 });
  const updateTimesheet = useUpdateTimesheet();
  const deleteTimesheet = useDeleteTimesheet();
  const submitTimesheet = useSubmitTimesheet();
  const approveTimesheet = useApproveTimesheet();
  const rejectTimesheet = useRejectTimesheet();

  const [editingNotes, setEditingNotes] = useState(false);
  const [notesValue, setNotesValue] = useState('');

  // Filter time entries for this timesheet's period and user
  const allEntries = entriesData?.data ?? [];
  const timesheetEntries = timesheet
    ? allEntries.filter((e) => e.timesheet_id === timesheet.id)
    : [];

  const totalHours = timesheetEntries.reduce((acc, e) => acc + e.actual_minutes / 60, 0);

  const isAdmin = user?.role === 'admin';
  const isDraft = timesheet?.status === 'draft';
  const isSubmitted = timesheet?.status === 'submitted';

  function handleSubmit() {
    if (!id) return;
    submitTimesheet.mutate(id, {
      onSuccess: () => toast.success(t('timesheets.submitted', 'Timesheet submitted for approval')),
      onError: () => toast.error(t('timesheets.submit_failed', 'Failed to submit timesheet')),
    });
  }

  function handleApprove() {
    if (!id) return;
    approveTimesheet.mutate(id, {
      onSuccess: () => toast.success(t('timesheets.approved', 'Timesheet approved')),
      onError: () => toast.error(t('timesheets.approve_failed', 'Failed to approve timesheet')),
    });
  }

  function handleReject() {
    if (!id) return;
    rejectTimesheet.mutate(id, {
      onSuccess: () => toast.success(t('timesheets.rejected', 'Timesheet rejected')),
      onError: () => toast.error(t('timesheets.reject_failed', 'Failed to reject timesheet')),
    });
  }

  function handleDelete() {
    if (!id) return;
    deleteTimesheet.mutate(id, {
      onSuccess: () => {
        toast.success(t('timesheets.deleted', 'Timesheet deleted'));
        navigate('/timesheets');
      },
      onError: () => toast.error(t('timesheets.delete_failed', 'Failed to delete timesheet')),
    });
  }

  function handleSaveNotes() {
    if (!id) return;
    updateTimesheet.mutate(
      { id, data: { notes: notesValue || undefined } },
      {
        onSuccess: () => {
          toast.success(t('timesheets.notes_updated', 'Notes updated'));
          setEditingNotes(false);
        },
        onError: () => toast.error(t('common.update_failed', 'Update failed')),
      }
    );
  }

  if (isLoading) {
    return (
      <div className="space-y-4">
        <Skeleton className="h-8 w-48" />
        <Skeleton className="h-32 w-full" />
        <Skeleton className="h-64 w-full" />
      </div>
    );
  }

  if (!timesheet) {
    return (
      <div className="py-8 text-center text-sm text-muted-foreground">
        {t('timesheets.not_found', 'Timesheet not found')}
      </div>
    );
  }

  return (
    <div className="space-y-4">
      {/* Header */}
      <div className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
        <div className="flex items-center gap-3">
          <Button variant="ghost" size="icon" onClick={() => navigate('/timesheets')}>
            <ArrowLeft className="h-4 w-4" />
          </Button>
          <div>
            <h2 className="text-lg font-semibold">
              {formatDate(timesheet.period_start, dateFormat)} - {formatDate(timesheet.period_end, dateFormat)}
            </h2>
            <p className="text-sm text-muted-foreground">
              {t('timesheets.timesheet', 'Timesheet')}
            </p>
          </div>
          <Badge variant={statusVariant[timesheet.status] ?? 'outline'}>
            {t(`timesheets.status.${timesheet.status}`, timesheet.status)}
          </Badge>
        </div>
        <div className="flex gap-2">
          {isDraft && (
            <>
              <Button size="sm" onClick={handleSubmit} disabled={submitTimesheet.isPending}>
                <Send className="mr-1 h-4 w-4" /> {t('timesheets.submit', 'Submit')}
              </Button>
              <AlertDialog>
                <AlertDialogTrigger asChild>
                  <Button size="sm" variant="destructive">
                    <Trash2 className="mr-1 h-4 w-4" /> {t('common.delete', 'Delete')}
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
                    <AlertDialogAction onClick={handleDelete}>
                      {t('common.delete', 'Delete')}
                    </AlertDialogAction>
                  </AlertDialogFooter>
                </AlertDialogContent>
              </AlertDialog>
            </>
          )}
          {isSubmitted && isAdmin && (
            <>
              <Button size="sm" onClick={handleApprove} disabled={approveTimesheet.isPending}>
                <CheckCircle className="mr-1 h-4 w-4" /> {t('timesheets.approve', 'Approve')}
              </Button>
              <Button size="sm" variant="outline" onClick={handleReject} disabled={rejectTimesheet.isPending}>
                <XCircle className="mr-1 h-4 w-4" /> {t('timesheets.reject', 'Reject')}
              </Button>
            </>
          )}
        </div>
      </div>

      {/* Summary Cards */}
      <div className="grid gap-4 sm:grid-cols-3">
        <Card>
          <CardHeader className="flex flex-row items-center gap-3 pb-2">
            <Clock className="h-4 w-4 text-muted-foreground" />
            <CardTitle className="text-sm font-medium">{t('timesheets.total_hours', 'Total Hours')}</CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-2xl font-bold font-mono">{totalHours.toFixed(1)}h</p>
          </CardContent>
        </Card>
        <Card>
          <CardHeader className="flex flex-row items-center gap-3 pb-2">
            <FileText className="h-4 w-4 text-muted-foreground" />
            <CardTitle className="text-sm font-medium">{t('timesheets.entries', 'Entries')}</CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-2xl font-bold font-mono">{timesheetEntries.length}</p>
          </CardContent>
        </Card>
        <Card>
          <CardHeader className="flex flex-row items-center gap-3 pb-2">
            <Calendar className="h-4 w-4 text-muted-foreground" />
            <CardTitle className="text-sm font-medium">{t('timesheets.period', 'Period')}</CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-sm font-mono">
              {formatDate(timesheet.period_start, dateFormat)} - {formatDate(timesheet.period_end, dateFormat)}
            </p>
          </CardContent>
        </Card>
      </div>

      {/* Notes */}
      <Card>
        <CardHeader className="pb-2">
          <div className="flex items-center justify-between">
            <CardTitle className="text-sm font-medium">{t('common.notes', 'Notes')}</CardTitle>
            {isDraft && !editingNotes && (
              <Button
                variant="ghost"
                size="sm"
                onClick={() => { setNotesValue(timesheet.notes ?? ''); setEditingNotes(true); }}
              >
                {t('common.update', 'Update')}
              </Button>
            )}
          </div>
        </CardHeader>
        <CardContent>
          {editingNotes ? (
            <div className="space-y-2">
              <RichTextEditor
                value={notesValue}
                onChange={(md) => setNotesValue(md)}
              />
              <div className="flex gap-2">
                <Button size="sm" onClick={handleSaveNotes} disabled={updateTimesheet.isPending}>
                  {t('common.save', 'Save')}
                </Button>
                <Button size="sm" variant="outline" onClick={() => setEditingNotes(false)}>
                  {t('common.cancel', 'Cancel')}
                </Button>
              </div>
            </div>
          ) : (
            <p className="text-sm text-muted-foreground">
              {timesheet.notes || t('timesheets.no_notes', 'No notes')}
            </p>
          )}
        </CardContent>
      </Card>

      {/* Time Entries Table */}
      <Card>
        <CardHeader className="pb-2">
          <CardTitle className="text-sm font-medium">{t('time_entries.title', 'Time Entries')}</CardTitle>
        </CardHeader>
        <CardContent className="p-0">
          {timesheetEntries.length > 0 ? (
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>{t('common.date', 'Date')}</TableHead>
                  <TableHead>{t('common.hours', 'Hours')}</TableHead>
                  <TableHead className="hidden md:table-cell">{t('common.description', 'Description')}</TableHead>
                  <TableHead className="hidden sm:table-cell">{t('common.status', 'Status')}</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {timesheetEntries.map((entry) => (
                  <TableRow key={entry.id}>
                    <TableCell className="font-mono text-sm">
                      {formatDate(entry.date, dateFormat)}
                    </TableCell>
                    <TableCell className="font-mono text-sm">
                      {(entry.actual_minutes / 60).toFixed(1)}h
                    </TableCell>
                    <TableCell className="hidden md:table-cell max-w-xs truncate text-sm">
                      {entry.description ?? '-'}
                    </TableCell>
                    <TableCell className="hidden sm:table-cell">
                      <Badge variant={entry.billed ? 'default' : 'outline'}>
                        {entry.billed
                          ? t('time_entries.filter.billed', 'Billed')
                          : t('time_entries.filter.unbilled', 'Unbilled')}
                      </Badge>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          ) : (
            <p className="py-8 text-center text-sm text-muted-foreground">
              {t('timesheets.no_entries', 'No time entries linked to this timesheet.')}
            </p>
          )}
        </CardContent>
      </Card>

      {/* Locked warning */}
      {timesheet.status === 'locked' && (
        <Card className="border-destructive">
          <CardContent className="py-3">
            <p className="text-sm text-destructive font-medium">
              {t('timesheets.locked_warning', 'This timesheet is locked and cannot be modified.')}
            </p>
          </CardContent>
        </Card>
      )}
    </div>
  );
}
