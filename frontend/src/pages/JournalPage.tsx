import { useNavigate, useSearchParams } from 'react-router-dom';
import { Link } from 'react-router-dom';
import {
  useJournalEntries,
  usePostJournal,
  useReverseJournal,
  useBulkPostJournal,
} from '@/hooks/useApi';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Skeleton } from '@/components/ui/skeleton';
import { Badge } from '@/components/ui/badge';
import { Pagination } from '@/components/ui/pagination';
import { StickyToolbar, type ToolbarAction, type ToolbarOverflowItem } from '@/components/ui/sticky-toolbar';
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
import { Plus, CheckCircle, CheckCircle2, Undo2, Download, ArrowUp, ArrowDown, ArrowUpDown } from 'lucide-react';
import { useI18n } from '@/i18n';
import { toast } from 'sonner';
import { downloadCsv } from '@/lib/export';
import { useCallback, useState } from 'react';

const statusColors: Record<string, string> = {
  draft: 'bg-yellow-100 text-yellow-800',
  posted: 'bg-green-100 text-green-800',
  reversed: 'bg-red-100 text-red-800',
};

type SortField = 'date' | 'reference' | 'description' | 'status';
type SortOrder = 'asc' | 'desc';

function SortIcon({ field, active, order }: { field: string; active: string; order: SortOrder }) {
  if (field !== active) return <ArrowUpDown className="ml-1 h-3 w-3 text-muted-foreground/50" />;
  return order === 'asc'
    ? <ArrowUp className="ml-1 h-3 w-3" />
    : <ArrowDown className="ml-1 h-3 w-3" />;
}

