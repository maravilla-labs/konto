import { useState } from 'react';
import { useNavigate, useSearchParams, Link } from 'react-router-dom';
import { useCreateInvoice, useSendInvoice } from '@/hooks/useApi';
import {
  InvoiceForm,
  toCreateLines,
  type InvoiceFormData,
} from '@/components/invoice/InvoiceForm';
import { emptyLine } from '@/components/invoice/InvoiceFormTypes';
import { Button } from '@/components/ui/button';
import { toast } from 'sonner';
import { extractErrorMessage } from '@/api/client';
import { useI18n } from '@/i18n';
import { ArrowLeft } from 'lucide-react';

function today(): string {
  return new Date().toISOString().split('T')[0];
}

function in30Days(): string {
  const d = new Date();
  d.setDate(d.getDate() + 30);
  return d.toISOString().split('T')[0];
}

export function InvoiceCreatePage() {
  const { t, language } = useI18n();
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const createInvoice = useCreateInvoice();
  const sendInvoice = useSendInvoice();

  const [form, setForm] = useState<InvoiceFormData>({
    contact_id: searchParams.get('contact_id') ?? '',
    project_id: searchParams.get('project_id') ?? '',
    issue_date: today(),
    due_date: in30Days(),
    language,
    notes: '',
    payment_terms: 'Net 30',
    header_text: '',
    footer_text: '',
    contact_person_id: '',
    default_vat_rate_id: '',
    default_account_id: '',
    bank_account_id: '',
    lines: [emptyLine()],
  });

  function buildPayload() {
    return {
      contact_id: form.contact_id,
      project_id: form.project_id || undefined,
      issue_date: form.issue_date,
      due_date: form.due_date,
      language: form.language || undefined,
      notes: form.notes || undefined,
      payment_terms: form.payment_terms || undefined,
      header_text: form.header_text || undefined,
      footer_text: form.footer_text || undefined,
      contact_person_id: form.contact_person_id || undefined,
      bank_account_id: form.bank_account_id || undefined,
      lines: toCreateLines(form.lines, form.default_vat_rate_id, form.default_account_id),
    };
  }

  function handleSaveDraft() {
    createInvoice.mutate(buildPayload(), {
      onSuccess: (res) => {
        toast.success(t('invoice_form.saved_as_draft', 'Invoice saved as draft'));
        navigate(`/invoices/${res.data.id}`);
      },
      onError: (err) => toast.error(extractErrorMessage(err)),
    });
  }

  function handleSaveAndSend() {
    createInvoice.mutate(buildPayload(), {
      onSuccess: (res) => {
        sendInvoice.mutate(res.data.id, {
          onSuccess: () => {
            toast.success(t('invoice_form.created_and_sent', 'Invoice created and sent'));
            navigate(`/invoices/${res.data.id}`);
          },
          onError: () => {
            toast.error(t('invoice_form.created_but_send_failed', 'Invoice created but failed to send'));
            navigate(`/invoices/${res.data.id}`);
          },
        });
      },
      onError: (err) => toast.error(extractErrorMessage(err)),
    });
  }

  return (
    <div className="space-y-4">
      <div className="flex items-center gap-3">
        <Button asChild variant="ghost" size="icon">
          <Link to="/invoices"><ArrowLeft className="h-4 w-4" /></Link>
        </Button>
        <div>
          <h2 className="text-lg font-semibold">{t('invoices.new_invoice', 'New Invoice')}</h2>
          <p className="text-sm text-muted-foreground">{t('invoices.create_description', 'Create a new customer invoice')}</p>
        </div>
      </div>
      <InvoiceForm
        form={form}
        setForm={setForm}
        onSubmit={handleSaveDraft}
        isPending={createInvoice.isPending}
        submitLabel={t('invoice_form.save_draft', 'Save as Draft')}
        secondaryAction={{
          label: t('invoice_form.save_and_send', 'Save & Send'),
          onClick: handleSaveAndSend,
          isPending: createInvoice.isPending || sendInvoice.isPending,
        }}
      />
    </div>
  );
}
