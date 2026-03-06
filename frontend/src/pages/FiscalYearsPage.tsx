import { useState } from 'react';
import { useFiscalYears, useCreateFiscalYear, useUpdateFiscalYear, useCloseFiscalYear } from '@/hooks/useApi';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Skeleton } from '@/components/ui/skeleton';
import { Badge } from '@/components/ui/badge';
import {
  Dialog,
  DialogContent,
  DialogDescription,
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
import { Plus, Lock, Pencil } from 'lucide-react';
import { toast } from 'sonner';
import { useI18n } from '@/i18n';
import { StickyToolbar, type ToolbarAction } from '@/components/ui/sticky-toolbar';
import type { FiscalYear } from '@/types/fiscal-year';

export function FiscalYearsPage() {
  const { t } = useI18n();
  const { data, isLoading } = useFiscalYears();
  const createFiscalYear = useCreateFiscalYear();
  const updateFiscalYear = useUpdateFiscalYear();
  const closeFiscalYear = useCloseFiscalYear();
  const [createOpen, setCreateOpen] = useState(false);
  const [editOpen, setEditOpen] = useState(false);
  const [form, setForm] = useState({ name: '', start_date: '', end_date: '' });
  const [editForm, setEditForm] = useState({ id: '', name: '', start_date: '', end_date: '' });

  const years = data?.data ?? [];

  function handleCreate() {
    createFiscalYear.mutate(form, {
      onSuccess: () => {
        toast.success(t('fiscal_years.created', 'Fiscal year created'));
        setCreateOpen(false);
        setForm({ name: '', start_date: '', end_date: '' });
      },
      onError: () => toast.error(t('fiscal_years.create_failed', 'Failed to create fiscal year')),
    });
  }

  function openEdit(fy: FiscalYear) {
    setEditForm({ id: fy.id, name: fy.name, start_date: fy.start_date, end_date: fy.end_date });
    setEditOpen(true);
  }

  function handleUpdate() {
    updateFiscalYear.mutate(
      { id: editForm.id, data: { name: editForm.name, start_date: editForm.start_date, end_date: editForm.end_date } },
      {
        onSuccess: () => {
          toast.success(t('fiscal_years.updated', 'Fiscal year updated'));
          setEditOpen(false);
        },
        onError: () => toast.error(t('fiscal_years.update_failed', 'Failed to update fiscal year')),
      },
    );
  }

  function handleClose(id: string) {
    closeFiscalYear.mutate(id, {
      onSuccess: () => toast.success(t('fiscal_years.closed', 'Fiscal year closed')),
      onError: () => toast.error(t('fiscal_years.close_failed', 'Failed to close fiscal year')),
    });
  }

  return (
    <div className="space-y-4">
      <StickyToolbar
        actions={[
          { icon: <Plus className="h-4 w-4" />, label: t('fiscal_years.new', 'New Fiscal Year'), onClick: () => setCreateOpen(true), primary: true },
        ] satisfies ToolbarAction[]}
      >
        <Badge variant="secondary">{years.length} {t('fiscal_years.title', 'Fiscal Years')}</Badge>
      </StickyToolbar>

      <Dialog open={createOpen} onOpenChange={setCreateOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>{t('fiscal_years.create', 'Create Fiscal Year')}</DialogTitle>
            <DialogDescription>{t('fiscal_years.create_desc', 'Define a new accounting period.')}</DialogDescription>
          </DialogHeader>
          <div className="space-y-4">
            <div>
              <Label>{t('common.name', 'Name')}</Label>
              <Input
                value={form.name}
                onChange={(e) => setForm({ ...form, name: e.target.value })}
                placeholder="FY 2026"
              />
            </div>
            <div className="grid grid-cols-2 gap-4">
              <div>
                <Label>{t('fiscal_years.start_date', 'Start Date')}</Label>
                <Input
                  type="date"
                  value={form.start_date}
                  onChange={(e) => setForm({ ...form, start_date: e.target.value })}
                />
              </div>
              <div>
                <Label>{t('fiscal_years.end_date', 'End Date')}</Label>
                <Input
                  type="date"
                  value={form.end_date}
                  onChange={(e) => setForm({ ...form, end_date: e.target.value })}
                />
              </div>
            </div>
            <Button onClick={handleCreate} className="w-full" disabled={createFiscalYear.isPending}>
              {t('fiscal_years.create', 'Create Fiscal Year')}
            </Button>
          </div>
        </DialogContent>
      </Dialog>

      {/* Edit Dialog */}
      <Dialog open={editOpen} onOpenChange={setEditOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>{t('fiscal_years.edit', 'Edit Fiscal Year')}</DialogTitle>
            <DialogDescription>{t('fiscal_years.edit_desc', 'Update fiscal year details.')}</DialogDescription>
          </DialogHeader>
          <div className="space-y-4">
            <div>
              <Label>{t('common.name', 'Name')}</Label>
              <Input
                value={editForm.name}
                onChange={(e) => setEditForm({ ...editForm, name: e.target.value })}
              />
            </div>
            <div className="grid grid-cols-2 gap-4">
              <div>
                <Label>{t('fiscal_years.start_date', 'Start Date')}</Label>
                <Input
                  type="date"
                  value={editForm.start_date}
                  onChange={(e) => setEditForm({ ...editForm, start_date: e.target.value })}
                />
              </div>
              <div>
                <Label>{t('fiscal_years.end_date', 'End Date')}</Label>
                <Input
                  type="date"
                  value={editForm.end_date}
                  onChange={(e) => setEditForm({ ...editForm, end_date: e.target.value })}
                />
              </div>
            </div>
            <Button onClick={handleUpdate} className="w-full" disabled={updateFiscalYear.isPending}>
              {t('common.save', 'Save Changes')}
            </Button>
          </div>
        </DialogContent>
      </Dialog>

      <Card>
        <CardContent className="p-0">
          {isLoading ? (
            <div className="space-y-2 p-4">
              {Array.from({ length: 3 }).map((_, i) => (
                <Skeleton key={i} className="h-10 w-full" />
              ))}
            </div>
          ) : years.length > 0 ? (
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>{t('common.name', 'Name')}</TableHead>
                  <TableHead>{t('fiscal_years.start_date', 'Start')}</TableHead>
                  <TableHead>{t('fiscal_years.end_date', 'End')}</TableHead>
                  <TableHead>{t('common.status', 'Status')}</TableHead>
                  <TableHead className="w-24" />
                </TableRow>
              </TableHeader>
              <TableBody>
                {years.map((fy) => (
                  <TableRow key={fy.id}>
                    <TableCell className="font-medium">{fy.name}</TableCell>
                    <TableCell className="font-mono text-sm">{fy.start_date}</TableCell>
                    <TableCell className="font-mono text-sm">{fy.end_date}</TableCell>
                    <TableCell>
                      <Badge
                        variant={fy.status === 'open' ? 'default' : 'secondary'}
                        className={
                          fy.status === 'open'
                            ? 'bg-green-100 text-green-800'
                            : 'bg-red-100 text-red-800'
                        }
                      >
                        {fy.status === 'open' ? t('fiscal_years.status.open', 'Open') : t('fiscal_years.status.closed', 'Closed')}
                      </Badge>
                    </TableCell>
                    <TableCell>
                      {fy.status === 'open' && (
                        <div className="flex gap-1">
                          <Button
                            variant="ghost"
                            size="icon"
                            title={t('fiscal_years.edit', 'Edit fiscal year')}
                            onClick={() => openEdit(fy)}
                          >
                            <Pencil className="h-4 w-4" />
                          </Button>
                          <AlertDialog>
                            <AlertDialogTrigger asChild>
                              <Button variant="ghost" size="icon" title={t('fiscal_years.close', 'Close fiscal year')}>
                                <Lock className="h-4 w-4" />
                              </Button>
                            </AlertDialogTrigger>
                            <AlertDialogContent>
                              <AlertDialogHeader>
                                <AlertDialogTitle>{t('fiscal_years.close_confirm_title', 'Close Fiscal Year?')}</AlertDialogTitle>
                                <AlertDialogDescription>
                                  {t('fiscal_years.close_confirm_desc', 'This will close "{name}". No more entries can be posted to this period. This action cannot be undone.').replace('{name}', fy.name)}
                                </AlertDialogDescription>
                              </AlertDialogHeader>
                              <AlertDialogFooter>
                                <AlertDialogCancel>{t('common.cancel', 'Cancel')}</AlertDialogCancel>
                                <AlertDialogAction onClick={() => handleClose(fy.id)}>
                                  {t('fiscal_years.close_year', 'Close Year')}
                                </AlertDialogAction>
                              </AlertDialogFooter>
                            </AlertDialogContent>
                          </AlertDialog>
                        </div>
                      )}
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          ) : (
            <p className="py-8 text-center text-sm text-muted-foreground">
              {t('fiscal_years.no_results', 'No fiscal years defined. Create your first fiscal year.')}
            </p>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
