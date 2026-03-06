import { useState } from 'react';
import { Button } from '@/components/ui/button';
import { Card, CardContent } from '@/components/ui/card';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table';
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogFooter } from '@/components/ui/dialog';
import { Label } from '@/components/ui/label';
import { StickyToolbar } from '@/components/ui/sticky-toolbar';
import { Check, X, FileText } from 'lucide-react';
import { toast } from 'sonner';
import { useI18n } from '@/i18n';
import { useBankTransactions, useManualMatch, useIgnoreTransaction, useCreateJournalFromTx } from '@/hooks/useBankingApi';
import { useBankAccounts } from '@/hooks/useSettingsApi';
import type { BankTransaction } from '@/types/bank-transaction';

export function ReconciliationPage() {
  const { t } = useI18n();
  const [bankAccountId, setBankAccountId] = useState<string>('');
  const [selected, setSelected] = useState<BankTransaction | null>(null);
  const [matchDialog, setMatchDialog] = useState(false);
  const [journalDialog, setJournalDialog] = useState(false);
  const [targetType, setTargetType] = useState<'invoice' | 'expense'>('invoice');
  const [targetId, setTargetId] = useState('');
  const [debitAccId, setDebitAccId] = useState('');
  const [creditAccId, setCreditAccId] = useState('');

  const { data: bankAccounts } = useBankAccounts();
  const { data: txData, isLoading } = useBankTransactions({
    bank_account_id: bankAccountId || undefined,
    status: 'unmatched',
    per_page: 100,
  });
  const matchMut = useManualMatch();
  const ignoreMut = useIgnoreTransaction();
  const journalMut = useCreateJournalFromTx();

  const transactions = txData?.data ?? [];

  const handleMatch = async () => {
    if (!selected || !targetId) return;
    try {
      await matchMut.mutateAsync({ id: selected.id, data: { target_type: targetType, target_id: targetId } });
      toast.success('Transaction matched');
      setMatchDialog(false);
      setSelected(null);
      setTargetId('');
    } catch {
      toast.error('Match failed');
    }
  };

  const handleIgnore = async (tx: BankTransaction) => {
    try {
      await ignoreMut.mutateAsync(tx.id);
      toast.success('Transaction ignored');
    } catch {
      toast.error('Failed to ignore');
    }
  };

  const handleCreateJournal = async () => {
    if (!selected || !debitAccId || !creditAccId) return;
    try {
      await journalMut.mutateAsync({ id: selected.id, data: { debit_account_id: debitAccId, credit_account_id: creditAccId } });
      toast.success('Journal entry created');
      setJournalDialog(false);
      setSelected(null);
    } catch {
      toast.error('Failed to create journal entry');
    }
  };

  return (
    <div className="space-y-4">
      <StickyToolbar>
        <Select value={bankAccountId} onValueChange={setBankAccountId}>
          <SelectTrigger className="h-8 w-72 text-sm">
            <SelectValue placeholder={t('reconciliation.select_bank_account', 'Select bank account')} />
          </SelectTrigger>
          <SelectContent>
            {(bankAccounts ?? []).map((a: { id: string; name: string; iban: string }) => (
              <SelectItem key={a.id} value={a.id}>{a.name} ({a.iban})</SelectItem>
            ))}
          </SelectContent>
        </Select>
        {transactions.length > 0 && (
          <span className="text-xs text-muted-foreground">
            {transactions.length} {t('reconciliation.unmatched', 'unmatched')}
          </span>
        )}
      </StickyToolbar>

      <Card>
        <CardContent className="p-0">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Date</TableHead>
                <TableHead>Description</TableHead>
                <TableHead>Counterparty</TableHead>
                <TableHead className="text-right">Amount</TableHead>
                <TableHead>Actions</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {isLoading ? (
                <TableRow><TableCell colSpan={5} className="text-center py-8">Loading...</TableCell></TableRow>
              ) : transactions.length === 0 ? (
                <TableRow><TableCell colSpan={5} className="text-center py-8 text-muted-foreground">
                  {bankAccountId ? 'All transactions are matched!' : 'Select a bank account'}
                </TableCell></TableRow>
              ) : transactions.map((tx) => (
                <TableRow key={tx.id}>
                  <TableCell className="whitespace-nowrap">{tx.transaction_date}</TableCell>
                  <TableCell className="max-w-[200px] truncate">{tx.description}</TableCell>
                  <TableCell>{tx.counterparty_name ?? '-'}</TableCell>
                  <TableCell className={`text-right font-mono ${parseFloat(tx.amount) >= 0 ? 'text-green-600' : 'text-red-600'}`}>
                    {parseFloat(tx.amount).toLocaleString('de-CH', { minimumFractionDigits: 2 })}
                  </TableCell>
                  <TableCell>
                    <div className="flex gap-1">
                      <Button size="sm" variant="outline" onClick={() => { setSelected(tx); setMatchDialog(true); }}>
                        <Check className="h-3 w-3 mr-1" /> Match
                      </Button>
                      <Button size="sm" variant="outline" onClick={() => { setSelected(tx); setJournalDialog(true); }}>
                        <FileText className="h-3 w-3 mr-1" /> Journal
                      </Button>
                      <Button size="sm" variant="ghost" onClick={() => handleIgnore(tx)}>
                        <X className="h-3 w-3" />
                      </Button>
                    </div>
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </CardContent>
      </Card>

      {/* Manual Match Dialog */}
      <Dialog open={matchDialog} onOpenChange={setMatchDialog}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Match Transaction</DialogTitle>
          </DialogHeader>
          {selected && (
            <div className="space-y-4">
              <div className="rounded-lg border p-3 text-sm">
                <p><strong>Date:</strong> {selected.transaction_date}</p>
                <p><strong>Amount:</strong> {selected.amount} {selected.currency}</p>
                <p><strong>Description:</strong> {selected.description}</p>
              </div>
              <div className="space-y-2">
                <Label>Match to</Label>
                <Select value={targetType} onValueChange={(v) => setTargetType(v as 'invoice' | 'expense')}>
                  <SelectTrigger><SelectValue /></SelectTrigger>
                  <SelectContent>
                    <SelectItem value="invoice">Invoice</SelectItem>
                    <SelectItem value="expense">Expense</SelectItem>
                  </SelectContent>
                </Select>
              </div>
              <div className="space-y-2">
                <Label>{targetType === 'invoice' ? 'Invoice' : 'Expense'} ID</Label>
                <input
                  className="w-full rounded-md border px-3 py-2 text-sm"
                  value={targetId}
                  onChange={(e) => setTargetId(e.target.value)}
                  placeholder={`Enter ${targetType} ID`}
                />
              </div>
            </div>
          )}
          <DialogFooter>
            <Button variant="outline" onClick={() => setMatchDialog(false)}>Cancel</Button>
            <Button onClick={handleMatch} disabled={!targetId || matchMut.isPending}>Match</Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Create Journal Dialog */}
      <Dialog open={journalDialog} onOpenChange={setJournalDialog}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Create Journal Entry</DialogTitle>
          </DialogHeader>
          {selected && (
            <div className="space-y-4">
              <div className="rounded-lg border p-3 text-sm">
                <p><strong>Amount:</strong> {selected.amount} {selected.currency}</p>
                <p><strong>Description:</strong> {selected.description}</p>
              </div>
              <div className="space-y-2">
                <Label>Debit Account ID</Label>
                <input
                  className="w-full rounded-md border px-3 py-2 text-sm"
                  value={debitAccId}
                  onChange={(e) => setDebitAccId(e.target.value)}
                  placeholder="Debit account ID"
                />
              </div>
              <div className="space-y-2">
                <Label>Credit Account ID</Label>
                <input
                  className="w-full rounded-md border px-3 py-2 text-sm"
                  value={creditAccId}
                  onChange={(e) => setCreditAccId(e.target.value)}
                  placeholder="Credit account ID"
                />
              </div>
            </div>
          )}
          <DialogFooter>
            <Button variant="outline" onClick={() => setJournalDialog(false)}>Cancel</Button>
            <Button onClick={handleCreateJournal} disabled={!debitAccId || !creditAccId || journalMut.isPending}>
              Create Entry
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}