export function JournalPage() {
  const navigate = useNavigate();
  const [searchParams, setSearchParams] = useSearchParams();

  const dateFrom = searchParams.get('from') ?? '';
  const dateTo = searchParams.get('to') ?? '';
  const page = parseInt(searchParams.get('page') ?? '1', 10) || 1;
  const sortBy = (searchParams.get('sort') ?? 'date') as SortField;
  const sortOrder = (searchParams.get('dir') ?? 'desc') as SortOrder;

  const updateParams = useCallback((updates: Record<string, string | undefined>) => {
    setSearchParams((prev) => {
      const next = new URLSearchParams(prev);
      for (const [k, v] of Object.entries(updates)) {
        if (v === undefined || v === '') next.delete(k);
        else next.set(k, v);
      }
      return next;
    }, { replace: true });
  }, [setSearchParams]);

  const { data, isLoading } = useJournalEntries({
    date_from: dateFrom || undefined,
    date_to: dateTo || undefined,
    page,
    sort_by: sortBy,
    sort_order: sortOrder,
  });
  const postJournal = usePostJournal();
  const reverseJournal = useReverseJournal();
  const bulkPost = useBulkPostJournal();
  const { t } = useI18n();

  function handlePost(id: string) {
    postJournal.mutate(id, {
      onSuccess: () => toast.success('Entry posted'),
      onError: () => toast.error('Failed to post entry'),
    });
  }

  function handleReverse(id: string) {
    reverseJournal.mutate(id, {
      onSuccess: () => toast.success('Entry reversed'),
      onError: () => toast.error('Failed to reverse entry'),
    });
  }

  function handleSort(field: SortField) {
    if (sortBy === field) {
      updateParams({ dir: sortOrder === 'asc' ? 'desc' : 'asc', page: '1' });
    } else {
      updateParams({ sort: field, dir: field === 'date' ? 'desc' : 'asc', page: '1' });
    }
  }

  const entries = data?.data ?? [];
  const draftCount = entries.filter((e) => e.status === 'draft').length;
  const [bulkPostOpen, setBulkPostOpen] = useState(false);

  function handleBulkPost() {
    bulkPost.mutate(
      { allDrafts: true },
      {
        onSuccess: (result) => { toast.success(t('journal.bulk_posted', `${result.posted} entries posted`).replace('{n}', String(result.posted))); setBulkPostOpen(false); },
        onError: () => toast.error(t('journal.bulk_post_error', 'Failed to post entries')),
      },
    );
  }

  const actions: ToolbarAction[] = [
    ...(draftCount > 0
      ? [{
          icon: <CheckCircle2 className="h-4 w-4" />,
          label: t('journal.bulk_post', 'Post All Drafts'),
          onClick: () => setBulkPostOpen(true),
        }]
      : []),
    {
      icon: <Plus className="h-4 w-4" />,
      label: t('journal.new_entry', 'New Entry'),
      onClick: () => navigate('/journal/new'),
      primary: true,
    },
  ];

  const overflow: ToolbarOverflowItem[] = [
    {
      icon: <Download className="h-4 w-4" />,
      label: t('invoices.export_csv', 'Export CSV'),
      onClick: () => downloadCsv('/journal'),
    },
  ];

  return (
    <div className="space-y-4">
      <StickyToolbar actions={actions} overflow={overflow}>
        <Badge variant="secondary">
          {t('journal.subtitle', 'Record and view transactions')}
          {data && data.total > 0 && ` (${data.total})`}
        </Badge>
      </StickyToolbar>

      {/* Bulk Post Confirmation */}
      <AlertDialog open={bulkPostOpen} onOpenChange={setBulkPostOpen}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>{t('journal.bulk_post', 'Post All Drafts')}</AlertDialogTitle>
            <AlertDialogDescription>
              {t('journal.bulk_post_confirm', 'Post all draft entries? This cannot be undone.')}
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>{t('common.cancel', 'Cancel')}</AlertDialogCancel>
            <AlertDialogAction onClick={handleBulkPost}>{t('journal.bulk_post', 'Post All Drafts')}</AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>

      <div className="flex gap-3">
        <div>
          <Label className="text-xs">From</Label>
          <Input
            type="date"
            value={dateFrom}
            onChange={(e) => updateParams({ from: e.target.value, page: '1' })}
            className="w-40"
          />
        </div>
        <div>
          <Label className="text-xs">To</Label>
          <Input
            type="date"
            value={dateTo}
            onChange={(e) => updateParams({ to: e.target.value, page: '1' })}
            className="w-40"
          />
        </div>
        {(dateFrom || dateTo) && (
          <div className="flex items-end">
            <Button
              variant="ghost"
              size="sm"
              onClick={() => updateParams({ from: undefined, to: undefined, page: '1' })}
            >
              Clear
            </Button>
          </div>
        )}
      </div>

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
                  <TableHead
                    className="cursor-pointer select-none"
                    onClick={() => handleSort('date')}
                  >
                    <span className="inline-flex items-center">
                      Date
                      <SortIcon field="date" active={sortBy} order={sortOrder} />
                    </span>
                  </TableHead>
                  <TableHead
                    className="cursor-pointer select-none"
                    onClick={() => handleSort('reference')}
                  >
                    <span className="inline-flex items-center">
                      Reference
                      <SortIcon field="reference" active={sortBy} order={sortOrder} />
                    </span>
                  </TableHead>
                  <TableHead
                    className="cursor-pointer select-none"
                    onClick={() => handleSort('description')}
                  >
                    <span className="inline-flex items-center">
                      Description
                      <SortIcon field="description" active={sortBy} order={sortOrder} />
                    </span>
                  </TableHead>
                  <TableHead
                    className="hidden sm:table-cell cursor-pointer select-none"
                    onClick={() => handleSort('status')}
                  >
                    <span className="inline-flex items-center">
                      Status
                      <SortIcon field="status" active={sortBy} order={sortOrder} />
                    </span>
                  </TableHead>
                  <TableHead className="w-24" />
                </TableRow>
              </TableHeader>
              <TableBody>
                {entries.map((entry) => (
                  <TableRow
                    key={entry.id}
                    className="cursor-pointer"
                    onClick={() => navigate(`/journal/${entry.id}`)}
                  >
                    <TableCell className="font-mono text-sm">{entry.date}</TableCell>
                    <TableCell>{entry.reference}</TableCell>
                    <TableCell>{entry.description}</TableCell>
                    <TableCell className="hidden sm:table-cell">
                      <Badge variant="secondary" className={statusColors[entry.status] ?? ''}>
                        {entry.status}
                      </Badge>
                    </TableCell>
                    <TableCell onClick={(e) => e.stopPropagation()}>
                      <div className="flex gap-1">
                        {entry.status === 'draft' && (
                          <PostButton onPost={() => handlePost(entry.id)} />
                        )}
                        {entry.status === 'posted' && (
                          <ReverseButton onReverse={() => handleReverse(entry.id)} />
                        )}
                      </div>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          ) : (
            <p className="py-8 text-center text-sm text-muted-foreground">
              No journal entries found.
            </p>
          )}
        </CardContent>
      </Card>

      {data && (
        <Pagination
          page={page}
          totalPages={data.total_pages}
          onPageChange={(p) => updateParams({ page: String(p) })}
        />
      )}
    </div>
  );
}

function PostButton({ onPost }: { onPost: () => void }) {
  return (
    <AlertDialog>
      <AlertDialogTrigger asChild>
        <Button variant="ghost" size="icon" title="Post entry">
          <CheckCircle className="h-4 w-4 text-green-600" />
        </Button>
      </AlertDialogTrigger>
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>Post Entry?</AlertDialogTitle>
          <AlertDialogDescription>
            This will finalize the journal entry. It cannot be edited after posting.
          </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <AlertDialogCancel>Cancel</AlertDialogCancel>
          <AlertDialogAction onClick={onPost}>Post</AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  );
}

function ReverseButton({ onReverse }: { onReverse: () => void }) {
  return (
    <AlertDialog>
      <AlertDialogTrigger asChild>
        <Button variant="ghost" size="icon" title="Reverse entry">
          <Undo2 className="h-4 w-4 text-red-600" />
        </Button>
      </AlertDialogTrigger>
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>Reverse Entry?</AlertDialogTitle>
          <AlertDialogDescription>
            This will create a reversing entry. The original will be marked as reversed.
          </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <AlertDialogCancel>Cancel</AlertDialogCancel>
          <AlertDialogAction onClick={onReverse}>Reverse</AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  );
}
