import { Link } from 'react-router-dom';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Separator } from '@/components/ui/separator';
import { formatAmount } from '@/lib/format';
import {
  Pencil, Send, CreditCard, XCircle, Trash2, Download, Mail, ReceiptText, Bell, Plus, Copy,
} from 'lucide-react';

const statusVariant: Record<string, 'default' | 'secondary' | 'destructive' | 'outline'> = {
  draft: 'secondary', sent: 'default', partial: 'default',
  paid: 'outline', overdue: 'destructive', cancelled: 'destructive',
};

interface InvoiceActionsSidebarProps {
  id: string;
  status: string;
  subtotal: string;
  vatAmount: string;
  total: string;
  notes: string | null;
  paymentTerms: string | null;
  remaining: number;
  recordForm: { amount: string; payment_date: string; payment_account_id: string; payment_method: string; reference: string };
  setRecordForm: (f: InvoiceActionsSidebarProps['recordForm']) => void;
  setRecordOpen: (v: boolean) => void;
  setPayOpen: (v: boolean) => void;
  setReminderOpen: (v: boolean) => void;
  onSend: () => void;
  onCancel: () => void;
  onDelete: () => void;
  onEmail: () => void;
  onDownload: () => void;
  onDuplicate: () => void;
  t: (key: string, fallback?: string) => string;
  sendPending: boolean;
  cancelPending: boolean;
  deletePending: boolean;
  emailPending: boolean;
}

export function InvoiceActionsSidebar({
  id, status, subtotal, vatAmount, total, notes, paymentTerms,
  remaining, recordForm, setRecordForm, setRecordOpen, setPayOpen, setReminderOpen,
  onSend, onCancel, onDelete, onEmail, onDownload, onDuplicate,
  t, sendPending, cancelPending, deletePending, emailPending,
}: InvoiceActionsSidebarProps) {
  return (
    <div className="space-y-4">
      {/* Status */}
      <Card>
        <CardContent className="flex items-center justify-between py-3">
          <span className="text-sm font-medium text-muted-foreground">{t('common.status', 'Status')}</span>
          <Badge variant={statusVariant[status] ?? 'outline'} className="text-sm">
            {t(`status.${status}`, status)}
          </Badge>
        </CardContent>
      </Card>

      {/* Actions */}
      <Card>
        <CardHeader className="pb-2">
          <CardTitle className="text-sm font-medium text-muted-foreground">{t('invoices.actions', 'Actions')}</CardTitle>
        </CardHeader>
        <CardContent className="space-y-2">
          <Button variant="outline" className="w-full justify-start" onClick={onDownload}>
            <Download className="mr-2 h-4 w-4" /> PDF
          </Button>
          <Button variant="outline" className="w-full justify-start" onClick={onDuplicate}>
            <Copy className="mr-2 h-4 w-4" /> {t('invoices.duplicate', 'Duplicate')}
          </Button>

          {status === 'draft' && (
            <>
              <Separator />
              <Button asChild variant="outline" className="w-full justify-start">
                <Link to={`/invoices/${id}/edit`}>
                  <Pencil className="mr-2 h-4 w-4" /> {t('invoices.edit', 'Edit')}
                </Link>
              </Button>
              <Button className="w-full justify-start" onClick={onSend} disabled={sendPending}>
                <Send className="mr-2 h-4 w-4" /> {t('status.sent', 'Send')}
              </Button>
              <Separator />
              <Button variant="destructive" className="w-full justify-start" onClick={onDelete} disabled={deletePending}>
                <Trash2 className="mr-2 h-4 w-4" /> {t('common.delete', 'Delete')}
              </Button>
            </>
          )}

          {(status === 'sent' || status === 'overdue' || status === 'partial') && (
            <>
              <Separator />
              <Button variant="outline" className="w-full justify-start" onClick={onEmail} disabled={emailPending}>
                <Mail className="mr-2 h-4 w-4" /> {t('common.email', 'Email')}
              </Button>
              {(status === 'sent' || status === 'overdue') && (
                <Button variant="outline" className="w-full justify-start" onClick={() => setReminderOpen(true)}>
                  <Bell className="mr-2 h-4 w-4" /> {t('invoice_dialogs.reminder.send', 'Remind')}
                </Button>
              )}
              <Button variant="outline" className="w-full justify-start" onClick={() => {
                setRecordForm({ ...recordForm, amount: remaining.toFixed(2) });
                setRecordOpen(true);
              }}>
                <Plus className="mr-2 h-4 w-4" /> {t('invoice_dialogs.record.action', 'Record Payment')}
              </Button>
              <Button className="w-full justify-start" onClick={() => setPayOpen(true)}>
                <CreditCard className="mr-2 h-4 w-4" /> {t('invoices.mark_fully_paid', 'Mark Fully Paid')}
              </Button>
              <Separator />
              <Button variant="destructive" className="w-full justify-start" onClick={onCancel} disabled={cancelPending}>
                <XCircle className="mr-2 h-4 w-4" /> {t('common.cancel', 'Cancel')}
              </Button>
            </>
          )}

          {status === 'paid' && (
            <>
              <Separator />
              <Button variant="outline" className="w-full justify-start" onClick={onEmail} disabled={emailPending}>
                <Mail className="mr-2 h-4 w-4" /> {t('common.email', 'Email')}
              </Button>
            </>
          )}

          {(status === 'sent' || status === 'paid' || status === 'overdue') && (
            <Button asChild variant="outline" className="w-full justify-start">
              <Link to={`/credit-notes/new?invoice_id=${id}`}>
                <ReceiptText className="mr-2 h-4 w-4" /> {t('nav.credit-notes', 'Credit Note')}
              </Link>
            </Button>
          )}
        </CardContent>
      </Card>

      {/* Summary */}
      <Card>
        <CardHeader className="pb-2">
          <CardTitle className="text-sm font-medium text-muted-foreground">{t('invoices.summary', 'Summary')}</CardTitle>
        </CardHeader>
        <CardContent className="space-y-2">
          <div className="flex justify-between text-sm">
            <span className="text-muted-foreground">{t('invoice_form.subtotal', 'Subtotal')}</span>
            <span className="font-mono">{formatAmount(subtotal)}</span>
          </div>
          <div className="flex justify-between text-sm">
            <span className="text-muted-foreground">VAT</span>
            <span className="font-mono">{formatAmount(vatAmount)}</span>
          </div>
          <Separator />
          <div className="flex justify-between font-medium">
            <span>{t('common.total', 'Total')}</span>
            <span className="font-mono">{formatAmount(total)}</span>
          </div>
        </CardContent>
      </Card>

      {/* Notes */}
      {(notes || paymentTerms) && (
        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm font-medium text-muted-foreground">{t('common.notes', 'Notes')}</CardTitle>
          </CardHeader>
          <CardContent className="space-y-1 text-sm">
            {paymentTerms && (
              <p><span className="font-medium">{t('invoice_form.payment_terms', 'Payment Terms')}:</span> {paymentTerms}</p>
            )}
            {notes && <p className="text-muted-foreground">{notes}</p>}
          </CardContent>
        </Card>
      )}
    </div>
  );
}
