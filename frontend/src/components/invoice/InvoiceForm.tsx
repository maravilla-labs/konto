import { useState, useEffect, useRef } from 'react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { RichTextEditor } from '@/components/ui/rich-text-editor';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import {
  useAccounts,
  useContacts,
  useDefaultAccounts,
  useProjects,
  useVatRates,
  useContactVatInfo,
} from '@/hooks/useApi';
import { useBankAccounts } from '@/hooks/useSettingsApi';
import { SUPPORTED_LANGUAGES } from '@/lib/language';
import { useI18n } from '@/i18n';
import { FileText, Import, QrCode } from 'lucide-react';
import { InvoiceLineTable } from './InvoiceLineTable';
import { TimeEntryImportDialog } from './TimeEntryImportDialog';
import type { InvoiceFormData, LineFormData } from './InvoiceFormTypes';
import { emptyLine, computeLineTotals } from './InvoiceFormTypes';
import { ContactPicker } from '@/components/contacts/ContactPicker';
import { VatModeBanner } from '@/components/contacts/VatModeBanner';

// Re-export types for backward compatibility
export type { InvoiceFormData, LineFormData } from './InvoiceFormTypes';
export { emptyLine, toCreateLines } from './InvoiceFormTypes';

interface InvoiceFormProps {
  form: InvoiceFormData;
  setForm: React.Dispatch<React.SetStateAction<InvoiceFormData>>;
  onSubmit: () => void;
  isPending: boolean;
  submitLabel: string;
  secondaryAction?: { label: string; onClick: () => void; isPending?: boolean };
}

