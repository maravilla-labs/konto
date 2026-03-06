import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Link } from 'react-router-dom';
import { useCreditNotes } from '@/hooks/useApi';
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
import { formatAmount } from '@/lib/format';
import { downloadCsv } from '@/lib/export';
import { useI18n } from '@/i18n';

const statusTabs: { label: string; value: string | undefined }[] = [
  { label: 'All', value: undefined },
  { label: 'Draft', value: 'draft' },
  { label: 'Issued', value: 'issued' },
  { label: 'Applied', value: 'applied' },
  { label: 'Cancelled', value: 'cancelled' },
];

const statusVariant: Record<string, 'default' | 'secondary' | 'destructive' | 'outline'> = {
  draft: 'secondary',
  issued: 'default',
  applied: 'outline',
  cancelled: 'destructive',
};

export function CreditNotesPage() {
  const { t } = useI18n();
  const navigate = useNavigate();
  const [search, setSearch] = useState('');
  const [statusFilter, setStatusFilter] = useState<string | undefined>();
  const [page, setPage] = useState(1);

  const { data, isLoading } = useCreditNotes({
    search: search || undefined,
    status: statusFilter,
    page,
  });

  const creditNotes = data?.data ?? [];

  const actions: ToolbarAction[] = [
    {
      icon: <Plus className="h-4 w-4" />,
      label: t('credit_notes.new_credit_note', 'New Credit Note'),
      onClick: () => navigate('/credit-notes/new'),
      primary: true,
    },
  ];

  const overflow: ToolbarOverflowItem[] = [
    {
      icon: <Download className="h-4 w-4" />,
      label: t('invoices.export_csv', 'Export CSV'),
      onClick: () => downloadCsv('/credit-notes'),
    },
  ];

  return (
    <div className="space-y-4">
      <StickyToolbar actions={actions} overflow={overflow}>
        <Badge variant="secondary">
          {t('credit_notes.subtitle', 'Manage credit notes / Gutschriften')}
        </Badge>
      </StickyToolbar>

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

      <div className="relative">
        <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
        <Input
          placeholder="Search credit notes..."
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
          ) : creditNotes.length > 0 ? (
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>Number</TableHead>
                  <TableHead>Contact</TableHead>
                  <TableHead className="hidden sm:table-cell">Issue Date</TableHead>
                  <TableHead className="text-right">Total</TableHead>
                  <TableHead>Status</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {creditNotes.map((cn) => (
                  <TableRow key={cn.id}>
                    <TableCell>
                      <Link
                        to={`/credit-notes/${cn.id}`}
                        className="font-mono text-sm font-medium text-primary hover:underline"
                      >
                        {cn.credit_note_number ?? 'DRAFT'}
                      </Link>
                    </TableCell>
                    <TableCell>{cn.contact_id}</TableCell>
                    <TableCell className="hidden sm:table-cell font-mono text-sm">
                      {cn.issue_date}
                    </TableCell>
                    <TableCell className="text-right font-mono text-sm font-medium">
                      {formatAmount(cn.total)}
                    </TableCell>
                    <TableCell>
                      <Badge variant={statusVariant[cn.status] ?? 'outline'}>
                        {cn.status}
                      </Badge>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          ) : (
            <p className="py-8 text-center text-sm text-muted-foreground">
              No credit notes found. Create your first credit note.
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
