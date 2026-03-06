import { useState, useMemo } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { useExpenses, useExpenseCategories, useContacts } from '@/hooks/useApi';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Skeleton } from '@/components/ui/skeleton';
import { Badge } from '@/components/ui/badge';
import { Pagination } from '@/components/ui/pagination';
import { StickyToolbar, type ToolbarAction, type ToolbarOverflowItem } from '@/components/ui/sticky-toolbar';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import {
  Table, TableBody, TableCell, TableHead, TableHeader, TableRow,
} from '@/components/ui/table';
import { Plus, Search, Download } from 'lucide-react';
import { formatAmount } from '@/lib/format';
import { downloadCsv } from '@/lib/export';
import { useI18n } from '@/i18n';
import type { Contact } from '@/types/contacts';

const statusTabs: { label: string; value: string | undefined }[] = [
  { label: 'All', value: undefined },
  { label: 'Pending', value: 'pending' },
  { label: 'Approved', value: 'approved' },
  { label: 'Paid', value: 'paid' },
  { label: 'Cancelled', value: 'cancelled' },
];

const typeTabs: { label: string; value: string | undefined }[] = [
  { label: 'All Types', value: undefined },
  { label: 'Vendor Bills', value: 'single' },
  { label: 'Expense Reports', value: 'report' },
];

const statusVariant: Record<string, 'default' | 'secondary' | 'destructive' | 'outline'> = {
  pending: 'secondary',
  approved: 'default',
  paid: 'outline',
  cancelled: 'destructive',
};

export function ExpensesPage() {
  const { t } = useI18n();
  const navigate = useNavigate();
  const [search, setSearch] = useState('');
  const [statusFilter, setStatusFilter] = useState<string | undefined>();
  const [typeFilter, setTypeFilter] = useState<string | undefined>();
  const [categoryFilter, setCategoryFilter] = useState<string | undefined>();
  const [page, setPage] = useState(1);

  const { data, isLoading } = useExpenses({
    search: search || undefined,
    status: statusFilter,
    category_id: categoryFilter,
    page,
  });

  const { data: categories } = useExpenseCategories();
  const contacts = useContacts({ per_page: 500 });
  const allExpenses = data?.data ?? [];
  const expenses = typeFilter
    ? allExpenses.filter((e) => e.expense_type === typeFilter)
    : allExpenses;

  const categoryMap = useMemo(() => {
    return new Map((categories ?? []).map((c) => [c.id, c.name]));
  }, [categories]);

  const contactMap = useMemo(() => {
    const list: Contact[] = contacts.data?.data ?? [];
    return new Map(
      list.map((c) => [c.id, c.name2 ? `${c.name1} (${c.name2})` : c.name1]),
    );
  }, [contacts.data]);

  const actions: ToolbarAction[] = [
    {
      icon: <Plus className="h-4 w-4" />,
      label: t('expenses.new_expense', 'New Expense'),
      onClick: () => navigate('/expenses/new'),
      primary: true,
    },
  ];

  const overflow: ToolbarOverflowItem[] = [
    {
      icon: <Download className="h-4 w-4" />,
      label: t('invoices.export_csv', 'Export CSV'),
      onClick: () => downloadCsv('/expenses'),
    },
  ];

  return (
    <div className="space-y-4">
      <StickyToolbar actions={actions} overflow={overflow}>
        <Badge variant="secondary">
          {t('expenses.subtitle', 'Manage supplier bills and expenses')}
        </Badge>
      </StickyToolbar>

      <div className="flex flex-wrap items-center gap-4">
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
        <div className="h-5 w-px bg-border" />
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
      </div>

      <div className="flex gap-2">
        <div className="relative flex-1">
          <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
          <Input
            placeholder="Search expenses..."
            value={search}
            onChange={(e) => { setSearch(e.target.value); setPage(1); }}
            className="pl-9"
          />
        </div>
        <Select
          value={categoryFilter ?? '__all__'}
          onValueChange={(v) => { setCategoryFilter(v === '__all__' ? undefined : v); setPage(1); }}
        >
          <SelectTrigger className="w-48">
            <SelectValue placeholder="All categories" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="__all__">All categories</SelectItem>
            {(categories ?? []).map((c) => (
              <SelectItem key={c.id} value={c.id}>{c.name}</SelectItem>
            ))}
          </SelectContent>
        </Select>
      </div>

      <Card>
        <CardContent className="p-0">
          {isLoading ? (
            <div className="space-y-2 p-4">
              {Array.from({ length: 5 }).map((_, i) => (
                <Skeleton key={i} className="h-10 w-full" />
              ))}
            </div>
          ) : expenses.length > 0 ? (
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>Number</TableHead>
                  <TableHead className="hidden sm:table-cell">Type</TableHead>
                  <TableHead>Contact</TableHead>
                  <TableHead className="hidden sm:table-cell">Category</TableHead>
                  <TableHead>Description</TableHead>
                  <TableHead className="hidden md:table-cell">Date</TableHead>
                  <TableHead className="text-right">Total</TableHead>
                  <TableHead>Status</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {expenses.map((exp) => (
                  <TableRow key={exp.id}>
                    <TableCell>
                      <Link
                        to={`/expenses/${exp.id}`}
                        className="font-mono text-sm font-medium text-primary hover:underline"
                      >
                        {exp.expense_number ?? '—'}
                      </Link>
                    </TableCell>
                    <TableCell className="hidden sm:table-cell">
                      <Badge variant="outline" className="text-xs">
                        {exp.expense_type === 'report' ? 'Report' : 'Bill'}
                      </Badge>
                    </TableCell>
                    <TableCell className="text-sm">
                      {exp.contact_id ? (contactMap.get(exp.contact_id) || '—') : '—'}
                    </TableCell>
                    <TableCell className="hidden sm:table-cell text-sm text-muted-foreground">
                      {exp.category_id ? (categoryMap.get(exp.category_id) || '—') : '—'}
                    </TableCell>
                    <TableCell className="max-w-[200px] truncate text-sm">
                      {exp.description}
                    </TableCell>
                    <TableCell className="hidden md:table-cell font-mono text-sm">
                      {exp.expense_date}
                    </TableCell>
                    <TableCell className="text-right font-mono text-sm font-medium">
                      {formatAmount(exp.total)}
                    </TableCell>
                    <TableCell>
                      <Badge variant={statusVariant[exp.status] ?? 'outline'}>
                        {exp.status}
                      </Badge>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          ) : (
            <p className="py-8 text-center text-sm text-muted-foreground">
              No expenses found. Create your first expense.
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
