import { useState, useEffect } from 'react';
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Skeleton } from '@/components/ui/skeleton';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { defaultAccountsApi } from '@/api/default-accounts';
import { useAccounts } from '@/hooks/useApi';
import { toast } from 'sonner';
import { Save } from 'lucide-react';
import { useI18n } from '@/i18n';
import { StickyToolbar, type ToolbarAction } from '@/components/ui/sticky-toolbar';
import type { DefaultAccount } from '@/types/default-account';

const SETTING_LABELS: Record<string, { label: string; group: string }> = {
  ar_account: { label: 'Accounts Receivable (Debitoren)', group: 'Balance Sheet' },
  ap_account: { label: 'Accounts Payable (Kreditoren)', group: 'Balance Sheet' },
  bank_default: { label: 'Default Bank Account', group: 'Balance Sheet' },
  cash_account: { label: 'Cash Account (Kasse)', group: 'Balance Sheet' },
  vat_payable: { label: 'VAT Payable (MWST Schuld)', group: 'Balance Sheet' },
  vat_receivable: { label: 'Input VAT (Vorsteuer)', group: 'Balance Sheet' },
  retained_earnings: { label: 'Retained Earnings (Gewinnvortrag)', group: 'Balance Sheet' },
  revenue_default: { label: 'Default Revenue Account', group: 'Income / Expense' },
  expense_default: { label: 'Default Expense Account', group: 'Income / Expense' },
  wage_expense: { label: 'Wage Expense (Lohnaufwand)', group: 'Income / Expense' },
};

export function DefaultAccountsPage() {
  const { t } = useI18n();
  const queryClient = useQueryClient();
  const { data: defaults, isLoading } = useQuery({
    queryKey: ['default-accounts'],
    queryFn: () => defaultAccountsApi.list().then((r) => r.data),
  });
  const { data: accountsData } = useAccounts({ per_page: 500 });
  const accounts = accountsData?.data ?? [];

  const [edits, setEdits] = useState<Record<string, string | null>>({});
  const [hasChanges, setHasChanges] = useState(false);

  useEffect(() => {
    if (defaults) {
      const initial: Record<string, string | null> = {};
      defaults.forEach((d) => { initial[d.setting_key] = d.account_id; });
      setEdits(initial);
      setHasChanges(false);
    }
  }, [defaults]);

  const mutation = useMutation({
    mutationFn: (settings: { setting_key: string; account_id: string | null }[]) =>
      defaultAccountsApi.update(settings),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['default-accounts'] });
      toast.success('Default accounts saved');
      setHasChanges(false);
    },
    onError: () => toast.error('Failed to save default accounts'),
  });

  function handleChange(key: string, value: string) {
    const accountId = value === '__none__' ? null : value;
    setEdits((prev) => ({ ...prev, [key]: accountId }));
    setHasChanges(true);
  }

  function handleSave() {
    const settings = Object.entries(edits).map(([setting_key, account_id]) => ({
      setting_key,
      account_id,
    }));
    mutation.mutate(settings);
  }

  const groups = new Map<string, DefaultAccount[]>();
  (defaults ?? []).forEach((d) => {
    const group = SETTING_LABELS[d.setting_key]?.group ?? 'Other';
    if (!groups.has(group)) groups.set(group, []);
    groups.get(group)!.push(d);
  });

  return (
    <div className="space-y-4">
      <StickyToolbar
        actions={[
          { icon: <Save className="h-4 w-4" />, label: t('common.save', 'Save Changes'), onClick: handleSave, disabled: !hasChanges || mutation.isPending, loading: mutation.isPending, primary: true },
        ] satisfies ToolbarAction[]}
      >
        <span />
      </StickyToolbar>

      {isLoading ? (
        <div className="space-y-2">
          {Array.from({ length: 5 }).map((_, i) => <Skeleton key={i} className="h-16 w-full" />)}
        </div>
      ) : (
        Array.from(groups.entries()).map(([group, items]) => (
          <Card key={group}>
            <CardHeader className="pb-3">
              <CardTitle className="text-base">{group}</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              {items.map((da) => {
                const meta = SETTING_LABELS[da.setting_key];
                return (
                  <div key={da.setting_key} className="grid gap-2 sm:grid-cols-[1fr_300px] sm:items-center">
                    <div>
                      <p className="text-sm font-medium">{meta?.label ?? da.setting_key}</p>
                      {da.description && (
                        <p className="text-xs text-muted-foreground">{da.description}</p>
                      )}
                    </div>
                    <Select
                      value={edits[da.setting_key] ?? '__none__'}
                      onValueChange={(v) => handleChange(da.setting_key, v)}
                    >
                      <SelectTrigger>
                        <SelectValue placeholder="Select account..." />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="__none__">— None —</SelectItem>
                        {accounts.map((a) => (
                          <SelectItem key={a.id} value={a.id}>
                            {a.number} · {a.name}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                  </div>
                );
              })}
            </CardContent>
          </Card>
        ))
      )}
    </div>
  );
}
