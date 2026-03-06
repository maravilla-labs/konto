import { useState, useEffect, useRef } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { Skeleton } from '@/components/ui/skeleton';
import { Switch } from '@/components/ui/switch';
import { useSettings, useUpdateSettings, useUploadLogo } from '@/hooks/useSettingsApi';
import { useCurrencies } from '@/hooks/useApi';
import { toast } from 'sonner';
import { Upload, AlertTriangle } from 'lucide-react';
import type { UpdateCompanySettings } from '@/types/settings';
import { SUPPORTED_LANGUAGES } from '@/lib/language';
import { isTauri, resolveUploadUrl } from '@/lib/platform';
import { resetDatabase } from '@/lib/native';
import { NumberingCard } from '@/components/settings/NumberingCard';
import { useI18n } from '@/i18n';

const vatMethods = [
  { value: 'effective', label: 'Effective Method' },
  { value: 'flat_rate', label: 'Flat-Rate Method (Saldosteuersatz)' },
  { value: 'net_tax', label: 'Net Tax Rate Method' },
];

export function CompanySettingsPage() {
  const { data, isLoading } = useSettings();
  const updateSettings = useUpdateSettings();
  const uploadLogo = useUploadLogo();
  const { data: currencies } = useCurrencies();
  const fileRef = useRef<HTMLInputElement>(null);
  const [resetting, setResetting] = useState(false);
  const { t } = useI18n();
  const isDesktop = isTauri();

  const [form, setForm] = useState<UpdateCompanySettings>({
    legal_name: '',
    trade_name: '',
    street: '',
    postal_code: '',
    city: '',
    country: 'CH',
    email: '',
    phone: '',
    website: '',
    vat_number: '',
    vat_method: 'flat_rate',
    flat_rate_percentage: 6.2,
    register_number: '',
    default_currency_id: '',
    date_format: 'dd.MM.yyyy',
    number_format: 'ch',
    ui_language: 'en',
    fiscal_year_start_month: 1,
    tax_id_label: 'UID/MWST',
    audit_optout: true,
  });

  useEffect(() => {
    if (data) {
      setForm({
        legal_name: data.legal_name,
        trade_name: data.trade_name ?? '',
        street: data.street,
        postal_code: data.postal_code,
        city: data.city,
        country: data.country,
        email: data.email ?? '',
        phone: data.phone ?? '',
        website: data.website ?? '',
        vat_number: data.vat_number ?? '',
        vat_method: data.vat_method,
        flat_rate_percentage: data.flat_rate_percentage ? parseFloat(data.flat_rate_percentage) : null,
        register_number: data.register_number ?? '',
        default_currency_id: data.default_currency_id ?? '',
        date_format: data.date_format ?? 'dd.MM.yyyy',
        number_format: data.number_format ?? 'ch',
        ui_language: data.ui_language ?? 'en',
        fiscal_year_start_month: data.fiscal_year_start_month ?? 1,
        tax_id_label: data.tax_id_label ?? 'UID/MWST',
        audit_optout: data.audit_optout ?? true,
        project_number_auto: data.project_number_auto ?? false,
        project_number_prefix: data.project_number_prefix ?? 'P-',
        project_number_start: data.project_number_start ?? 1,
        project_number_min_length: data.project_number_min_length ?? 3,
        project_number_restart_yearly: data.project_number_restart_yearly ?? false,
        customer_number_auto: data.customer_number_auto ?? false,
        customer_number_prefix: data.customer_number_prefix ?? 'K-',
        customer_number_start: data.customer_number_start ?? 1,
        customer_number_min_length: data.customer_number_min_length ?? 6,
        customer_number_restart_yearly: data.customer_number_restart_yearly ?? false,
        employee_number_auto: data.employee_number_auto ?? false,
        employee_number_prefix: data.employee_number_prefix ?? 'M-',
        employee_number_start: data.employee_number_start ?? 1,
        employee_number_min_length: data.employee_number_min_length ?? 3,
        employee_number_restart_yearly: data.employee_number_restart_yearly ?? false,
      });
    }
  }, [data]);

  function handleSave() {
    updateSettings.mutate(form, {
      onSuccess: () => toast.success('Settings saved'),
      onError: () => toast.error('Failed to save settings'),
    });
  }

  function handleLogoUpload(e: React.ChangeEvent<HTMLInputElement>) {
    const file = e.target.files?.[0];
    if (!file) return;
    uploadLogo.mutate(file, {
      onSuccess: () => toast.success('Logo uploaded'),
      onError: () => toast.error('Failed to upload logo'),
    });
  }

  function set(field: keyof UpdateCompanySettings, value: string) {
    setForm({ ...form, [field]: value });
  }

  if (isLoading) {
    return (
      <div className="space-y-4">
        <Skeleton className="h-8 w-48" />
        <Skeleton className="h-64 w-full" />
      </div>
    );
  }

  return (
    <div className="space-y-4">
      <div>
        <h2 className="text-lg font-semibold">Company Settings</h2>
        <p className="text-sm text-muted-foreground">Manage your company information</p>
      </div>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">Logo</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="flex items-center gap-4">
            {data?.logo_url ? (
              <img src={resolveUploadUrl(data.logo_url) ?? ''} alt="Logo" className="h-16 w-16 rounded object-contain border" />
            ) : (
              <div className="flex h-16 w-16 items-center justify-center rounded border bg-muted text-xs text-muted-foreground">
                No logo
              </div>
            )}
            <div>
              <input ref={fileRef} type="file" accept="image/*" className="hidden" onChange={handleLogoUpload} />
              <Button variant="outline" size="sm" onClick={() => fileRef.current?.click()} disabled={uploadLogo.isPending}>
                <Upload className="mr-1 h-3.5 w-3.5" /> Upload Logo
              </Button>
            </div>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">Company Details</CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="grid gap-4 sm:grid-cols-2">
            <div>
              <Label>Legal Name</Label>
              <Input value={form.legal_name} onChange={(e) => set('legal_name', e.target.value)} />
            </div>
            <div>
              <Label>Trade Name</Label>
              <Input value={form.trade_name ?? ''} onChange={(e) => set('trade_name', e.target.value)} placeholder="Optional" />
            </div>
          </div>
          <div>
            <Label>Street</Label>
            <Input value={form.street} onChange={(e) => set('street', e.target.value)} />
          </div>
          <div className="grid gap-4 sm:grid-cols-4">
            <div>
              <Label>Postal Code</Label>
              <Input value={form.postal_code} onChange={(e) => set('postal_code', e.target.value)} />
            </div>
            <div>
              <Label>City</Label>
              <Input value={form.city} onChange={(e) => set('city', e.target.value)} />
            </div>
            <div>
              <Label>Country</Label>
              <Input value={form.country} onChange={(e) => set('country', e.target.value)} />
            </div>
          </div>
          <div className="grid gap-4 sm:grid-cols-2">
            <div>
              <Label>Email</Label>
              <Input type="email" value={form.email ?? ''} onChange={(e) => set('email', e.target.value)} />
            </div>
            <div>
              <Label>Phone</Label>
              <Input value={form.phone ?? ''} onChange={(e) => set('phone', e.target.value)} />
            </div>
          </div>
          <div>
            <Label>Website</Label>
            <Input value={form.website ?? ''} onChange={(e) => set('website', e.target.value)} />
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">Tax & Registration</CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="grid gap-4 sm:grid-cols-2">
            <div>
              <Label>VAT Number</Label>
              <Input value={form.vat_number ?? ''} onChange={(e) => set('vat_number', e.target.value)} placeholder="e.g. CHE-123.456.789 MWST" />
            </div>
            <div>
              <Label>VAT Method</Label>
              <Select value={form.vat_method} onValueChange={(v) => set('vat_method', v)}>
                <SelectTrigger><SelectValue /></SelectTrigger>
                <SelectContent>
                  {vatMethods.map((m) => (
                    <SelectItem key={m.value} value={m.value}>{m.label}</SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
          </div>
          {form.vat_method === 'flat_rate' && (
            <div className="max-w-xs">
              <Label>Flat Rate Percentage (%)</Label>
              <Input
                type="number"
                step="0.1"
                value={form.flat_rate_percentage ?? ''}
                onChange={(e) => setForm({ ...form, flat_rate_percentage: e.target.value ? parseFloat(e.target.value) : null })}
                placeholder="e.g. 6.2"
              />
              <p className="text-xs text-muted-foreground mt-1">
                ESTV-approved flat rate for your NOGA code
              </p>
            </div>
          )}
          <div>
            <Label>Register Number (HR)</Label>
            <Input value={form.register_number ?? ''} onChange={(e) => set('register_number', e.target.value)} placeholder="Optional" />
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">Regional Settings</CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="grid gap-4 sm:grid-cols-2">
            <div>
              <Label>Default Currency</Label>
              <Select
                value={form.default_currency_id || '__none__'}
                onValueChange={(v) => setForm({ ...form, default_currency_id: v === '__none__' ? '' : v })}
              >
                <SelectTrigger><SelectValue placeholder="Select currency" /></SelectTrigger>
                <SelectContent>
                  <SelectItem value="__none__">Not set</SelectItem>
                  {(currencies ?? []).map((c) => (
                    <SelectItem key={c.id} value={c.id}>{c.code} — {c.name}</SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
            <div>
              <Label>Tax ID Label</Label>
              <Input
                value={form.tax_id_label ?? 'UID/MWST'}
                onChange={(e) => setForm({ ...form, tax_id_label: e.target.value })}
                placeholder="UID/MWST"
              />
            </div>
          </div>
          <div className="grid gap-4 sm:grid-cols-3">
            <div>
              <Label>Date Format</Label>
              <Select
                value={form.date_format ?? 'dd.MM.yyyy'}
                onValueChange={(v) => setForm({ ...form, date_format: v })}
              >
                <SelectTrigger><SelectValue /></SelectTrigger>
                <SelectContent>
                  <SelectItem value="dd.MM.yyyy">dd.MM.yyyy (31.12.2025)</SelectItem>
                  <SelectItem value="yyyy-MM-dd">yyyy-MM-dd (2025-12-31)</SelectItem>
                  <SelectItem value="MM/dd/yyyy">MM/dd/yyyy (12/31/2025)</SelectItem>
                </SelectContent>
              </Select>
            </div>
            <div>
              <Label>Number Format</Label>
              <Select
                value={form.number_format ?? 'ch'}
                onValueChange={(v) => setForm({ ...form, number_format: v })}
              >
                <SelectTrigger><SelectValue /></SelectTrigger>
                <SelectContent>
                  <SelectItem value="ch">Swiss (1&apos;234.56)</SelectItem>
                  <SelectItem value="de">German (1.234,56)</SelectItem>
                  <SelectItem value="en">English (1,234.56)</SelectItem>
                </SelectContent>
              </Select>
            </div>
            <div>
              <Label>Default UI Language</Label>
              <Select
                value={form.ui_language ?? 'en'}
                onValueChange={(v) => setForm({ ...form, ui_language: v })}
              >
                <SelectTrigger><SelectValue /></SelectTrigger>
                <SelectContent>
                  {SUPPORTED_LANGUAGES.map((lang) => (
                    <SelectItem key={lang.code} value={lang.code}>
                      {lang.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
            <div>
              <Label>Fiscal Year Start</Label>
              <Select
                value={String(form.fiscal_year_start_month ?? 1)}
                onValueChange={(v) => setForm({ ...form, fiscal_year_start_month: Number(v) })}
              >
                <SelectTrigger><SelectValue /></SelectTrigger>
                <SelectContent>
                  {['January','February','March','April','May','June','July','August','September','October','November','December'].map((m, i) => (
                    <SelectItem key={i + 1} value={String(i + 1)}>{m}</SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
          </div>
        </CardContent>
      </Card>

      <NumberingCard
        title={t('settings.project_numbering', 'Project Numbering')}
        autoField="project_number_auto"
        prefixField="project_number_prefix"
        startField="project_number_start"
        minLengthField="project_number_min_length"
        yearlyField="project_number_restart_yearly"
        form={form}
        setForm={setForm}
      />

      <NumberingCard
        title={t('settings.customer_numbering', 'Customer Numbering')}
        autoField="customer_number_auto"
        prefixField="customer_number_prefix"
        startField="customer_number_start"
        minLengthField="customer_number_min_length"
        yearlyField="customer_number_restart_yearly"
        form={form}
        setForm={setForm}
      />

      <NumberingCard
        title={t('settings.employee_numbering', 'Employee Numbering')}
        autoField="employee_number_auto"
        prefixField="employee_number_prefix"
        startField="employee_number_start"
        minLengthField="employee_number_min_length"
        yearlyField="employee_number_restart_yearly"
        form={form}
        setForm={setForm}
      />

      <Card>
        <CardHeader>
          <CardTitle className="text-base">Audit & Revision</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="flex items-center justify-between">
            <div className="space-y-0.5">
              <Label>Verzicht auf Revision (Opting-out)</Label>
              <p className="text-xs text-muted-foreground">
                When enabled, the company opts out of audit review per Art. 727a Abs. 2 OR. This section will be included in the Anhang of the annual report.
              </p>
            </div>
            <Switch
              checked={form.audit_optout ?? true}
              onCheckedChange={(checked) => setForm({ ...form, audit_optout: checked })}
            />
          </div>
        </CardContent>
      </Card>

      <div className="flex justify-end">
        <Button onClick={handleSave} disabled={updateSettings.isPending}>
          {updateSettings.isPending ? 'Saving...' : 'Save Settings'}
        </Button>
      </div>

      {isDesktop && (
        <Card className="border-destructive/50">
          <CardHeader>
            <CardTitle className="text-base flex items-center gap-2 text-destructive">
              <AlertTriangle className="h-4 w-4" />
              {t('settings.danger_zone', 'Danger Zone')}
            </CardTitle>
            <CardDescription>
              {t('settings.reset_description', 'Delete the database and start fresh. All data will be permanently lost.')}
            </CardDescription>
          </CardHeader>
          <CardContent>
            <Button
              variant="destructive"
              disabled={resetting}
              onClick={async () => {
                const confirmed = window.confirm(
                  t('settings.reset_confirm', 'Are you sure? This will delete ALL data and cannot be undone.')
                );
                if (!confirmed) return;
                setResetting(true);
                try {
                  await resetDatabase();
                  // App restarts automatically via Tauri
                } catch (e) {
                  toast.error(t('settings.reset_failed', 'Failed to reset database'));
                  setResetting(false);
                }
              }}
            >
              {resetting
                ? t('settings.resetting', 'Resetting...')
                : t('settings.reset_database', 'Reset Database')}
            </Button>
          </CardContent>
        </Card>
      )}
    </div>
  );
}
