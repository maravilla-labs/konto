import { useState } from 'react';
import {
  useRecurringInvoices,
  useCreateRecurringInvoice,
  useUpdateRecurringInvoice,
  useDeleteRecurringInvoice,
  useTriggerRecurringInvoices,
  useContacts,
  useProjects,
  useAccounts,
} from '@/hooks/useApi';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Badge } from '@/components/ui/badge';
import { Pagination } from '@/components/ui/pagination';
import { Skeleton } from '@/components/ui/skeleton';
import {
  Table, TableBody, TableCell, TableHead, TableHeader, TableRow,
} from '@/components/ui/table';
import { Plus, Search, Download, Play, Pencil, Trash2 } from 'lucide-react';
import { downloadCsv } from '@/lib/export';
import { toast } from 'sonner';
import { StickyToolbar, type ToolbarAction } from '@/components/ui/sticky-toolbar';
import type { RecurringInvoice, CreateRecurringInvoice, UpdateRecurringInvoice } from '@/types/recurring-invoice';
import type { Contact } from '@/types/contacts';
import { RecurringInvoiceDialog } from '@/components/recurring-invoice/RecurringInvoiceDialog';
import { useI18n } from '@/i18n';
import { useSettings } from '@/hooks/useSettingsApi';
import { formatDate } from '@/lib/locale';

