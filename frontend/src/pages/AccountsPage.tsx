import { useState, useMemo } from 'react';
import { Link } from 'react-router-dom';
import { useAccountTreeWithBalances, useCreateAccount, useUpdateAccount } from '@/hooks/useApi';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Skeleton } from '@/components/ui/skeleton';
import { StickyToolbar, type ToolbarAction } from '@/components/ui/sticky-toolbar';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { Badge } from '@/components/ui/badge';
import {
  ChevronRight,
  ChevronDown,
  Plus,
  Pencil,
  Search,
} from 'lucide-react';
import { toast } from 'sonner';
import type { AccountTreeWithBalance } from '@/types/accounts';
import { useI18n } from '@/i18n';

const typeColors: Record<string, string> = {
  asset: 'bg-blue-100 text-blue-800',
  liability: 'bg-red-100 text-red-800',
  equity: 'bg-purple-100 text-purple-800',
  revenue: 'bg-green-100 text-green-800',
  expense: 'bg-orange-100 text-orange-800',
};

const classConfig: { key: string; label: string; types: string[]; color: string }[] = [
  { key: '1', label: '1 — Assets', types: ['asset'], color: 'text-blue-700 bg-blue-50 border-blue-200' },
  { key: '2', label: '2 — Liabilities & Equity', types: ['liability', 'equity'], color: 'text-red-700 bg-red-50 border-red-200' },
  { key: '3', label: '3 — Revenue', types: ['revenue'], color: 'text-green-700 bg-green-50 border-green-200' },
  { key: '4-9', label: '4–9 — Expenses', types: ['expense'], color: 'text-orange-700 bg-orange-50 border-orange-200' },
];

function formatBalance(balance: string): string {
  const num = parseFloat(balance);
  if (isNaN(num)) return '0.00';
  return num.toLocaleString('de-CH', { minimumFractionDigits: 2, maximumFractionDigits: 2 });
}

function matchesSearch(account: AccountTreeWithBalance, term: string): boolean {
  const lower = term.toLowerCase();
  if (account.number.toString().includes(term)) return true;
  if (account.name.toLowerCase().includes(lower)) return true;
  return account.children.some((c) => matchesSearch(c, term));
}

function countAccounts(nodes: AccountTreeWithBalance[]): number {
  let count = 0;
  for (const n of nodes) {
    count += 1 + countAccounts(n.children);
  }
  return count;
}

function sumBalances(nodes: AccountTreeWithBalance[]): number {
  let total = 0;
  for (const n of nodes) {
    total += parseFloat(n.balance) || 0;
    total += sumBalances(n.children);
  }
  return total;
}

function AccountNode({
  account,
  level,
  onEdit,
  search,
  forceExpand,
}: {
  account: AccountTreeWithBalance;
  level: number;
  onEdit: (a: AccountTreeWithBalance) => void;
  search: string;
  forceExpand: boolean | null;
}) {
  const [expanded, setExpanded] = useState(level < 1);
  const hasChildren = account.children.length > 0;
  const isExpanded = forceExpand !== null ? forceExpand : (search ? true : expanded);
  const bal = parseFloat(account.balance) || 0;

  return (
    <div>
      <div
        className="group flex items-center gap-2 rounded-md px-2 py-1.5 hover:bg-muted"
        style={{ paddingLeft: `${level * 20 + 8}px` }}
      >
        {hasChildren ? (
          <button onClick={() => setExpanded(!expanded)} className="p-0.5">
            {isExpanded ? (
              <ChevronDown className="h-4 w-4 text-muted-foreground" />
            ) : (
              <ChevronRight className="h-4 w-4 text-muted-foreground" />
            )}
          </button>
        ) : (
          <span className="w-5" />
        )}
        <Link
          to={`/reports/account-ledger?account_id=${account.id}`}
          className="font-mono text-sm text-muted-foreground hover:text-primary hover:underline"
        >
          {account.number}
        </Link>
        <div className="min-w-0 flex-1">
          <Link
            to={`/reports/account-ledger?account_id=${account.id}`}
            className="text-sm font-medium hover:text-primary hover:underline"
          >
            {account.name}
          </Link>
        </div>
        <span
          className={`mr-2 font-mono text-sm tabular-nums ${bal < 0 ? 'text-red-600' : 'text-foreground'}`}
        >
          {formatBalance(account.balance)}
        </span>
        <Badge
          variant="secondary"
          className={`text-xs ${typeColors[account.account_type] ?? ''}`}
        >
          {account.account_type}
        </Badge>
        <button
          onClick={() => onEdit(account)}
          className="ml-1 opacity-0 group-hover:opacity-100 transition-opacity"
        >
          <Pencil className="h-3.5 w-3.5 text-muted-foreground hover:text-foreground" />
        </button>
      </div>
      {isExpanded && hasChildren && (
        <div>
          {account.children.map((child) => (
            <AccountNode
              key={child.id}
              account={child}
              level={level + 1}
              onEdit={onEdit}
              search={search}
              forceExpand={forceExpand}
            />
          ))}
        </div>
      )}
    </div>
  );
}

