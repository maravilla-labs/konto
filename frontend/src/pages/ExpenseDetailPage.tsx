import { useState, useRef } from 'react';
import { useParams, useNavigate, Link } from 'react-router-dom';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Label } from '@/components/ui/label';
import { Skeleton } from '@/components/ui/skeleton';
import {
  Dialog, DialogContent, DialogHeader, DialogTitle,
} from '@/components/ui/dialog';
import {
  Select, SelectContent, SelectItem, SelectTrigger, SelectValue,
} from '@/components/ui/select';
import {
  useExpense, useApproveExpense, usePayExpense, useCancelExpense,
  useDeleteExpense, useUploadReceipt, useAccounts,
} from '@/hooks/useApi';
import { formatAmount } from '@/lib/format';
import { toast } from 'sonner';
import { extractErrorMessage } from '@/api/client';
import { resolveUploadUrl } from '@/lib/platform';
import {
  Pencil, CheckCircle, CreditCard, XCircle, Trash2, Upload,
} from 'lucide-react';

const statusVariant: Record<string, 'default' | 'secondary' | 'destructive' | 'outline'> = {
  pending: 'secondary',
  approved: 'default',
  paid: 'outline',
  cancelled: 'destructive',
};

export function ExpenseDetailPage() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const { data, isLoading } = useExpense(id);
  const approveExpense = useApproveExpense();
  const payExpense = usePayExpense();
  const cancelExpense = useCancelExpense();
  const deleteExpense = useDeleteExpense();
  const uploadReceipt = useUploadReceipt();
  const fileInputRef = useRef<HTMLInputElement>(null);

  const [payOpen, setPayOpen] = useState(false);
  const [payAccountId, setPayAccountId] = useState('');
  const { data: accountsData } = useAccounts({ per_page: 500 });
  const bankAccounts = (accountsData?.data ?? []).filter(
    (a) => a.account_type === 'asset' && a.number >= 1000 && a.number <= 1099,
  );

  if (isLoading) {
    return (
      <div className="space-y-4">
        <Skeleton className="h-8 w-48" />
        <Skeleton className="h-64 w-full" />
      </div>
    );
  }

  if (!data) {
    return <p className="text-center text-muted-foreground">Expense not found.</p>;
  }

  function handleApprove() {
    approveExpense.mutate(id!, {
      onSuccess: () => toast.success('Expense approved'),
      onError: (err) => toast.error(extractErrorMessage(err)),
    });
  }

  function handlePay() {
    payExpense.mutate(
      { id: id!, data: { payment_account_id: payAccountId } },
      {
        onSuccess: () => { toast.success('Expense marked as paid'); setPayOpen(false); },
        onError: (err) => toast.error(extractErrorMessage(err)),
      },
    );
  }

  function handleCancel() {
    cancelExpense.mutate(id!, {
      onSuccess: () => toast.success('Expense cancelled'),
      onError: (err) => toast.error(extractErrorMessage(err)),
    });
  }

  function handleDelete() {
    deleteExpense.mutate(id!, {
      onSuccess: () => { toast.success('Expense deleted'); navigate('/expenses'); },
      onError: () => toast.error('Failed to delete expense'),
    });
  }

  function handleFileChange(e: React.ChangeEvent<HTMLInputElement>) {
    const file = e.target.files?.[0];
    if (file) {
      uploadReceipt.mutate(
        { id: id!, file },
        {
          onSuccess: () => toast.success('Receipt uploaded'),
          onError: () => toast.error('Failed to upload receipt'),
        },
      );
    }
  }

  return (
    <div className="space-y-4">
      <div className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h2 className="text-lg font-semibold">
            Expense {data.expense_number ?? '—'}
          </h2>
          <p className="text-sm text-muted-foreground">
            {data.contact_name ?? 'No supplier'}
            {data.project_name && ` — ${data.project_name}`}
          </p>
        </div>
        <div className="flex flex-wrap gap-2">
          {data.status === 'pending' && (
            <>
              <Button asChild variant="outline" size="sm">
                <Link to={`/expenses/${id}/edit`}>
                  <Pencil className="mr-1 h-3.5 w-3.5" /> Edit
                </Link>
              </Button>
              <Button size="sm" onClick={handleApprove} disabled={approveExpense.isPending}>
                <CheckCircle className="mr-1 h-3.5 w-3.5" /> Approve
              </Button>
              <Button
                variant="destructive" size="sm"
                onClick={handleDelete} disabled={deleteExpense.isPending}
              >
                <Trash2 className="mr-1 h-3.5 w-3.5" /> Delete
              </Button>
              <Button
                variant="outline" size="sm"
                onClick={handleCancel} disabled={cancelExpense.isPending}
              >
                <XCircle className="mr-1 h-3.5 w-3.5" /> Cancel
              </Button>
            </>
          )}
          {data.status === 'approved' && (
            <>
              <Button size="sm" onClick={() => setPayOpen(true)}>
                <CreditCard className="mr-1 h-3.5 w-3.5" /> Mark Paid
              </Button>
              <Button
                variant="destructive" size="sm"
                onClick={handleCancel} disabled={cancelExpense.isPending}
              >
                <XCircle className="mr-1 h-3.5 w-3.5" /> Cancel
              </Button>
            </>
          )}
          <input
            ref={fileInputRef}
            type="file"
            accept="image/*,.pdf"
            className="hidden"
            onChange={handleFileChange}
          />
        </div>
      </div>

      <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
        <InfoCard label="Status">
          <Badge variant={statusVariant[data.status] ?? 'outline'}>{data.status}</Badge>
        </InfoCard>
        <InfoCard label="Category">{data.category_name ?? '—'}</InfoCard>
        <InfoCard label="Expense Date">{data.expense_date}</InfoCard>
        <InfoCard label="Total">
          <span className="font-mono font-bold">{formatAmount(data.total)}</span>
        </InfoCard>
      </div>

      <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
        <InfoCard label="Net Amount">
          <span className="font-mono">{formatAmount(data.amount)}</span>
        </InfoCard>
        <InfoCard label="VAT">
          <span className="font-mono">{formatAmount(data.vat_amount)}</span>
        </InfoCard>
        {data.due_date && <InfoCard label="Due Date">{data.due_date}</InfoCard>}
        {data.project_name && <InfoCard label="Project">{data.project_name}</InfoCard>}
      </div>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">Description</CardTitle>
        </CardHeader>
        <CardContent>
          <p className="whitespace-pre-wrap text-sm">{data.description}</p>
        </CardContent>
      </Card>

      <Card>
        <CardHeader className="flex flex-row items-center justify-between">
          <CardTitle className="text-base">Receipt</CardTitle>
          <Button
            variant="outline" size="sm"
            onClick={() => fileInputRef.current?.click()}
            disabled={uploadReceipt.isPending}
          >
            <Upload className="mr-1 h-3.5 w-3.5" />
            {data.receipt_url ? 'Replace Receipt' : 'Upload Receipt'}
          </Button>
        </CardHeader>
        <CardContent>
          {data.receipt_url ? (
            data.receipt_url.match(/\.(jpg|jpeg|png|gif|webp)$/i) ? (
              <img
                src={resolveUploadUrl(data.receipt_url) ?? ''}
                alt="Receipt"
                className="max-h-96 rounded border"
              />
            ) : (
              <a
                href={resolveUploadUrl(data.receipt_url) ?? ''}
                target="_blank"
                rel="noopener noreferrer"
                className="text-sm text-primary hover:underline"
              >
                Download Receipt
              </a>
            )
          ) : (
            <p className="text-sm text-muted-foreground">
              No receipt uploaded yet. Click "Upload Receipt" to attach one.
            </p>
          )}
        </CardContent>
      </Card>

      {(data.journal_entry_id || data.payment_journal_entry_id) && (
        <Card>
          <CardHeader>
            <CardTitle className="text-base">Linked Journal Entries</CardTitle>
          </CardHeader>
          <CardContent className="space-y-1 text-sm">
            {data.journal_entry_id && (
              <p>Approval Entry: <span className="font-mono">{data.journal_entry_id}</span></p>
            )}
            {data.payment_journal_entry_id && (
              <p>Payment Entry: <span className="font-mono">{data.payment_journal_entry_id}</span></p>
            )}
          </CardContent>
        </Card>
      )}

      <Dialog open={payOpen} onOpenChange={setPayOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Mark Expense as Paid</DialogTitle>
          </DialogHeader>
          <div className="space-y-4">
            <div>
              <Label>Payment Account</Label>
              <Select value={payAccountId} onValueChange={setPayAccountId}>
                <SelectTrigger><SelectValue placeholder="Select bank/cash account" /></SelectTrigger>
                <SelectContent>
                  {bankAccounts.map((a) => (
                    <SelectItem key={a.id} value={a.id}>
                      {a.number} — {a.name}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
            <Button
              onClick={handlePay}
              className="w-full"
              disabled={payExpense.isPending || !payAccountId}
            >
              Confirm Payment
            </Button>
          </div>
        </DialogContent>
      </Dialog>
    </div>
  );
}

function InfoCard({ label, children }: { label: string; children: React.ReactNode }) {
  return (
    <Card>
      <CardContent className="py-3">
        <p className="text-xs text-muted-foreground">{label}</p>
        <div className="mt-1">{children}</div>
      </CardContent>
    </Card>
  );
}
