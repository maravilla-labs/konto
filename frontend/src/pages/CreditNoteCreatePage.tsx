import { useState, useEffect } from 'react';
import { useNavigate, useSearchParams } from 'react-router-dom';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { useCreateCreditNote, useIssueCreditNote, useInvoice } from '@/hooks/useApi';
import {
  CreditNoteForm,
  emptyLine,
  toCreateLines,
  type CreditNoteFormData,
  type LineFormData,
} from '@/components/credit-note/CreditNoteForm';
import { toast } from 'sonner';

function today(): string {
  return new Date().toISOString().split('T')[0];
}

export function CreditNoteCreatePage() {
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const invoiceId = searchParams.get('invoice_id');

  const createCreditNote = useCreateCreditNote();
  const issueCreditNote = useIssueCreditNote();
  const { data: invoiceData } = useInvoice(invoiceId ?? undefined);

  const [form, setForm] = useState<CreditNoteFormData>({
    contact_id: '',
    invoice_id: '',
    issue_date: today(),
    notes: '',
    lines: [emptyLine()],
  });

  // Pre-fill from invoice if invoice_id is in URL
  useEffect(() => {
    if (invoiceData) {
      setForm({
        contact_id: invoiceData.contact_id,
        invoice_id: invoiceData.id,
        issue_date: today(),
        notes: `Credit for invoice ${invoiceData.invoice_number ?? invoiceData.id}`,
        lines: invoiceData.lines.map((l): LineFormData => ({
          description: l.description,
          quantity: l.quantity,
          unit_price: l.unit_price,
          vat_rate_id: l.vat_rate_id ?? '',
          account_id: l.account_id,
        })),
      });
    }
  }, [invoiceData]);

  function handleSaveDraft() {
    createCreditNote.mutate(
      {
        contact_id: form.contact_id,
        invoice_id: form.invoice_id || undefined,
        issue_date: form.issue_date,
        notes: form.notes || undefined,
        lines: toCreateLines(form.lines),
      },
      {
        onSuccess: (res) => {
          toast.success('Credit note saved as draft');
          navigate(`/credit-notes/${res.data.id}`);
        },
        onError: () => toast.error('Failed to create credit note'),
      },
    );
  }

  function handleSaveAndIssue() {
    createCreditNote.mutate(
      {
        contact_id: form.contact_id,
        invoice_id: form.invoice_id || undefined,
        issue_date: form.issue_date,
        notes: form.notes || undefined,
        lines: toCreateLines(form.lines),
      },
      {
        onSuccess: (res) => {
          issueCreditNote.mutate(res.data.id, {
            onSuccess: () => {
              toast.success('Credit note created and issued');
              navigate(`/credit-notes/${res.data.id}`);
            },
            onError: () => {
              toast.error('Credit note created but failed to issue');
              navigate(`/credit-notes/${res.data.id}`);
            },
          });
        },
        onError: () => toast.error('Failed to create credit note'),
      },
    );
  }

  return (
    <div className="space-y-4">
      <div>
        <h2 className="text-lg font-semibold">New Credit Note</h2>
        <p className="text-sm text-muted-foreground">Create a new credit note / Gutschrift</p>
      </div>
      <Card>
        <CardHeader>
          <CardTitle className="text-base">Credit Note Details</CardTitle>
        </CardHeader>
        <CardContent>
          <CreditNoteForm
            form={form}
            setForm={setForm}
            onSubmit={handleSaveDraft}
            isPending={createCreditNote.isPending}
            submitLabel="Save as Draft"
            secondaryAction={{
              label: 'Save & Issue',
              onClick: handleSaveAndIssue,
              isPending: createCreditNote.isPending || issueCreditNote.isPending,
            }}
          />
        </CardContent>
      </Card>
    </div>
  );
}
