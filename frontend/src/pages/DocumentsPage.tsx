import { useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Badge } from '@/components/ui/badge';
import { Skeleton } from '@/components/ui/skeleton';
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
import { useDocuments } from '@/hooks/useDocumentsApi';
import { Plus, Search } from 'lucide-react';
import { useI18n } from '@/i18n';
import { useSettings } from '@/hooks/useSettingsApi';
import { formatDate, formatNumber } from '@/lib/locale';

const typeVariant: Record<string, 'default' | 'secondary' | 'outline'> = {
  quote: 'secondary',
  offer: 'secondary',
  sow: 'default',
  contract: 'outline',
};

const statusVariant: Record<string, 'default' | 'secondary' | 'destructive' | 'outline'> = {
  draft: 'secondary',
  sent: 'default',
  accepted: 'outline',
  signed: 'outline',
  rejected: 'destructive',
  completed: 'outline',
};

export function DocumentsPage() {
  const { t } = useI18n();
  const { data: settings } = useSettings();
  const [search, setSearch] = useState('');
  const [typeFilter, setTypeFilter] = useState<string | undefined>();
  const [statusFilter, setStatusFilter] = useState<string | undefined>();
  const [page, setPage] = useState(1);
  const typeTabs = [
    { label: t('common.all', 'All'), value: undefined as string | undefined },
    { label: t('documents.type.quotes', 'Quotes'), value: 'quote' },
    { label: t('documents.type.offers', 'Offers'), value: 'offer' },
    { label: t('documents.type.sows', 'SOWs'), value: 'sow' },
    { label: t('documents.type.contracts', 'Contracts'), value: 'contract' },
  ];
  const statusPills = [
    { label: t('common.all', 'All'), value: undefined as string | undefined },
    { label: t('status.draft', 'Draft'), value: 'draft' },
    { label: t('status.sent', 'Sent'), value: 'sent' },
    { label: t('documents.status.accepted', 'Accepted'), value: 'accepted' },
    { label: t('documents.status.signed', 'Signed'), value: 'signed' },
    { label: t('documents.status.rejected', 'Rejected'), value: 'rejected' },
  ];
  const docTypes = [
    { value: 'quote', label: t('documents.type.quote', 'Quote') },
    { value: 'offer', label: t('documents.type.offer', 'Offer') },
    { value: 'sow', label: t('documents.type.sow', 'SOW') },
    { value: 'contract', label: t('documents.type.contract', 'Contract') },
  ];
  const dateFormat = settings?.date_format ?? 'dd.MM.yyyy';
  const numberFormat = settings?.number_format ?? 'ch';

  const navigate = useNavigate();

  const { data, isLoading } = useDocuments({
    search: search || undefined,
    doc_type: typeFilter,
    status: statusFilter,
    page,
  });

  const documents = data?.data ?? [];

  const actions: ToolbarAction[] = [
    {
      icon: <Plus className="h-4 w-4" />,
      label: t('documents.new_document', 'New Document'),
      onClick: () => navigate('/documents/new?type=quote'),
      primary: true,
    },
  ];

  const overflow: ToolbarOverflowItem[] = docTypes.map((dt) => ({
    icon: <Plus className="h-4 w-4" />,
    label: dt.label,
    onClick: () => navigate(`/documents/new?type=${dt.value}`),
  }));

  return (
    <div className="space-y-4">
      <StickyToolbar actions={actions} overflow={overflow}>
        <Badge variant="secondary">
          {t('documents.subtitle', 'Manage quotes, offers, SOWs, and contracts')}
        </Badge>
      </StickyToolbar>

      <div className="flex flex-wrap gap-1">
        {typeTabs.map((tab) => (
          <Button
            key={tab.label}
            variant={typeFilter === tab.value ? 'default' : 'outline'}
            size="sm"
            onClick={() => { setTypeFilter(tab.value); setPage(1); }}
          >
            {tab.label}
          </Button>
        ))}
      </div>

      <div className="flex flex-wrap gap-1">
        {statusPills.map((pill) => (
          <Button
            key={pill.label}
            variant={statusFilter === pill.value ? 'default' : 'ghost'}
            size="sm"
            className="h-7 text-xs"
            onClick={() => { setStatusFilter(pill.value); setPage(1); }}
          >
            {pill.label}
          </Button>
        ))}
      </div>

      <div className="relative">
        <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
        <Input
          placeholder={t('documents.search_placeholder', 'Search documents...')}
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
          ) : documents.length > 0 ? (
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>{t('common.number', 'Number')}</TableHead>
                  <TableHead>{t('common.title', 'Title')}</TableHead>
                  <TableHead className="hidden sm:table-cell">{t('common.type', 'Type')}</TableHead>
                  <TableHead className="hidden md:table-cell">{t('common.status', 'Status')}</TableHead>
                  <TableHead className="text-right">{t('common.total', 'Total')}</TableHead>
                  <TableHead className="hidden lg:table-cell">{t('common.date', 'Date')}</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {documents.map((doc) => (
                  <TableRow key={doc.id}>
                    <TableCell>
                      <Link
                        to={`/documents/${doc.id}`}
                        className="font-mono text-sm font-medium text-primary hover:underline"
                      >
                        {doc.doc_number ?? t('invoices.draft_label', 'DRAFT')}
                      </Link>
                    </TableCell>
                    <TableCell className="max-w-[200px] truncate">
                      {doc.title}
                    </TableCell>
                    <TableCell className="hidden sm:table-cell">
                      <Badge variant={typeVariant[doc.doc_type] ?? 'outline'}>
                        {t(`documents.type.${doc.doc_type}`, doc.doc_type)}
                      </Badge>
                    </TableCell>
                    <TableCell className="hidden md:table-cell">
                      <Badge variant={statusVariant[doc.status] ?? 'outline'}>
                        {t(`documents.status.${doc.status}`, t(`status.${doc.status}`, doc.status))}
                      </Badge>
                    </TableCell>
                    <TableCell className="text-right font-mono text-sm font-medium">
                      {formatNumber(doc.total, numberFormat)}
                    </TableCell>
                    <TableCell className="hidden lg:table-cell font-mono text-sm">
                      {doc.created_at ? formatDate(doc.created_at.split('T')[0], dateFormat) : ''}
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          ) : (
            <p className="py-8 text-center text-sm text-muted-foreground">
              {t('documents.no_results', 'No documents found.')}
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
