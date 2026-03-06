import { useState } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Skeleton } from '@/components/ui/skeleton';
import { Progress } from '@/components/ui/progress';
import { MarkdownPreview } from '@/components/ui/markdown-preview';
import { StickyToolbar } from '@/components/ui/sticky-toolbar';
import type { ToolbarAction, ToolbarOverflowItem } from '@/components/ui/sticky-toolbar';
import {
  Table, TableBody, TableCell, TableHead, TableHeader, TableRow,
} from '@/components/ui/table';
import {
  useInvoice, useSendInvoice, usePayInvoice, useCancelInvoice,
  useDeleteInvoice, useAccounts, useInvoiceDunning, useDunningLevels,
  useSendReminder, useInvoicePayments, useRecordPayment,
} from '@/hooks/useApi';
import { formatAmount } from '@/lib/format';
import { toast } from 'sonner';
import { extractErrorMessage } from '@/api/client';
import { invoicesApi } from '@/api/invoices';
import { useEmailInvoice } from '@/hooks/useEmailApi';
import { ReminderDialog, MarkPaidDialog, RecordPaymentDialog } from '@/components/invoices/InvoiceDialogs';
import { PdfPreviewDialog } from '@/components/invoice/PdfPreviewDialog';
import { saveFile } from '@/lib/native';
import { useI18n } from '@/i18n';
import { useSettings } from '@/hooks/useSettingsApi';
import { formatDate } from '@/lib/locale';
import {
  Pencil, Send, CreditCard, XCircle, Trash2, Download, Mail,
  ReceiptText, Bell, Plus, Copy, Eye,
} from 'lucide-react';

const statusVariant: Record<string, 'default' | 'secondary' | 'destructive' | 'outline'> = {
  draft: 'secondary', sent: 'default', partial: 'default',
  paid: 'outline', overdue: 'destructive', cancelled: 'destructive',
};

