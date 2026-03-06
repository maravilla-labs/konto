import { useState, useEffect } from 'react';
import { useParams, useNavigate, Link } from 'react-router-dom';
import { Skeleton } from '@/components/ui/skeleton';
import { Button } from '@/components/ui/button';
import { useInvoice, useUpdateInvoice } from '@/hooks/useApi';
import {
  InvoiceForm,
  toCreateLines,
  type InvoiceFormData,
  type LineFormData,
} from '@/components/invoice/InvoiceForm';
import { toast } from 'sonner';
import { extractErrorMessage } from '@/api/client';
import { useI18n } from '@/i18n';
import { ArrowLeft } from 'lucide-react';

export function InvoiceEditPage() {
  const { t, language } = useI18n();
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const { data, isLoading } = useInvoice(id);
  const updateInvoice = useUpdateInvoice();

  const [form, setForm] = useState<InvoiceFormData>({
    contact_id: '',
    project_id: '',
    issue_date: '',
    due_date: '',
    language,
    notes: '',
    payment_terms: '',
    header_text: '',
    footer_text: '',
    contact_person_id: '',
    default_vat_rate_id: '',
    default_account_id: '',
    bank_account_id: '',
    lines: [{ _key: crypto.randomUUID(), description: '', quantity: '1', unit_price: '', vat_rate_id: '', account_id: '', discount_percent: '' }],
  });

  useEffect(() => {
    if (data) {
      // Detect shared account/vat across lines for invoice-level defaults
      const lineVatIds = data.lines.map((l) => l.vat_rate_id).filter(Boolean);
      const lineAccountIds = data.lines.map((l) => l.account_id).filter(Boolean);
      const allSameVat = lineVatIds.length > 0 && lineVatIds.every((v) => v === lineVatIds[0]);
      const allSameAccount = lineAccountIds.length > 0 && lineAccountIds.every((a) => a === lineAccountIds[0]);

      setForm({
        contact_id: data.contact_id,
        project_id: data.project_id ?? '',
        issue_date: data.issue_date,
        due_date: data.due_date,
        language: data.language ?? language,
        notes: data.notes ?? '',
        payment_terms: data.payment_terms ?? '',
        header_text: data.header_text ?? '',
        footer_text: data.footer_text ?? '',
        contact_person_id: data.contact_person_id ?? '',
        bank_account_id: data.bank_account_id ?? '',
        default_vat_rate_id: allSameVat ? lineVatIds[0]! : '',
        default_account_id: allSameAccount ? lineAccountIds[0]! : '',
        lines: data.lines.map((l): LineFormData => ({
          _key: crypto.randomUUID(),
          description: l.description,
          quantity: l.quantity,
          unit_price: l.unit_price,
          // If all lines share the same VAT/account, clear per-line values (they'll inherit from defaults)
          vat_rate_id: allSameVat ? '' : (l.vat_rate_id ?? ''),
          account_id: allSameAccount ? '' : l.account_id,
          discount_percent: l.discount_percent ?? '',
        })),
      });
    }
  }, [data, language]);

  if (isLoading) {
    return (
      <div className="space-y-4">
        <Skeleton className="h-8 w-48" />
        <Skeleton className="h-64 w-full" />
      </div>
    );
  }

  if (!data) {
    return <p className="text-center text-muted-foreground">{t('invoices.not_found', 'Invoice not found.')}</p>;
  }

  if (data.status !== 'draft') {
    return <p className="text-center text-muted-foreground">{t('invoices.only_draft_editable', 'Only draft invoices can be edited.')}</p>;
  }

  function handleUpdate() {
    updateInvoice.mutate(
      {
        id: id!,
        data: {
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
        },
      },
      {
        onSuccess: () => {
          toast.success(t('invoices.updated', 'Invoice updated'));
          navigate(`/invoices/${id}`);
        },
        onError: (err) => toast.error(extractErrorMessage(err)),
      },
    );
  }

  return (
    <div className="space-y-4">
      <div className="flex items-center gap-3">
        <Button asChild variant="ghost" size="icon">
          <Link to={`/invoices/${id}`}><ArrowLeft className="h-4 w-4" /></Link>
        </Button>
        <div>
          <h2 className="text-lg font-semibold">
            {t('invoices.edit_invoice', 'Edit Invoice')} {data.invoice_number ?? 'DRAFT'}
          </h2>
          <p className="text-sm text-muted-foreground">{t('invoices.modify_details', 'Modify invoice details')}</p>
        </div>
      </div>
      <InvoiceForm
        form={form}
        setForm={setForm}
        onSubmit={handleUpdate}
        isPending={updateInvoice.isPending}
        submitLabel={t('common.save_changes', 'Save Changes')}
      />
    </div>
  );
}
