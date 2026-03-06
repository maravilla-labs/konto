import { useState, useMemo } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { useInvoices, useContacts } from '@/hooks/useApi';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Skeleton } from '@/components/ui/skeleton';
import { Badge } from '@/components/ui/badge';
import { Pagination } from '@/components/ui/pagination';
import { StickyToolbar, type ToolbarAction, type ToolbarOverflowItem } from '@/components/ui/sticky-toolbar';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import { Plus, Search, Download } from 'lucide-react';
import { downloadCsv } from '@/lib/export';
import type { Contact } from '@/types/contacts';
import { useI18n } from '@/i18n';
import { useSettings } from '@/hooks/useSettingsApi';
import { formatDate, formatNumber } from '@/lib/locale';

function getDunningBadge(status: string, dueDate: string) {
  if (status !== 'sent' && status !== 'overdue') return null;
  const due = new Date(dueDate);
  const now = new Date();
  const days = Math.floor((now.getTime() - due.getTime()) / (1000 * 60 * 60 * 24));
  if (days < 14) return null;
  if (days < 30) return { level: 'first', color: 'bg-yellow-100 text-yellow-800' };
  if (days < 45) return { level: 'second', color: 'bg-orange-100 text-orange-800' };
  return { level: 'third', color: 'bg-red-100 text-red-800' };
}

const statusVariant: Record<string, 'default' | 'secondary' | 'destructive' | 'outline'> = {
  draft: 'secondary',
  sent: 'default',
  paid: 'outline',
  overdue: 'destructive',
  cancelled: 'destructive',
};

export function InvoicesPage() {
  const { t } = useI18n();
  const { data: settings } = useSettings();
  const [search, setSearch] = useState('');
  const [statusFilter, setStatusFilter] = useState<string | undefined>();
  const [page, setPage] = useState(1);
  const statusTabs: { label: string; value: string | undefined }[] = [
    { label: t('invoices.filter.all', 'All'), value: undefined },
    { label: t('invoices.filter.draft', 'Draft'), value: 'draft' },
    { label: t('invoices.filter.sent', 'Sent'), value: 'sent' },
    { label: t('invoices.filter.paid', 'Paid'), value: 'paid' },
    { label: t('invoices.filter.overdue', 'Overdue'), value: 'overdue' },
    { label: t('invoices.filter.cancelled', 'Cancelled'), value: 'cancelled' },
  ];
  const dateFormat = settings?.date_format ?? 'dd.MM.yyyy';
  const numberFormat = settings?.number_format ?? 'ch';

  const { data, isLoading } = useInvoices({
    search: search || undefined,
    status: statusFilter,
    page,
  });

  const contacts = useContacts({ per_page: 500 });
  const invoices = data?.data ?? [];

  const contactMap = useMemo(() => {
    const list: Contact[] = contacts.data?.data ?? [];
    return new Map(
      list.map((c) => [c.id, c.name2 ? `${c.name1} (${c.name2})` : c.name1]),
    );
  }, [contacts.data]);

  const navigate = useNavigate();

  const actions: ToolbarAction[] = [
    {
      icon: <Plus className="h-4 w-4" />,
      label: t('invoices.new_invoice', 'New Invoice'),
      onClick: () => navigate('/invoices/new'),
      primary: true,
    },
  ];

  const overflow: ToolbarOverflowItem[] = [
    {
      icon: <Download className="h-4 w-4" />,
      label: t('invoices.export_csv', 'Export CSV'),
      onClick: () => downloadCsv('/invoices'),
    },
  ];

  return (
    <div className="space-y-4">
      <StickyToolbar actions={actions} overflow={overflow}>
        <Badge variant="secondary">
          {t('invoices.subtitle', 'Manage customer invoices')}
        </Badge>
      </StickyToolbar>

      <div className="flex flex-wrap gap-1 border-b pb-2">
        {statusTabs.map((tab) => (
          <Button
            key={tab.label}
            variant={statusFilter === tab.value ? 'default' : 'ghost'}
            size="sm"
            onClick={() => { setStatusFilter(tab.value); setPage(1); }}
          >
            {tab.label}
          </Button>
        ))}
      </div>

      <div className="relative">
        <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
        <Input
          placeholder={t('invoices.search_placeholder', 'Search invoices...')}
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
          ) : invoices.length > 0 ? (
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>{t('invoices.table.number', 'Number')}</TableHead>
                  <TableHead>{t('invoices.table.contact', 'Contact')}</TableHead>
                  <TableHead className="hidden sm:table-cell">{t('invoices.table.issue_date', 'Issue Date')}</TableHead>
                  <TableHead className="hidden md:table-cell">{t('invoices.table.due_date', 'Due Date')}</TableHead>
                  <TableHead className="text-right">{t('invoices.table.total', 'Total')}</TableHead>
                  <TableHead>{t('invoices.table.status', 'Status')}</TableHead>
                  <TableHead className="hidden lg:table-cell">{t('invoices.table.dunning', 'Dunning')}</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {invoices.map((inv) => (
                  <TableRow key={inv.id}>
                    <TableCell>
                      <Link
                        to={`/invoices/${inv.id}`}
                        className="font-mono text-sm font-medium text-primary hover:underline"
                      >
                        {inv.invoice_number ?? t('invoices.draft_label', 'DRAFT')}
                      </Link>
                    </TableCell>
                    <TableCell>{contactMap.get(inv.contact_id) || inv.contact_id}</TableCell>
                    <TableCell className="hidden sm:table-cell font-mono text-sm">
                      {formatDate(inv.issue_date, dateFormat)}
                    </TableCell>
                    <TableCell className="hidden md:table-cell font-mono text-sm">
                      {formatDate(inv.due_date, dateFormat)}
                    </TableCell>
                    <TableCell className="text-right font-mono text-sm font-medium">
                      {formatNumber(inv.total, numberFormat)}
                    </TableCell>
                    <TableCell>
                      <Badge variant={statusVariant[inv.status] ?? 'outline'}>
                        {t(`status.${inv.status}`, inv.status)}
                      </Badge>
                    </TableCell>
                    <TableCell className="hidden lg:table-cell">
                      {(() => {
                        const badge = getDunningBadge(inv.status, inv.due_date);
                        if (!badge) return null;
                        const variant = badge.level === 'third' ? 'destructive' : badge.level === 'second' ? 'default' : 'secondary';
                        return (
                          <Badge variant={variant}>
                            {t(`dunning.${badge.level}`, badge.level)}
                          </Badge>
                        );
                      })()}
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          ) : (
            <p className="py-8 text-center text-sm text-muted-foreground">
              {t('invoices.no_results', 'No invoices found. Create your first invoice.')}
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
