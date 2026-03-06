import { useParams, useNavigate, Link } from 'react-router-dom';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Skeleton } from '@/components/ui/skeleton';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import {
  useCreditNote,
  useIssueCreditNote,
  useApplyCreditNote,
  useCancelCreditNote,
  useDeleteCreditNote,
} from '@/hooks/useApi';
import { formatAmount } from '@/lib/format';
import { toast } from 'sonner';
import { creditNotesApi } from '@/api/credit-notes';
import { Pencil, CheckCircle, XCircle, Trash2, Download, Stamp } from 'lucide-react';

const statusVariant: Record<string, 'default' | 'secondary' | 'destructive' | 'outline'> = {
  draft: 'secondary',
  issued: 'default',
  applied: 'outline',
  cancelled: 'destructive',
};

export function CreditNoteDetailPage() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const { data, isLoading } = useCreditNote(id);
  const issueCreditNote = useIssueCreditNote();
  const applyCreditNote = useApplyCreditNote();
  const cancelCreditNote = useCancelCreditNote();
  const deleteCreditNote = useDeleteCreditNote();

  if (isLoading) {
    return (
      <div className="space-y-4">
        <Skeleton className="h-8 w-48" />
        <Skeleton className="h-64 w-full" />
      </div>
    );
  }

  if (!data) {
    return <p className="text-center text-muted-foreground">Credit note not found.</p>;
  }

  function handleIssue() {
    issueCreditNote.mutate(id!, {
      onSuccess: () => toast.success('Credit note issued'),
      onError: () => toast.error('Failed to issue credit note'),
    });
  }

  function handleApply() {
    applyCreditNote.mutate(id!, {
      onSuccess: () => toast.success('Credit note applied'),
      onError: () => toast.error('Failed to apply credit note'),
    });
  }

  function handleCancel() {
    cancelCreditNote.mutate(id!, {
      onSuccess: () => toast.success('Credit note cancelled'),
      onError: () => toast.error('Failed to cancel credit note'),
    });
  }

  function handleDelete() {
    deleteCreditNote.mutate(id!, {
      onSuccess: () => { toast.success('Credit note deleted'); navigate('/credit-notes'); },
      onError: () => toast.error('Failed to delete credit note'),
    });
  }

  async function handleDownloadPdf() {
    try {
      const res = await creditNotesApi.downloadPdf(id!);
      const blob = new Blob([res.data], { type: 'application/pdf' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `credit-note-${data?.credit_note_number ?? 'draft'}.pdf`;
      a.click();
      URL.revokeObjectURL(url);
    } catch {
      toast.error('Failed to download PDF');
    }
  }

  return (
    <div className="space-y-4">
      <div className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h2 className="text-lg font-semibold">
            Credit Note {data.credit_note_number ?? 'DRAFT'}
          </h2>
          <p className="text-sm text-muted-foreground">
            {data.contact_name ?? data.contact_id}
          </p>
        </div>
        <div className="flex flex-wrap gap-2">
          {data.status !== 'draft' && (
            <Button variant="outline" size="sm" onClick={handleDownloadPdf}>
              <Download className="mr-1 h-3.5 w-3.5" /> Download PDF
            </Button>
          )}
          {data.status === 'draft' && (
            <>
              <Button asChild variant="outline" size="sm">
                <Link to={`/credit-notes/${id}/edit`}>
                  <Pencil className="mr-1 h-3.5 w-3.5" /> Edit
                </Link>
              </Button>
              <Button size="sm" onClick={handleIssue} disabled={issueCreditNote.isPending}>
                <Stamp className="mr-1 h-3.5 w-3.5" /> Issue
              </Button>
              <Button
                variant="destructive"
                size="sm"
                onClick={handleDelete}
                disabled={deleteCreditNote.isPending}
              >
                <Trash2 className="mr-1 h-3.5 w-3.5" /> Delete
              </Button>
            </>
          )}
          {data.status === 'issued' && (
            <>
              <Button size="sm" onClick={handleApply} disabled={applyCreditNote.isPending}>
                <CheckCircle className="mr-1 h-3.5 w-3.5" /> Mark Applied
              </Button>
              <Button
                variant="destructive"
                size="sm"
                onClick={handleCancel}
                disabled={cancelCreditNote.isPending}
              >
                <XCircle className="mr-1 h-3.5 w-3.5" /> Cancel
              </Button>
            </>
          )}
        </div>
      </div>

      <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
        <InfoCard label="Status">
          <Badge variant={statusVariant[data.status] ?? 'outline'}>{data.status}</Badge>
        </InfoCard>
        <InfoCard label="Issue Date">{data.issue_date}</InfoCard>
        <InfoCard label="Total">
          <span className="font-mono font-bold">{formatAmount(data.total)}</span>
        </InfoCard>
        {data.invoice_number && (
          <InfoCard label="Linked Invoice">
            <Link
              to={`/invoices/${data.invoice_id}`}
              className="font-mono text-sm text-primary hover:underline"
            >
              {data.invoice_number}
            </Link>
          </InfoCard>
        )}
      </div>

      {data.notes && (
        <Card>
          <CardContent className="py-4">
            <p className="mt-1 text-sm text-muted-foreground">{data.notes}</p>
          </CardContent>
        </Card>
      )}

      <Card>
        <CardHeader>
          <CardTitle className="text-base">Line Items</CardTitle>
        </CardHeader>
        <CardContent className="p-0">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead className="w-12">#</TableHead>
                <TableHead>Description</TableHead>
                <TableHead className="text-right">Qty</TableHead>
                <TableHead className="text-right">Unit Price</TableHead>
                <TableHead className="text-right">VAT</TableHead>
                <TableHead className="text-right">Total</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {data.lines.map((line) => (
                <TableRow key={line.id}>
                  <TableCell className="text-muted-foreground">{line.sort_order}</TableCell>
                  <TableCell>{line.description}</TableCell>
                  <TableCell className="text-right font-mono text-sm">
                    {formatAmount(line.quantity)}
                  </TableCell>
                  <TableCell className="text-right font-mono text-sm">
                    {formatAmount(line.unit_price)}
                  </TableCell>
                  <TableCell className="text-right font-mono text-sm">
                    {formatAmount(line.vat_amount)}
                  </TableCell>
                  <TableCell className="text-right font-mono text-sm font-medium">
                    {formatAmount(line.line_total)}
                  </TableCell>
                </TableRow>
              ))}
              <TableRow className="bg-muted/50">
                <TableCell colSpan={5} className="text-right font-medium">
                  Subtotal
                </TableCell>
                <TableCell className="text-right font-mono text-sm font-medium">
                  {formatAmount(data.subtotal)}
                </TableCell>
              </TableRow>
              <TableRow className="bg-muted/50">
                <TableCell colSpan={5} className="text-right font-medium">
                  VAT
                </TableCell>
                <TableCell className="text-right font-mono text-sm font-medium">
                  {formatAmount(data.vat_amount)}
                </TableCell>
              </TableRow>
              <TableRow className="bg-muted/50 font-bold">
                <TableCell colSpan={5} className="text-right">Total</TableCell>
                <TableCell className="text-right font-mono text-sm">
                  {formatAmount(data.total)}
                </TableCell>
              </TableRow>
            </TableBody>
          </Table>
        </CardContent>
      </Card>

      {data.journal_entry_id && (
        <Card>
          <CardHeader>
            <CardTitle className="text-base">Linked Journal Entry</CardTitle>
          </CardHeader>
          <CardContent className="text-sm">
            <p>Journal Entry: <span className="font-mono">{data.journal_entry_id}</span></p>
          </CardContent>
        </Card>
      )}
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