export function InvoiceDetailPage() {
  const { t } = useI18n();
  const { data: settings } = useSettings();
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const { data, isLoading } = useInvoice(id);
  const sendInvoice = useSendInvoice();
  const payInvoice = usePayInvoice();
  const cancelInvoice = useCancelInvoice();
  const deleteInvoice = useDeleteInvoice();
  const emailInvoice = useEmailInvoice();
  const { data: dunningHistory } = useInvoiceDunning(id);
  const { data: dunningLevels } = useDunningLevels();
  const sendReminder = useSendReminder();
  const { data: payments } = useInvoicePayments(id);
  const recordPayment = useRecordPayment();

  const [reminderOpen, setReminderOpen] = useState(false);
  const [reminderLevelId, setReminderLevelId] = useState('');
  const [payOpen, setPayOpen] = useState(false);
  const [payDate, setPayDate] = useState(new Date().toISOString().split('T')[0]);
  const [payAccountId, setPayAccountId] = useState('');
  const [recordOpen, setRecordOpen] = useState(false);
  const [recordForm, setRecordForm] = useState({
    amount: '', payment_date: new Date().toISOString().split('T')[0],
    payment_account_id: '', payment_method: '', reference: '',
  });
  const [pdfOpen, setPdfOpen] = useState(false);
  const [savingPdf, setSavingPdf] = useState(false);

  const { data: accountsData } = useAccounts({ per_page: 500 });
  const bankAccounts = (accountsData?.data ?? []).filter(
    (a) => a.account_type === 'asset' && a.number >= 1000 && a.number <= 1099,
  );

  const totalPaid = (payments ?? []).reduce((sum, p) => sum + parseFloat(p.amount), 0);
  const invoiceTotal = data ? parseFloat(data.total) : 0;
  const remaining = Math.max(0, invoiceTotal - totalPaid);
  const paidPercent = invoiceTotal > 0 ? Math.min(100, (totalPaid / invoiceTotal) * 100) : 0;
  const dateFormat = settings?.date_format ?? 'dd.MM.yyyy';

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

  const status = data.status;

  // --- Handlers ---

  function handleSend() {
    sendInvoice.mutate(id!, {
      onSuccess: () => toast.success(t('invoices.sent', 'Invoice sent')),
      onError: (err) => toast.error(extractErrorMessage(err)),
    });
  }

  function handlePay() {
    payInvoice.mutate(
      { id: id!, data: { payment_date: payDate, payment_account_id: payAccountId } },
      {
        onSuccess: () => { toast.success(t('invoices.marked_paid', 'Invoice marked as paid')); setPayOpen(false); },
        onError: (err) => toast.error(extractErrorMessage(err)),
      },
    );
  }

  function handleCancel() {
    cancelInvoice.mutate(id!, {
      onSuccess: () => toast.success(t('invoices.cancelled_success', 'Invoice cancelled')),
      onError: (err) => toast.error(extractErrorMessage(err)),
    });
  }

  function handleSendReminder() {
    if (!reminderLevelId) return;
    sendReminder.mutate(
      { invoiceId: id!, data: { dunning_level_id: reminderLevelId, send_email: true } },
      {
        onSuccess: () => { toast.success(t('invoice_dialogs.reminder.sent', 'Reminder sent')); setReminderOpen(false); },
        onError: () => toast.error(t('invoice_dialogs.reminder.failed', 'Failed to send reminder')),
      },
    );
  }

  function handleDelete() {
    deleteInvoice.mutate(id!, {
      onSuccess: () => { toast.success(t('invoices.deleted', 'Invoice deleted')); navigate('/invoices'); },
      onError: () => toast.error(t('common.delete_failed', 'Failed to delete')),
    });
  }

  function handleRecordPayment() {
    const amount = parseFloat(recordForm.amount);
    if (!amount || amount <= 0 || !recordForm.payment_account_id) {
      toast.error(t('invoice_dialogs.amount_and_account_required', 'Amount and payment account are required'));
      return;
    }
    recordPayment.mutate(
      {
        invoiceId: id!,
        data: {
          amount,
          payment_date: recordForm.payment_date,
          payment_account_id: recordForm.payment_account_id,
          payment_method: recordForm.payment_method || undefined,
          reference: recordForm.reference || undefined,
        },
      },
      {
        onSuccess: () => {
          toast.success(t('invoice_dialogs.recorded', 'Payment recorded'));
          setRecordOpen(false);
          setRecordForm({ amount: '', payment_date: new Date().toISOString().split('T')[0], payment_account_id: '', payment_method: '', reference: '' });
        },
        onError: (err) => toast.error(extractErrorMessage(err)),
      },
    );
  }

  async function handleDuplicate() {
    try {
      const res = await invoicesApi.duplicate(id!);
      toast.success(t('invoices.duplicated', 'Invoice duplicated'));
      navigate(`/invoices/${res.data.id}`);
    } catch {
      toast.error(t('invoices.duplicate_failed', 'Failed to duplicate invoice'));
    }
  }

  async function handleSavePdf() {
    setSavingPdf(true);
    try {
      const res = await invoicesApi.downloadPdf(id!);
      const blob = new Blob([res.data], { type: 'application/pdf' });
      const filename = `invoice-${data?.invoice_number ?? 'draft'}.pdf`;
      await saveFile(blob, filename);
    } catch {
      toast.error(t('invoices.download_pdf_failed', 'Failed to download PDF'));
    } finally {
      setSavingPdf(false);
    }
  }

  function onEmail() {
    emailInvoice.mutate(id!, {
      onSuccess: () => toast.success(t('invoices.emailed', 'Invoice emailed')),
      onError: () => toast.error(t('invoices.email_failed', 'Failed to email invoice')),
    });
  }

  // --- Build toolbar actions ---

  const actions: ToolbarAction[] = [
    {
      icon: <Eye className="h-4 w-4" />,
      label: t('invoices.preview', 'Preview'),
      onClick: () => setPdfOpen(true),
    },
    {
      icon: <Download className="h-4 w-4" />,
      label: t('common.save', 'Save'),
      onClick: handleSavePdf,
      loading: savingPdf,
    },
  ];

  if (status === 'draft') {
    actions.push({
      icon: <Pencil className="h-4 w-4" />,
      label: t('invoices.edit', 'Edit'),
      onClick: () => navigate(`/invoices/${id}/edit`),
    });
    actions.push({
      icon: <Send className="h-4 w-4" />,
      label: t('status.sent', 'Send'),
      onClick: handleSend,
      primary: true,
      loading: sendInvoice.isPending,
    });
  }

  if (status === 'sent' || status === 'overdue' || status === 'partial') {
    actions.push({
      icon: <CreditCard className="h-4 w-4" />,
      label: t('invoices.mark_fully_paid', 'Mark Paid'),
      onClick: () => setPayOpen(true),
      primary: true,
    });
    actions.push({
      icon: <Plus className="h-4 w-4" />,
      label: t('invoice_dialogs.record.action', 'Record Payment'),
      onClick: () => {
        setRecordForm({ ...recordForm, amount: remaining.toFixed(2) });
        setRecordOpen(true);
      },
    });
  }

  // --- Build overflow items ---

  const overflow: ToolbarOverflowItem[] = [
    {
      icon: <Mail className="h-4 w-4" />,
      label: t('common.email', 'Email'),
      onClick: onEmail,
    },
    {
      icon: <Copy className="h-4 w-4" />,
      label: t('invoices.duplicate', 'Duplicate'),
      onClick: handleDuplicate,
    },
  ];

  if (status === 'sent' || status === 'overdue') {
    overflow.push({
      icon: <Bell className="h-4 w-4" />,
      label: t('invoice_dialogs.reminder.send', 'Send Reminder'),
      onClick: () => setReminderOpen(true),
    });
  }

  if (status === 'sent' || status === 'paid' || status === 'overdue') {
    overflow.push({
      icon: <ReceiptText className="h-4 w-4" />,
      label: t('nav.credit-notes', 'Credit Note'),
      onClick: () => navigate(`/credit-notes/new?invoice_id=${id}`),
    });
  }

  if (status === 'sent' || status === 'overdue' || status === 'partial') {
    overflow.push({
      icon: <XCircle className="h-4 w-4" />,
      label: t('common.cancel', 'Cancel Invoice'),
      onClick: handleCancel,
      destructive: true,
      separator: true,
    });
  }

  if (status === 'draft') {
    overflow.push({
      icon: <Trash2 className="h-4 w-4" />,
      label: t('common.delete', 'Delete'),
      onClick: handleDelete,
      destructive: true,
      separator: true,
    });
  }

  return (
    <div className="space-y-6">
      {/* Sticky toolbar */}
      <StickyToolbar actions={actions} overflow={overflow}>
        <h2 className="text-2xl font-bold truncate">
          {data.invoice_number ?? t('invoices.draft_label', 'DRAFT')}
        </h2>
        <Badge variant={statusVariant[status] ?? 'outline'}>
          {t(`status.${status}`, status)}
        </Badge>
      </StickyToolbar>

      {/* Subtitle */}
      <p className="text-sm text-muted-foreground -mt-3">
        {data.contact_name ?? data.contact_id}
        {data.project_name && ` — ${data.project_name}`}
      </p>

      {/* Metrics row */}
      <div className="grid gap-4 grid-cols-2 lg:grid-cols-4">
        <InfoCard label={t('common.issue_date', 'Issue Date')}>{formatDate(data.issue_date, dateFormat)}</InfoCard>
        <InfoCard label={t('common.due_date', 'Due Date')}>{formatDate(data.due_date, dateFormat)}</InfoCard>
        <InfoCard label={t('invoice_form.subtotal', 'Subtotal')}>
          <span className="font-mono">{formatAmount(data.subtotal)}</span>
        </InfoCard>
        <InfoCard label={t('common.total', 'Total')}>
          <span className="font-mono font-bold">{formatAmount(data.total)}</span>
        </InfoCard>
      </div>

      {/* Payment progress */}
      {status !== 'draft' && (payments ?? []).length > 0 && (
        <Card>
          <CardHeader><CardTitle className="text-base">{t('invoices.payment_progress', 'Payment Progress')}</CardTitle></CardHeader>
          <CardContent className="space-y-3">
            <div className="flex items-center justify-between text-sm">
              <span>{t('status.paid', 'Paid')}: <span className="font-mono font-medium">{formatAmount(totalPaid.toFixed(2))}</span></span>
              <span>{t('invoice_dialogs.remaining', 'Remaining')}: <span className="font-mono font-medium">{formatAmount(remaining.toFixed(2))}</span></span>
            </div>
            <Progress value={paidPercent} className="h-2" />
            <p className="text-xs text-muted-foreground text-right">{paidPercent.toFixed(0)}% {t('status.paid', 'paid')}</p>
          </CardContent>
        </Card>
      )}

      {/* Header text */}
      {data.header_text && (
        <Card>
          <CardContent className="py-4">
            <p className="text-xs font-medium text-muted-foreground mb-1">{t('invoice_form.header_text', 'Introduction Text')}</p>
            <MarkdownPreview content={data.header_text} className="text-sm" />
          </CardContent>
        </Card>
      )}

      {/* Line items */}
      <LineItemsTable data={data} t={t} />

      {/* Footer text */}
      {data.footer_text && (
        <Card>
          <CardContent className="py-4">
            <p className="text-xs font-medium text-muted-foreground mb-1">{t('invoice_form.footer_text', 'Closing Text')}</p>
            <MarkdownPreview content={data.footer_text} className="text-sm" />
          </CardContent>
        </Card>
      )}

      {/* Notes & payment terms */}
      {(data.notes || data.payment_terms) && (
        <Card>
          <CardContent className="py-4 space-y-3">
            {data.payment_terms && (
              <div>
                <p className="text-xs font-medium text-muted-foreground mb-1">{t('invoice_form.payment_terms', 'Payment Terms')}</p>
                <MarkdownPreview content={data.payment_terms} className="text-sm" />
              </div>
            )}
            {data.notes && (
              <div>
                <p className="text-xs font-medium text-muted-foreground mb-1">{t('common.notes', 'Notes')}</p>
                <MarkdownPreview content={data.notes} className="text-sm text-muted-foreground" />
              </div>
            )}
          </CardContent>
        </Card>
      )}

      {/* Journal entries */}
      {(data.journal_entry_id || data.payment_journal_entry_id) && (
        <Card>
          <CardHeader><CardTitle className="text-base">{t('invoices.linked_journal_entries', 'Linked Journal Entries')}</CardTitle></CardHeader>
          <CardContent className="space-y-1 text-sm">
            {data.journal_entry_id && <p>{t('invoices.invoice_entry', 'Invoice Entry')}: <span className="font-mono">{data.journal_entry_id}</span></p>}
            {data.payment_journal_entry_id && <p>{t('invoices.payment_entry', 'Payment Entry')}: <span className="font-mono">{data.payment_journal_entry_id}</span></p>}
          </CardContent>
        </Card>
      )}

      {/* Payment history */}
      {(payments ?? []).length > 0 && (
        <Card>
          <CardHeader><CardTitle className="text-base">{t('invoices.payment_history', 'Payment History')}</CardTitle></CardHeader>
          <CardContent className="p-0">
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>{t('common.date', 'Date')}</TableHead>
                  <TableHead className="text-right">{t('invoices.amount', 'Amount')}</TableHead>
                  <TableHead className="hidden sm:table-cell">{t('invoices.method', 'Method')}</TableHead>
                  <TableHead className="hidden sm:table-cell">{t('invoices.reference', 'Reference')}</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {(payments ?? []).map((p) => (
                  <TableRow key={p.id}>
                    <TableCell className="font-mono text-sm">{formatDate(p.payment_date, dateFormat)}</TableCell>
                    <TableCell className="text-right font-mono text-sm font-medium">{formatAmount(p.amount)}</TableCell>
                    <TableCell className="hidden sm:table-cell text-sm text-muted-foreground">{p.payment_method ?? '—'}</TableCell>
                    <TableCell className="hidden sm:table-cell text-sm text-muted-foreground">{p.reference ?? '—'}</TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </CardContent>
        </Card>
      )}

      {/* Dunning history */}
      {(dunningHistory ?? []).length > 0 && (
        <Card>
          <CardHeader><CardTitle className="text-base">{t('invoice_dialogs.reminder.title', 'Payment Reminders')}</CardTitle></CardHeader>
          <CardContent className="p-0">
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>{t('invoice_dialogs.reminder.level', 'Level')}</TableHead>
                  <TableHead>{t('status.sent', 'Sent')}</TableHead>
                  <TableHead>{t('invoices.fee', 'Fee')}</TableHead>
                  <TableHead className="hidden sm:table-cell">{t('common.email', 'Email')}</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {(dunningHistory ?? []).map((d) => (
                  <TableRow key={d.id}>
                    <TableCell><Badge variant="outline">{d.level_name}</Badge></TableCell>
                    <TableCell className="text-sm">{formatDate(d.sent_at.split(' ')[0], dateFormat)}</TableCell>
                    <TableCell className="font-mono text-sm">
                      {parseFloat(d.fee_amount) > 0 ? `CHF ${parseFloat(d.fee_amount).toFixed(2)}` : '—'}
                    </TableCell>
                    <TableCell className="hidden sm:table-cell text-sm">{d.email_sent ? t('common.yes', 'Yes') : t('common.no', 'No')}</TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </CardContent>
        </Card>
      )}

      {/* Dialogs */}
      <PdfPreviewDialog
        open={pdfOpen}
        onOpenChange={setPdfOpen}
        invoiceId={id!}
        invoiceNumber={data.invoice_number ?? undefined}
      />
      <ReminderDialog
        open={reminderOpen} onOpenChange={setReminderOpen}
        levelId={reminderLevelId} onLevelChange={setReminderLevelId}
        levels={dunningLevels ?? []} onSend={handleSendReminder}
        isPending={sendReminder.isPending}
      />
      <MarkPaidDialog
        open={payOpen} onOpenChange={setPayOpen}
        date={payDate} onDateChange={setPayDate}
        accountId={payAccountId} onAccountChange={setPayAccountId}
        accounts={bankAccounts} onConfirm={handlePay}
        isPending={payInvoice.isPending}
      />
      <RecordPaymentDialog
        open={recordOpen} onOpenChange={setRecordOpen}
        form={recordForm} onFormChange={setRecordForm}
        accounts={bankAccounts} invoiceTotal={data?.total ?? '0'}
        remaining={remaining.toFixed(2)} onRecord={handleRecordPayment}
        isPending={recordPayment.isPending}
      />
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

function LineItemsTable({
  data,
  t,
}: {
  data: { lines: Array<{ id: string; position: number; description: string; quantity: string; unit_price: string; vat_amount: string; line_total: string }>; subtotal: string; vat_amount: string; total: string };
  t: (key: string, fallback?: string) => string;
}) {
  return (
    <Card>
      <CardHeader><CardTitle className="text-base">{t('invoice_form.line_items', 'Line Items')}</CardTitle></CardHeader>
      <CardContent className="p-0">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead className="w-12">#</TableHead>
              <TableHead>{t('common.description', 'Description')}</TableHead>
              <TableHead className="text-right">{t('invoice_form.qty_short', 'Qty')}</TableHead>
              <TableHead className="text-right">{t('invoice_form.unit_price', 'Unit Price')}</TableHead>
              <TableHead className="text-right">VAT</TableHead>
              <TableHead className="text-right">{t('common.total', 'Total')}</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {data.lines.map((line) => (
              <TableRow key={line.id}>
                <TableCell className="text-muted-foreground">{line.position}</TableCell>
                <TableCell>
                  <MarkdownPreview content={line.description} className="text-sm" />
                </TableCell>
                <TableCell className="text-right font-mono text-sm">{formatAmount(line.quantity)}</TableCell>
                <TableCell className="text-right font-mono text-sm">{formatAmount(line.unit_price)}</TableCell>
                <TableCell className="text-right font-mono text-sm">{formatAmount(line.vat_amount)}</TableCell>
                <TableCell className="text-right font-mono text-sm font-medium">{formatAmount(line.line_total)}</TableCell>
              </TableRow>
            ))}
            <TableRow className="bg-muted/50">
              <TableCell colSpan={5} className="text-right font-medium">{t('invoice_form.subtotal', 'Subtotal')}</TableCell>
              <TableCell className="text-right font-mono text-sm font-medium">{formatAmount(data.subtotal)}</TableCell>
            </TableRow>
            <TableRow className="bg-muted/50">
              <TableCell colSpan={5} className="text-right font-medium">VAT</TableCell>
              <TableCell className="text-right font-mono text-sm font-medium">{formatAmount(data.vat_amount)}</TableCell>
            </TableRow>
            <TableRow className="bg-muted/50 font-bold">
              <TableCell colSpan={5} className="text-right">{t('common.total', 'Total')}</TableCell>
              <TableCell className="text-right font-mono text-sm">{formatAmount(data.total)}</TableCell>
            </TableRow>
          </TableBody>
        </Table>
      </CardContent>
    </Card>
  );
}