export function AccountsPage() {
  const { data: tree, isLoading } = useAccountTreeWithBalances();
  const createAccount = useCreateAccount();
  const updateAccount = useUpdateAccount();
  const [createOpen, setCreateOpen] = useState(false);
  const [editOpen, setEditOpen] = useState(false);
  const [search, setSearch] = useState('');
  const [allExpanded, _setAllExpanded] = useState(true);
  const [expandKey, _setExpandKey] = useState(0);
  const [collapsedGroups, setCollapsedGroups] = useState<Set<string>>(new Set());
  const [createForm, setCreateForm] = useState({ number: '', name: '', description: '' });
  const [editForm, setEditForm] = useState({ id: '', name: '', description: '', is_active: true });

  const grouped = useMemo(() => {
    if (!tree) return [];
    return classConfig.map((cls) => {
      const accounts = tree.filter((a) => cls.types.includes(a.account_type));
      const filtered = search
        ? accounts.filter((a) => matchesSearch(a, search))
        : accounts;
      const total = sumBalances(filtered);
      return { ...cls, accounts: filtered, total };
    }).filter((g) => g.accounts.length > 0);
  }, [tree, search]);

  const totalAccounts = useMemo(() => (tree ? countAccounts(tree) : 0), [tree]);

  function handleCreate() {
    const num = parseInt(createForm.number, 10);
    if (isNaN(num)) {
      toast.error('Account number must be numeric');
      return;
    }
    createAccount.mutate(
      { number: num, name: createForm.name },
      {
        onSuccess: () => {
          toast.success('Account created');
          setCreateOpen(false);
          setCreateForm({ number: '', name: '', description: '' });
        },
        onError: () => toast.error('Failed to create account'),
      }
    );
  }

  function handleEdit(account: AccountTreeWithBalance) {
    setEditForm({
      id: account.id,
      name: account.name,
      description: '',
      is_active: account.is_active,
    });
    setEditOpen(true);
  }

  function handleUpdate() {
    updateAccount.mutate(
      {
        id: editForm.id,
        data: {
          name: editForm.name,
          description: editForm.description || null,
          is_active: editForm.is_active,
        },
      },
      {
        onSuccess: () => {
          toast.success('Account updated');
          setEditOpen(false);
        },
        onError: () => toast.error('Failed to update account'),
      }
    );
  }

  const { t } = useI18n();

  const toolbarActions: ToolbarAction[] = [
    {
      icon: <Plus className="h-4 w-4" />,
      label: t('accounts.add_account', 'Add Account'),
      onClick: () => setCreateOpen(true),
      primary: true,
    },
  ];

  return (
    <div className="space-y-4">
      <StickyToolbar actions={toolbarActions}>
        <Badge variant="secondary">{totalAccounts} {t('accounts.accounts', 'accounts')}</Badge>
        <div className="relative flex-1 min-w-[120px] max-w-xs">
          <Search className="absolute left-2.5 top-2 h-4 w-4 text-muted-foreground" />
          <Input
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            placeholder={t('accounts.search_placeholder', 'Search...')}
            className="pl-9 h-8 text-sm"
          />
        </div>
      </StickyToolbar>

      {/* Create Dialog */}
      <Dialog open={createOpen} onOpenChange={setCreateOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>{t('accounts.create_account', 'Create Account')}</DialogTitle>
            <DialogDescription>{t('accounts.create_description', 'Add a new account to the chart of accounts.')}</DialogDescription>
          </DialogHeader>
          <div className="space-y-4">
            <div>
              <Label>{t('accounts.account_number', 'Account Number')}</Label>
              <Input
                value={createForm.number}
                onChange={(e) => setCreateForm({ ...createForm, number: e.target.value })}
                placeholder="1000"
              />
            </div>
            <div>
              <Label>{t('common.name', 'Name')}</Label>
              <Input
                value={createForm.name}
                onChange={(e) => setCreateForm({ ...createForm, name: e.target.value })}
                placeholder="Cash & Equivalents"
              />
            </div>
            <div>
              <Label>{t('common.description', 'Description')}</Label>
              <Input
                value={createForm.description}
                onChange={(e) => setCreateForm({ ...createForm, description: e.target.value })}
                placeholder={t('common.optional_description', 'Optional description')}
              />
            </div>
            <Button onClick={handleCreate} className="w-full" disabled={createAccount.isPending}>
              {t('accounts.create_account', 'Create Account')}
            </Button>
          </div>
        </DialogContent>
      </Dialog>

      {/* Edit Dialog */}
      <Dialog open={editOpen} onOpenChange={setEditOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>{t('accounts.edit_account', 'Edit Account')}</DialogTitle>
            <DialogDescription>{t('accounts.edit_description', 'Update account details.')}</DialogDescription>
          </DialogHeader>
          <div className="space-y-4">
            <div>
              <Label>{t('common.name', 'Name')}</Label>
              <Input
                value={editForm.name}
                onChange={(e) => setEditForm({ ...editForm, name: e.target.value })}
              />
            </div>
            <div>
              <Label>{t('common.description', 'Description')}</Label>
              <Input
                value={editForm.description}
                onChange={(e) => setEditForm({ ...editForm, description: e.target.value })}
                placeholder={t('common.optional_description', 'Optional description')}
              />
            </div>
            <div className="flex items-center gap-2">
              <input
                type="checkbox"
                id="is_active"
                checked={editForm.is_active}
                onChange={(e) => setEditForm({ ...editForm, is_active: e.target.checked })}
                className="h-4 w-4 rounded border-gray-300"
              />
              <Label htmlFor="is_active">{t('common.active', 'Active')}</Label>
            </div>
            <Button onClick={handleUpdate} className="w-full" disabled={updateAccount.isPending}>
              {t('common.save_changes', 'Save Changes')}
            </Button>
          </div>
        </DialogContent>
      </Dialog>

      {isLoading ? (
        <div className="space-y-2">
          {Array.from({ length: 5 }).map((_, i) => (
            <Skeleton key={i} className="h-8 w-full" />
          ))}
        </div>
      ) : grouped.length > 0 ? (
        <div className="space-y-4">
          {grouped.map((group) => {
            const isGroupCollapsed = collapsedGroups.has(group.key);
            return (
              <Card key={group.key} className="gap-0 py-0">
                <button
                  onClick={() =>
                    setCollapsedGroups((prev) => {
                      const next = new Set(prev);
                      if (next.has(group.key)) next.delete(group.key);
                      else next.add(group.key);
                      return next;
                    })
                  }
                  className={`flex w-full items-center justify-between border-b px-4 py-2.5 transition-colors ${group.color}`}
                >
                  <div className="flex items-center gap-1.5">
                    {isGroupCollapsed ? (
                      <ChevronRight className="h-4 w-4" />
                    ) : (
                      <ChevronDown className="h-4 w-4" />
                    )}
                    <span className="text-sm font-semibold">{group.label}</span>
                  </div>
                  <span className="font-mono text-sm tabular-nums">
                    {group.total.toLocaleString('de-CH', {
                      minimumFractionDigits: 2,
                      maximumFractionDigits: 2,
                    })}
                  </span>
                </button>
                {!isGroupCollapsed && (
                  <CardContent className="px-2 py-2">
                    <div className="space-y-0.5">
                      {group.accounts.map((account) => (
                        <AccountNode
                          key={`${account.id}-${expandKey}`}
                          account={account}
                          level={0}
                          onEdit={handleEdit}
                          search={search}
                          forceExpand={search ? true : allExpanded}
                        />
                      ))}
                    </div>
                  </CardContent>
                )}
              </Card>
            );
          })}
        </div>
      ) : (
        <Card>
          <CardContent className="py-8 text-center text-sm text-muted-foreground">
            {search ? 'No accounts match your search.' : 'No accounts yet. Add your first account to get started.'}
          </CardContent>
        </Card>
      )}
    </div>
  );
}
