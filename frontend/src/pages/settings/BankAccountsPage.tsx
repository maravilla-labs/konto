import { useState } from 'react';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Badge } from '@/components/ui/badge';
import { Checkbox } from '@/components/ui/checkbox';
import { Skeleton } from '@/components/ui/skeleton';
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import {
  useBankAccounts,
  useCreateBankAccount,
  useUpdateBankAccount,
  useDeleteBankAccount,
} from '@/hooks/useSettingsApi';
import { toast } from 'sonner';
import { Plus, Pencil, Trash2, QrCode } from 'lucide-react';
import { useI18n } from '@/i18n';
import { StickyToolbar, type ToolbarAction } from '@/components/ui/sticky-toolbar';
import type { BankAccount, CreateBankAccount } from '@/types/settings';

const emptyForm: CreateBankAccount = {
  name: '',
  bank_name: '',
  iban: '',
  bic: '',
  is_default: false,
};

export function BankAccountsPage() {
  const { t } = useI18n();
  const { data, isLoading } = useBankAccounts();
  const createAccount = useCreateBankAccount();
  const updateAccount = useUpdateBankAccount();
  const deleteAccount = useDeleteBankAccount();

  const [dialogOpen, setDialogOpen] = useState(false);
  const [editId, setEditId] = useState<string | null>(null);
  const [form, setForm] = useState<CreateBankAccount>(emptyForm);

  const accounts = data ?? [];

  function openCreate() {
    setEditId(null);
    setForm(emptyForm);
    setDialogOpen(true);
  }

  function openEdit(account: BankAccount) {
    setEditId(account.id);
    setForm({
      name: account.name,
      bank_name: account.bank_name,
      iban: account.iban,
      bic: account.bic ?? '',
      currency_id: account.currency_id,
      account_id: account.account_id,
      qr_iban: account.qr_iban ?? '',
      is_default: account.is_default,
    });
    setDialogOpen(true);
  }

  function handleSave() {
    if (editId) {
      updateAccount.mutate(
        { id: editId, data: form },
        {
          onSuccess: () => { toast.success('Bank account updated'); setDialogOpen(false); },
          onError: () => toast.error('Failed to update'),
        },
      );
    } else {
      createAccount.mutate(form, {
        onSuccess: () => { toast.success('Bank account created'); setDialogOpen(false); },
        onError: () => toast.error('Failed to create'),
      });
    }
  }

  function handleDelete(id: string) {
    deleteAccount.mutate(id, {
      onSuccess: () => toast.success('Bank account deleted'),
      onError: () => toast.error('Failed to delete'),
    });
  }

  return (
    <div className="space-y-4">
      <StickyToolbar
        actions={[
          { icon: <Plus className="h-4 w-4" />, label: t('bank_accounts.add', 'Add Bank Account'), onClick: openCreate, primary: true },
        ] satisfies ToolbarAction[]}
      >
        <Badge variant="secondary">{accounts.length} {t('bank_accounts.title', 'Bank Accounts')}</Badge>
      </StickyToolbar>

      <Card>
        <CardContent className="p-0">
          {isLoading ? (
            <div className="space-y-2 p-4">
              {Array.from({ length: 3 }).map((_, i) => (
                <Skeleton key={i} className="h-10 w-full" />
              ))}
            </div>
          ) : accounts.length > 0 ? (
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>Name</TableHead>
                  <TableHead>Bank</TableHead>
                  <TableHead className="hidden sm:table-cell">IBAN</TableHead>
                  <TableHead>Default</TableHead>
                  <TableHead className="w-24">Actions</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {accounts.map((a) => (
                  <TableRow key={a.id}>
                    <TableCell className="font-medium">{a.name}</TableCell>
                    <TableCell>{a.bank_name}</TableCell>
                    <TableCell className="hidden sm:table-cell font-mono text-sm">
                      {a.iban}
                    </TableCell>
                    <TableCell>
                      <div className="flex gap-1">
                        {a.is_default && <Badge variant="outline">Default</Badge>}
                        {a.qr_iban && (
                          <Badge variant="secondary" className="gap-1">
                            <QrCode className="h-3 w-3" /> QR
                          </Badge>
                        )}
                      </div>
                    </TableCell>
                    <TableCell>
                      <div className="flex gap-1">
                        <Button variant="ghost" size="icon" className="h-8 w-8" onClick={() => openEdit(a)}>
                          <Pencil className="h-3.5 w-3.5" />
                        </Button>
                        <Button variant="ghost" size="icon" className="h-8 w-8" onClick={() => handleDelete(a.id)}>
                          <Trash2 className="h-3.5 w-3.5" />
                        </Button>
                      </div>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          ) : (
            <p className="py-8 text-center text-sm text-muted-foreground">
              No bank accounts yet.
            </p>
          )}
        </CardContent>
      </Card>

      <Dialog open={dialogOpen} onOpenChange={setDialogOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>{editId ? 'Edit Bank Account' : 'New Bank Account'}</DialogTitle>
          </DialogHeader>
          <div className="space-y-4">
            <div>
              <Label>Name</Label>
              <Input value={form.name} onChange={(e) => setForm({ ...form, name: e.target.value })} placeholder="e.g. Main CHF Account" />
            </div>
            <div>
              <Label>Bank Name</Label>
              <Input value={form.bank_name} onChange={(e) => setForm({ ...form, bank_name: e.target.value })} placeholder="e.g. UBS" />
            </div>
            <div>
              <Label>IBAN</Label>
              <Input value={form.iban} onChange={(e) => setForm({ ...form, iban: e.target.value })} placeholder="CH00 0000 0000 0000 0000 0" />
            </div>
            <div>
              <Label>BIC/SWIFT</Label>
              <Input value={form.bic ?? ''} onChange={(e) => setForm({ ...form, bic: e.target.value })} placeholder="Optional" />
            </div>
            <div>
              <Label className="flex items-center gap-1.5">
                <QrCode className="h-3.5 w-3.5" /> QR-IBAN
              </Label>
              <Input
                value={form.qr_iban ?? ''}
                onChange={(e) => setForm({ ...form, qr_iban: e.target.value })}
                placeholder="CH44 3015 7..."
              />
              <p className="mt-1 text-xs text-muted-foreground">
                Optional. Obtain from your bank (IID 30000–31999). Enables QR-Referenz on payment slips.
              </p>
            </div>
            <div className="flex items-center gap-2">
              <Checkbox checked={form.is_default ?? false} onCheckedChange={(v) => setForm({ ...form, is_default: v })} />
              <Label className="mb-0">Default account</Label>
            </div>
            <Button onClick={handleSave} className="w-full" disabled={createAccount.isPending || updateAccount.isPending}>
              {editId ? 'Update' : 'Create'}
            </Button>
          </div>
        </DialogContent>
      </Dialog>
    </div>
  );
}
