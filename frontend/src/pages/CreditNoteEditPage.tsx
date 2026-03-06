import { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Skeleton } from '@/components/ui/skeleton';
import { useCreditNote, useUpdateCreditNote } from '@/hooks/useApi';
import {
  CreditNoteForm,
  toCreateLines,
  type CreditNoteFormData,
  type LineFormData,
} from '@/components/credit-note/CreditNoteForm';
import { toast } from 'sonner';

export function CreditNoteEditPage() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const { data, isLoading } = useCreditNote(id);
  const updateCreditNote = useUpdateCreditNote();

  const [form, setForm] = useState<CreditNoteFormData>({
    contact_id: '',
    invoice_id: '',
    issue_date: '',
    notes: '',
    lines: [{ description: '', quantity: '1', unit_price: '', vat_rate_id: '', account_id: '' }],
  });

  useEffect(() => {
    if (data) {
      setForm({
        contact_id: data.contact_id,
        invoice_id: data.invoice_id ?? '',
        issue_date: data.issue_date,
        notes: data.notes ?? '',
        lines: data.lines.map((l): LineFormData => ({
          description: l.description,
          quantity: l.quantity,
          unit_price: l.unit_price,
          vat_rate_id: l.vat_rate_id ?? '',
          account_id: l.account_id,
        })),
      });
    }
  }, [data]);

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

  if (data.status !== 'draft') {
    return <p className="text-center text-muted-foreground">Only draft credit notes can be edited.</p>;
  }

  function handleUpdate() {
    updateCreditNote.mutate(
      {
        id: id!,
        data: {
          contact_id: form.contact_id,
          invoice_id: form.invoice_id || undefined,
          issue_date: form.issue_date,
          notes: form.notes || undefined,
          lines: toCreateLines(form.lines),
        },
      },
      {
        onSuccess: () => {
          toast.success('Credit note updated');
          navigate(`/credit-notes/${id}`);
        },
        onError: () => toast.error('Failed to update credit note'),
      },
    );
  }

  return (
    <div className="space-y-4">
      <div>
        <h2 className="text-lg font-semibold">
          Edit Credit Note {data.credit_note_number ?? 'DRAFT'}
        </h2>
        <p className="text-sm text-muted-foreground">Modify credit note details</p>
      </div>
      <Card>
        <CardHeader>
          <CardTitle className="text-base">Credit Note Details</CardTitle>
        </CardHeader>
        <CardContent>
          <CreditNoteForm
            form={form}
            setForm={setForm}
            onSubmit={handleUpdate}
            isPending={updateCreditNote.isPending}
            submitLabel="Save Changes"
          />
        </CardContent>
      </Card>
    </div>
  );
}