export function InvoiceForm({
  form,
  setForm,
  onSubmit,
  isPending,
  submitLabel,
  secondaryAction,
}: InvoiceFormProps) {
  const { t } = useI18n();
  const [importDialogOpen, setImportDialogOpen] = useState(false);

  const { data: contactsData } = useContacts({ per_page: 200 });
  const { data: projectsData } = useProjects({ per_page: 200 });
  const { data: accountsData } = useAccounts({ per_page: 500 });
  const { data: vatRatesData } = useVatRates();
  const { data: defaultAccountsData } = useDefaultAccounts();
  const { data: vatInfoData } = useContactVatInfo(form.contact_id || undefined);
  const { data: bankAccountsData } = useBankAccounts();

  const contacts = contactsData?.data ?? [];
  const allProjects = projectsData?.data ?? [];
  const bankAccounts = bankAccountsData ?? [];
  const projects = form.contact_id
    ? allProjects.filter((p) => p.contact_id === form.contact_id || !p.contact_id)
    : allProjects;
  const accounts = (accountsData?.data ?? []).filter((a) => a.account_type === 'revenue');
  const vatRates = vatRatesData ?? [];
  const defaultAccounts = defaultAccountsData ?? [];
  const resolvedVatMode = vatInfoData?.resolved_mode;
  const prevContactIdRef = useRef(form.contact_id);
  useEffect(() => {
    if (form.contact_id !== prevContactIdRef.current && vatInfoData?.default_vat_rate_id) {
      const mode = vatInfoData.resolved_mode;
      if (mode === 'reverse_charge' || mode === 'export_exempt') {
        setForm((prev) => ({ ...prev, default_vat_rate_id: vatInfoData.default_vat_rate_id! }));
      }
    }
    prevContactIdRef.current = form.contact_id;
  }, [form.contact_id, vatInfoData]);

  const revenueDefault = defaultAccounts.find((d) => d.setting_key === 'revenue_default');
  const defaultAccountId = form.default_account_id || revenueDefault?.account_id || '';

  const activeVatRates = vatRates.filter((v) => v.is_active);
  const normalVatRate = activeVatRates.find((v) => v.vat_type === 'normal');
  const defaultVatRateId = form.default_vat_rate_id || normalVatRate?.id || '';

  const totals = computeLineTotals(form.lines, vatRates, defaultVatRateId);

  // Determine effective bank account and payment slip type
  const defaultBankAccount = bankAccounts.find((b) => b.is_default);
  const selectedBankAccount = form.bank_account_id
    ? bankAccounts.find((b) => b.id === form.bank_account_id)
    : defaultBankAccount;
  const effectiveIban = selectedBankAccount?.qr_iban || selectedBankAccount?.iban || '';
  const isQrIban = (() => {
    const clean = effectiveIban.replace(/\s/g, '').toUpperCase();
    if (clean.length < 9) return false;
    if (!clean.startsWith('CH') && !clean.startsWith('LI')) return false;
    const iid = parseInt(clean.slice(4, 9), 10);
    return iid >= 30000 && iid <= 31999;
  })();
  const paymentSlipType = isQrIban ? 'QRR' : 'SCOR';

  const paymentTermOptions = [
    { value: '__none__', label: t('invoice_form.payment_terms_custom', 'Custom') },
    { value: 'Due on Receipt', label: t('invoice_form.term_due_on_receipt', 'Due on Receipt') },
    { value: 'Net 10', label: t('invoice_form.term_net_10', 'Net 10') },
    { value: 'Net 30', label: t('invoice_form.term_net_30', 'Net 30') },
    { value: 'Net 60', label: t('invoice_form.term_net_60', 'Net 60') },
  ];

  function updateLine(index: number, field: keyof LineFormData, value: string) {
    const next = [...form.lines];
    next[index] = { ...next[index], [field]: value };
    setForm({ ...form, lines: next });
  }

  function addLine() {
    setForm({ ...form, lines: [...form.lines, emptyLine()] });
  }

  function removeLine(index: number) {
    if (form.lines.length <= 1) return;
    setForm({ ...form, lines: form.lines.filter((_, i) => i !== index) });
  }

  function handleProjectChange(projectId: string) {
    const project = allProjects.find((p) => p.id === projectId);
    if (!project) {
      setForm({ ...form, project_id: projectId });
      return;
    }
    setForm({
      ...form,
      project_id: projectId,
      contact_id: project.contact_id ?? form.contact_id,
      contact_person_id: project.contact_person_id ?? form.contact_person_id,
      language: project.language ?? form.language,
    });
  }

  function handleContactChange(contactId: string, personContactId?: string) {
    const selectedContact = contacts.find((c) => c.id === contactId);
    const updates: Partial<InvoiceFormData> = {
      contact_id: contactId,
      contact_person_id: personContactId || '',
      language: form.language || selectedContact?.language || '',
    };
    if (contactId !== form.contact_id) {
      updates.project_id = '';
    }
    setForm({ ...form, ...updates });
  }

  function handleTimeEntryImport(importedLines: LineFormData[]) {
    const hasEmptyFirstLine =
      form.lines.length === 1 &&
      !form.lines[0].description &&
      !form.lines[0].unit_price;
    if (hasEmptyFirstLine) {
      setForm({ ...form, lines: importedLines });
    } else {
      setForm({ ...form, lines: [...form.lines, ...importedLines] });
    }
  }

  const selectedProject = allProjects.find((p) => p.id === form.project_id);
  const projectRate = selectedProject?.hourly_rate ? parseFloat(selectedProject.hourly_rate) : undefined;

  return (
    <div className="space-y-6">
      <Tabs defaultValue="details">
        <TabsList>
          <TabsTrigger value="details">{t('invoice_form.tab_details', 'Details')}</TabsTrigger>
          <TabsTrigger value="lines">{t('invoice_form.tab_lines', 'Lines')}</TabsTrigger>
          <TabsTrigger value="text">{t('invoice_form.tab_text', 'Text & Notes')}</TabsTrigger>
        </TabsList>

        {/* Tab 1: Details */}
        <TabsContent value="details" className="space-y-4 pt-4">
          {/* Card 1: Client & Project */}
          <Card>
            <CardHeader className="pb-3">
              <CardTitle className="text-sm font-medium uppercase tracking-wider text-muted-foreground">
                {t('invoice_form.client_project', 'Client & Project')}
              </CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div>
                <Label>{t('common.contact', 'Contact')}</Label>
                <ContactPicker
                  value={form.contact_id}
                  personValue={form.contact_person_id}
                  onChange={handleContactChange}
                />
              </div>
              {resolvedVatMode && resolvedVatMode !== 'normal' && (
                <VatModeBanner vatMode={resolvedVatMode} />
              )}
              <div className="grid gap-4 sm:grid-cols-2">
                <div>
                  <Label>{t('common.project', 'Project')} ({t('common.optional', 'optional')})</Label>
                  <Select value={form.project_id || '__none__'} onValueChange={(v) => handleProjectChange(v === '__none__' ? '' : v)}>
                    <SelectTrigger>
                      <SelectValue placeholder={t('common.none', 'None')} />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="__none__">{t('common.none', 'None')}</SelectItem>
                      {projects.map((p) => (
                        <SelectItem key={p.id} value={p.id}>{p.number ? `${p.number} — ${p.name}` : p.name}</SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </div>
                {form.project_id && (
                  <div className="flex items-end">
                    <Button
                      type="button"
                      variant="outline"
                      onClick={() => setImportDialogOpen(true)}
                      className="w-full sm:w-auto"
                    >
                      <Import className="mr-2 h-4 w-4 sm:mr-1" />
                      <span className="sm:inline">{t('invoice_form.import_time_entries', 'Import Time Entries')}</span>
                    </Button>
                  </div>
                )}
              </div>
            </CardContent>
          </Card>

          {/* Card 2: Billing Settings */}
          <Card>
            <CardHeader className="pb-3">
              <CardTitle className="text-sm font-medium uppercase tracking-wider text-muted-foreground">
                {t('invoice_form.billing_settings', 'Billing Settings')}
              </CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
                <div>
                  <Label>{t('common.issue_date', 'Issue Date')}</Label>
                  <Input
                    type="date"
                    value={form.issue_date}
                    onChange={(e) => setForm({ ...form, issue_date: e.target.value })}
                  />
                </div>
                <div>
                  <Label>{t('common.due_date', 'Due Date')}</Label>
                  <Input
                    type="date"
                    value={form.due_date}
                    onChange={(e) => setForm({ ...form, due_date: e.target.value })}
                  />
                </div>
                <div>
                  <Label>{t('common.language', 'Language')}</Label>
                  <Select
                    value={form.language || '__auto__'}
                    onValueChange={(v) => setForm({ ...form, language: v === '__auto__' ? '' : v })}
                  >
                    <SelectTrigger>
                      <SelectValue placeholder={t('common.automatic', 'Automatic')} />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="__auto__">{t('common.automatic', 'Automatic')}</SelectItem>
                      {SUPPORTED_LANGUAGES.map((lang) => (
                        <SelectItem key={lang.code} value={lang.code}>
                          {lang.label}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </div>
                <div>
                  <Label>{t('invoice_form.payment_terms', 'Payment Terms')}</Label>
                  <Select
                    value={form.payment_terms || '__none__'}
                    onValueChange={(v) => {
                      const terms = v === '__none__' ? '' : v;
                      const daysMap: Record<string, number> = {
                        'Due on Receipt': 0,
                        'Net 10': 10,
                        'Net 30': 30,
                        'Net 60': 60,
                      };
                      const days = daysMap[terms];
                      let newDueDate = form.due_date;
                      if (days !== undefined && form.issue_date) {
                        const d = new Date(form.issue_date);
                        d.setDate(d.getDate() + days);
                        newDueDate = d.toISOString().split('T')[0];
                      }
                      setForm({ ...form, payment_terms: terms, due_date: newDueDate });
                    }}
                  >
                    <SelectTrigger>
                      <SelectValue placeholder={t('invoice_form.select_terms', 'Select terms')} />
                    </SelectTrigger>
                    <SelectContent>
                      {paymentTermOptions.map((opt) => (
                        <SelectItem key={opt.value} value={opt.value}>
                          {opt.label}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </div>
              </div>
              <div className="grid gap-4 sm:grid-cols-2">
                <div>
                  <Label>{t('invoice_form.revenue_account', 'Revenue Account')}</Label>
                  <Select
                    value={form.default_account_id || defaultAccountId || '__default__'}
                    onValueChange={(v) => setForm({ ...form, default_account_id: v === '__default__' ? '' : v })}
                  >
                    <SelectTrigger>
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="__default__">{t('invoice_form.default_account', 'Default')}</SelectItem>
                      {accounts.map((a) => (
                        <SelectItem key={a.id} value={a.id}>
                          {a.number} {a.name}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </div>
                <div>
                  <Label>{t('invoice_form.default_vat', 'Default VAT Rate')}</Label>
                  <Select
                    value={form.default_vat_rate_id || defaultVatRateId || '__default__'}
                    onValueChange={(v) => setForm({ ...form, default_vat_rate_id: v === '__default__' ? '' : v })}
                  >
                    <SelectTrigger>
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="__default__">{t('invoice_form.default_account', 'Default')}</SelectItem>
                      {activeVatRates.map((v) => (
                        <SelectItem key={v.id} value={v.id}>
                          {v.name} ({v.rate}%)
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </div>
              </div>
              {bankAccounts.length > 0 && (
                <div className="grid gap-4 sm:grid-cols-2">
                  <div>
                    <Label>{t('invoice_form.bank_account', 'Bank Account')}</Label>
                    <Select
                      value={form.bank_account_id || '__default__'}
                      onValueChange={(v) => setForm({ ...form, bank_account_id: v === '__default__' ? '' : v })}
                    >
                      <SelectTrigger>
                        <SelectValue />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="__default__">
                          {defaultBankAccount ? `${defaultBankAccount.name} (${t('common.default', 'Default')})` : t('common.default', 'Default')}
                        </SelectItem>
                        {bankAccounts.filter((b) => !b.is_default).map((b) => (
                          <SelectItem key={b.id} value={b.id}>
                            {b.name} — {b.iban}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                  </div>
                  <div className="flex items-end pb-1">
                    <div className="flex items-center gap-2 rounded-md border px-3 py-2 text-sm">
                      <QrCode className="h-4 w-4 text-muted-foreground" />
                      <span className="text-muted-foreground">{t('invoice_form.payment_slip', 'Payment Slip')}:</span>
                      <span className="font-medium">
                        {paymentSlipType === 'QRR' ? t('invoice_form.qr_reference', 'QR-Referenz') : t('invoice_form.iso_reference', 'ISO 11649 (SCOR)')}
                      </span>
                    </div>
                  </div>
                </div>
              )}
            </CardContent>
          </Card>
        </TabsContent>

        {/* Tab 2: Lines */}
        <TabsContent value="lines" className="pt-4">
          <InvoiceLineTable
            lines={form.lines}
            defaultVatRateId={defaultVatRateId}
            defaultAccountId={defaultAccountId}
            accounts={accounts}
            vatRates={vatRates}
            onUpdateLine={updateLine}
            onAddLine={addLine}
            onRemoveLine={removeLine}
          />
        </TabsContent>

        {/* Tab 3: Text & Notes */}
        <TabsContent value="text" className="space-y-4 pt-4">
          {/* Card 1: Printed on Invoice */}
          <Card>
            <CardHeader className="pb-3">
              <CardTitle className="flex items-center gap-2 text-sm font-medium uppercase tracking-wider text-muted-foreground">
                <FileText className="h-4 w-4" />
                {t('invoice_form.printed_text', 'Printed on Invoice')}
              </CardTitle>
              <p className="text-xs text-muted-foreground">
                {t('invoice_form.printed_text_desc', 'These texts appear on the invoice document')}
              </p>
            </CardHeader>
            <CardContent>
              <div className="grid gap-4 sm:grid-cols-2">
                <div>
                  <Label>{t('invoice_form.header_text', 'Introduction Text')}</Label>
                  <RichTextEditor
                    value={form.header_text}
                    onChange={(md) => setForm({ ...form, header_text: md })}
                    placeholder={t('invoice_form.header_text_placeholder', 'Text shown before the line items...')}
                    className="min-h-[100px]"
                  />
                </div>
                <div>
                  <Label>{t('invoice_form.footer_text', 'Closing Text')}</Label>
                  <RichTextEditor
                    value={form.footer_text}
                    onChange={(md) => setForm({ ...form, footer_text: md })}
                    placeholder={t('invoice_form.footer_text_placeholder', 'Text shown after the line items...')}
                    className="min-h-[100px]"
                  />
                </div>
              </div>
            </CardContent>
          </Card>

          {/* Card 2: Internal Notes */}
          <Card className="border-dashed">
            <CardHeader className="pb-3">
              <CardTitle className="text-sm font-medium uppercase tracking-wider text-muted-foreground">
                {t('invoice_form.internal_notes', 'Internal Notes')}
              </CardTitle>
              <p className="text-xs text-muted-foreground">
                {t('invoice_form.internal_notes_desc', 'Only visible to your team')}
              </p>
            </CardHeader>
            <CardContent>
              <RichTextEditor
                value={form.notes}
                onChange={(md) => setForm({ ...form, notes: md })}
                placeholder={t('invoice_form.optional_notes', 'Internal notes (not printed on invoice)')}
                className="min-h-[100px]"
              />
            </CardContent>
          </Card>
        </TabsContent>
      </Tabs>

      {/* Sticky action footer */}
      <div className="sticky bottom-0 z-10 -mx-4 border-t bg-background/95 px-4 py-3 backdrop-blur sm:-mx-6 sm:px-6">
        <div className="flex items-center justify-between">
          <div className="text-sm font-medium">
            {t('invoice_form.total_preview', 'Total')}:{' '}
            <span className="font-mono text-lg font-bold">{totals.grandTotal.toFixed(2)}</span>
          </div>
          <div className="flex gap-2">
            {secondaryAction && (
              <Button
                variant="outline"
                onClick={secondaryAction.onClick}
                disabled={secondaryAction.isPending || !form.contact_id}
              >
                {secondaryAction.label}
              </Button>
            )}
            <Button onClick={onSubmit} disabled={isPending || !form.contact_id || form.lines.length === 0}>
              {submitLabel}
            </Button>
          </div>
        </div>
      </div>

      {/* Time entry import dialog */}
      {form.project_id && (
        <TimeEntryImportDialog
          open={importDialogOpen}
          onOpenChange={setImportDialogOpen}
          projectId={form.project_id}
          defaultAccountId={defaultAccountId}
          defaultVatRateId={defaultVatRateId}
          defaultRate={projectRate}
          onImport={handleTimeEntryImport}
        />
      )}
    </div>
  );
}