export function RecurringInvoicesPage() {
  const { t } = useI18n();
  const { data: settings } = useSettings();
  const [search, setSearch] = useState('');
  const [activeFilter, setActiveFilter] = useState<boolean | undefined>(true);
  const [page, setPage] = useState(1);
  const [dialogOpen, setDialogOpen] = useState(false);
  const [editing, setEditing] = useState<RecurringInvoice | null>(null);
  const dateFormat = settings?.date_format ?? 'dd.MM.yyyy';
  const filterTabs: { label: string; value: boolean | undefined }[] = [
    { label: t('invoices.filter.all', 'All'), value: undefined },
    { label: t('recurring.active', 'Active'), value: true },
    { label: t('recurring.paused', 'Paused'), value: false },
  ];
  const frequencyLabel: Record<string, string> = {
    monthly: t('recurring.frequency.monthly', 'Monthly'),
    quarterly: t('recurring.frequency.quarterly', 'Quarterly'),
    semi_annual: t('recurring.frequency.semi_annual', 'Semi-Annual'),
    annual: t('recurring.frequency.annual', 'Annual'),
    custom: t('recurring.frequency.custom', 'Custom'),
  };

  const { data, isLoading } = useRecurringInvoices({
    search: search || undefined,
    is_active: activeFilter,
    page,
  });
  const contacts = useContacts({ per_page: 200 });
  const projects = useProjects({ per_page: 200 });
  const accounts = useAccounts({ per_page: 500 });
  const createMut = useCreateRecurringInvoice();
  const updateMut = useUpdateRecurringInvoice();
  const deleteMut = useDeleteRecurringInvoice();
  const triggerMut = useTriggerRecurringInvoices();

  const items = data?.data ?? [];
  const contactList: Contact[] = contacts.data?.data ?? [];
  const contactMap = new Map(
    contactList.map((c) => [c.id, c.name2 ? `${c.name1} (${c.name2})` : c.name1]),
  );

  function handleCreate() {
    setEditing(null);
    setDialogOpen(true);
  }

  function handleEdit(item: RecurringInvoice) {
    setEditing(item);
    setDialogOpen(true);
  }

  function handleDelete(item: RecurringInvoice) {
    if (item.is_active) {
      toast.error(t('recurring.deactivate_before_delete', 'Deactivate before deleting'));
      return;
    }
    deleteMut.mutate(item.id, {
      onSuccess: () => toast.success(t('recurring.deleted', 'Recurring invoice deleted')),
      onError: () => toast.error(t('common.delete_failed', 'Failed to delete')),
    });
  }

  function handleSave(formData: CreateRecurringInvoice | UpdateRecurringInvoice) {
    if (editing) {
      updateMut.mutate(
        { id: editing.id, data: formData as UpdateRecurringInvoice },
        {
          onSuccess: () => { toast.success(t('recurring.updated', 'Updated')); setDialogOpen(false); },
          onError: () => toast.error(t('common.update_failed', 'Update failed')),
        },
      );
    } else {
      createMut.mutate(formData as CreateRecurringInvoice, {
        onSuccess: () => { toast.success(t('common.created', 'Created')); setDialogOpen(false); },
        onError: () => toast.error(t('common.create_failed', 'Create failed')),
      });
    }
  }

  function handleTrigger() {
    triggerMut.mutate(undefined, {
      onSuccess: (res) => {
        const count = res.data.generated;
        toast.success(`${count} ${t('recurring.generated_invoices', 'invoice(s) generated')}`);
      },
      onError: () => toast.error(t('recurring.generation_failed', 'Generation failed')),

    });
  }

  return (
    <div className="space-y-4">
      <StickyToolbar
        actions={[
          { icon: <Download className="h-4 w-4" />, label: t('invoices.export_csv', 'Export CSV'), onClick: () => downloadCsv('/recurring-invoices') },
          { icon: <Play className="h-4 w-4" />, label: t('recurring.run_now', 'Run Now'), onClick: handleTrigger, disabled: triggerMut.isPending, loading: triggerMut.isPending },
          { icon: <Plus className="h-4 w-4" />, label: t('common.create', 'Create'), onClick: handleCreate, primary: true },
        ] satisfies ToolbarAction[]}
      >
        <Badge variant="secondary">{items.length} {t('recurring.title', 'Recurring Invoices')}</Badge>
      </StickyToolbar>

      <div className="flex flex-wrap gap-1">
        {filterTabs.map((tab) => (
          <Button
            key={tab.label}
            variant={activeFilter === tab.value ? 'default' : 'outline'}
            size="sm"
            onClick={() => { setActiveFilter(tab.value); setPage(1); }}
          >
            {tab.label}
          </Button>
        ))}
      </div>

      <div className="relative">
        <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
        <Input
          placeholder={t('recurring.search_placeholder', 'Search recurring invoices...')}
          value={search}
          onChange={(e) => { setSearch(e.target.value); setPage(1); }}
          className="pl-9"
        />
      </div>

      <Card>
        <CardContent className="p-0">
          {isLoading ? (
            <div className="space-y-2 p-4">
              {Array.from({ length: 5 }).map((_, i) => (
                <Skeleton key={i} className="h-10 w-full" />
              ))}
            </div>
          ) : items.length > 0 ? (
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>{t('common.contact', 'Contact')}</TableHead>
                  <TableHead>{t('recurring.frequency', 'Frequency')}</TableHead>
                  <TableHead className="hidden sm:table-cell">{t('recurring.next_run', 'Next Run')}</TableHead>
                  <TableHead className="hidden md:table-cell">{t('recurring.auto_send', 'Auto Send')}</TableHead>
                  <TableHead>{t('common.status', 'Status')}</TableHead>
                  <TableHead className="text-right">{t('common.actions', 'Actions')}</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {items.map((item) => (
                  <TableRow key={item.id}>
                    <TableCell className="font-medium">
                      {contactMap.get(item.contact_id) ?? item.contact_id}
                    </TableCell>
                    <TableCell>
                      {frequencyLabel[item.frequency] ?? item.frequency}
                      {item.frequency === 'custom' && item.interval_days
                        ? ` (${item.interval_days}${t('recurring.days_suffix', 'd')})`
                        : ''}
                    </TableCell>
                    <TableCell className="hidden sm:table-cell font-mono text-sm">
                      {formatDate(item.next_run_date, dateFormat)}
                    </TableCell>
                    <TableCell className="hidden md:table-cell">
                      {item.auto_send ? (
                        <Badge variant="default">{t('recurring.auto', 'Auto')}</Badge>
                      ) : (
                        <Badge variant="outline">{t('recurring.manual', 'Manual')}</Badge>
                      )}
                    </TableCell>
                    <TableCell>
                      <Badge variant={item.is_active ? 'default' : 'secondary'}>
                        {item.is_active ? t('recurring.active', 'Active') : t('recurring.paused', 'Paused')}
                      </Badge>
                    </TableCell>
                    <TableCell className="text-right">
                      <div className="flex justify-end gap-1">
                        <Button variant="ghost" size="icon" onClick={() => handleEdit(item)}>
                          <Pencil className="h-4 w-4" />
                        </Button>
                        <Button
                          variant="ghost"
                          size="icon"
                          onClick={() => handleDelete(item)}
                          disabled={item.is_active}
                        >
                          <Trash2 className="h-4 w-4" />
                        </Button>
                      </div>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          ) : (
            <p className="py-8 text-center text-sm text-muted-foreground">
              {t('recurring.no_results', 'No recurring invoices found. Create your first one.')}
            </p>
          )}
        </CardContent>
      </Card>

      {data && <Pagination page={page} totalPages={data.total_pages} onPageChange={setPage} />}

      <RecurringInvoiceDialog
        open={dialogOpen}
        onOpenChange={setDialogOpen}
        editing={editing}
        onSave={handleSave}
        isPending={createMut.isPending || updateMut.isPending}
        contacts={contactList.map((c) => ({
          id: c.id,
          name: c.name1,
          company: c.name2,
        }))}
        projects={projects.data?.data ?? []}
        accounts={accounts.data?.data ?? []}
      />
    </div>
  );
}
