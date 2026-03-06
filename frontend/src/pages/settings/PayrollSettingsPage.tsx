import { useState, useEffect } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Skeleton } from '@/components/ui/skeleton';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { usePayrollSettings, useUpdatePayrollSettings } from '@/hooks/useApi';
import { useBankAccounts } from '@/hooks/useSettingsApi';
import { toast } from 'sonner';
import { useI18n } from '@/i18n';
import type { UpdatePayrollSettings } from '@/types/payroll-settings';

const defaultForm: UpdatePayrollSettings = {
  ahv_iv_eo_rate_employee: 5.3, ahv_iv_eo_rate_employer: 5.3,
  alv_rate_employee: 1.1, alv_rate_employer: 1.1, alv_salary_cap: 148200,
  bvg_coordination_deduction: 26460, bvg_entry_threshold: 22680,
  bvg_min_insured_salary: 3780, bvg_max_insured_salary: 64260,
  bvg_rate_25_34: 7, bvg_rate_35_44: 10, bvg_rate_45_54: 15, bvg_rate_55_65: 18,
  bvg_risk_rate: 2.5, bvg_employer_share_pct: 50,
  nbu_rate_employee: 1.5, bu_rate_employer: 0.1,
  ktg_rate_employee: 0.5, ktg_rate_employer: 0.5,
  fak_rate_employer: 1.6, uvg_max_salary: 148200,
};

export function PayrollSettingsPage() {
  const { t } = useI18n();
  const { data, isLoading } = usePayrollSettings();
  const updateSettings = useUpdatePayrollSettings();
  const { data: bankAccounts } = useBankAccounts();

  const [form, setForm] = useState<UpdatePayrollSettings>({ ...defaultForm });

  useEffect(() => {
    if (data) {
      setForm({
        ahv_iv_eo_rate_employee: data.ahv_iv_eo_rate_employee,
        ahv_iv_eo_rate_employer: data.ahv_iv_eo_rate_employer,
        alv_rate_employee: data.alv_rate_employee,
        alv_rate_employer: data.alv_rate_employer,
        alv_salary_cap: data.alv_salary_cap,
        bvg_coordination_deduction: data.bvg_coordination_deduction,
        bvg_entry_threshold: data.bvg_entry_threshold,
        bvg_min_insured_salary: data.bvg_min_insured_salary,
        bvg_max_insured_salary: data.bvg_max_insured_salary,
        bvg_rate_25_34: data.bvg_rate_25_34,
        bvg_rate_35_44: data.bvg_rate_35_44,
        bvg_rate_45_54: data.bvg_rate_45_54,
        bvg_rate_55_65: data.bvg_rate_55_65,
        bvg_risk_rate: data.bvg_risk_rate,
        bvg_employer_share_pct: data.bvg_employer_share_pct,
        nbu_rate_employee: data.nbu_rate_employee,
        bu_rate_employer: data.bu_rate_employer,
        ktg_rate_employee: data.ktg_rate_employee,
        ktg_rate_employer: data.ktg_rate_employer,
        fak_rate_employer: data.fak_rate_employer,
        uvg_max_salary: data.uvg_max_salary,
        payment_bank_account_id: data.payment_bank_account_id,
        company_clearing_number: data.company_clearing_number,
      });
    }
  }, [data]);

  function handleSave() {
    updateSettings.mutate(form, {
      onSuccess: () => toast.success(t('payroll_settings.saved')),
      onError: () => toast.error(t('payroll_settings.save_failed')),
    });
  }

  function setNum(field: keyof UpdatePayrollSettings, value: string) {
    setForm({ ...form, [field]: parseFloat(value) || 0 });
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
        <h2 className="text-lg font-semibold">{t('payroll_settings.title')}</h2>
        <p className="text-sm text-muted-foreground">{t('payroll_settings.subtitle')}</p>
      </div>

      {/* AHV/IV/EO */}
      <Card>
        <CardHeader><CardTitle className="text-base">{t('payroll_settings.section_ahv')}</CardTitle></CardHeader>
        <CardContent>
          <div className="grid gap-4 sm:grid-cols-2">
            <div><Label>{t('payroll_settings.ahv_iv_eo_employee', 'AHV/IV/EO Employee')} (%)</Label><Input type="number" step="0.01" value={form.ahv_iv_eo_rate_employee} onChange={(e) => setNum('ahv_iv_eo_rate_employee', e.target.value)} /></div>
            <div><Label>{t('payroll_settings.ahv_iv_eo_employer', 'AHV/IV/EO Employer')} (%)</Label><Input type="number" step="0.01" value={form.ahv_iv_eo_rate_employer} onChange={(e) => setNum('ahv_iv_eo_rate_employer', e.target.value)} /></div>
          </div>
        </CardContent>
      </Card>

      {/* ALV */}
      <Card>
        <CardHeader><CardTitle className="text-base">{t('payroll_settings.section_alv')}</CardTitle></CardHeader>
        <CardContent>
          <div className="grid gap-4 sm:grid-cols-2">
            <div><Label>{t('payroll_settings.alv_employee', 'ALV Employee')} (%)</Label><Input type="number" step="0.01" value={form.alv_rate_employee} onChange={(e) => setNum('alv_rate_employee', e.target.value)} /></div>
            <div><Label>{t('payroll_settings.alv_employer', 'ALV Employer')} (%)</Label><Input type="number" step="0.01" value={form.alv_rate_employer} onChange={(e) => setNum('alv_rate_employer', e.target.value)} /></div>
            <div><Label>{t('payroll_settings.alv_salary_cap', 'ALV Salary Cap (CHF)')}</Label><Input type="number" step="1" value={form.alv_salary_cap} onChange={(e) => setNum('alv_salary_cap', e.target.value)} /></div>
          </div>
        </CardContent>
      </Card>

      {/* BVG */}
      <Card>
        <CardHeader><CardTitle className="text-base">{t('payroll_settings.section_bvg')}</CardTitle></CardHeader>
        <CardContent className="space-y-4">
          <div className="grid gap-4 sm:grid-cols-2">
            <div><Label>{t('payroll_settings.bvg_coordination', 'Coordination Deduction (CHF)')}</Label><Input type="number" step="1" value={form.bvg_coordination_deduction} onChange={(e) => setNum('bvg_coordination_deduction', e.target.value)} /></div>
            <div><Label>{t('payroll_settings.bvg_entry_threshold', 'Entry Threshold (CHF)')}</Label><Input type="number" step="1" value={form.bvg_entry_threshold} onChange={(e) => setNum('bvg_entry_threshold', e.target.value)} /></div>
            <div><Label>{t('payroll_settings.bvg_min_insured', 'Min Insured Salary (CHF)')}</Label><Input type="number" step="1" value={form.bvg_min_insured_salary} onChange={(e) => setNum('bvg_min_insured_salary', e.target.value)} /></div>
            <div><Label>{t('payroll_settings.bvg_max_insured', 'Max Insured Salary (CHF)')}</Label><Input type="number" step="1" value={form.bvg_max_insured_salary} onChange={(e) => setNum('bvg_max_insured_salary', e.target.value)} /></div>
          </div>
          <h4 className="text-sm font-medium text-muted-foreground">{t('payroll_settings.bvg_age_rates', 'Age-Based Savings Rates')}</h4>
          <div className="grid gap-4 sm:grid-cols-4">
            <div><Label>{t('payroll_settings.bvg_25_34', '25-34')} (%)</Label><Input type="number" step="0.1" value={form.bvg_rate_25_34} onChange={(e) => setNum('bvg_rate_25_34', e.target.value)} /></div>
            <div><Label>{t('payroll_settings.bvg_35_44', '35-44')} (%)</Label><Input type="number" step="0.1" value={form.bvg_rate_35_44} onChange={(e) => setNum('bvg_rate_35_44', e.target.value)} /></div>
            <div><Label>{t('payroll_settings.bvg_45_54', '45-54')} (%)</Label><Input type="number" step="0.1" value={form.bvg_rate_45_54} onChange={(e) => setNum('bvg_rate_45_54', e.target.value)} /></div>
            <div><Label>{t('payroll_settings.bvg_55_65', '55-65')} (%)</Label><Input type="number" step="0.1" value={form.bvg_rate_55_65} onChange={(e) => setNum('bvg_rate_55_65', e.target.value)} /></div>
          </div>
          <div className="grid gap-4 sm:grid-cols-2">
            <div><Label>{t('payroll_settings.bvg_risk_rate', 'Risk Rate')} (%)</Label><Input type="number" step="0.01" value={form.bvg_risk_rate} onChange={(e) => setNum('bvg_risk_rate', e.target.value)} /></div>
            <div><Label>{t('payroll_settings.bvg_employer_share', 'Employer Share')} (%)</Label><Input type="number" step="1" value={form.bvg_employer_share_pct} onChange={(e) => setNum('bvg_employer_share_pct', e.target.value)} /></div>
          </div>
        </CardContent>
      </Card>

      {/* UVG / NBU / BU */}
      <Card>
        <CardHeader><CardTitle className="text-base">{t('payroll_settings.section_uvg', 'UVG / Accident Insurance')}</CardTitle></CardHeader>
        <CardContent>
          <div className="grid gap-4 sm:grid-cols-2">
            <div><Label>{t('payroll_settings.nbu_rate', 'NBU Employee')} (%)</Label><Input type="number" step="0.01" value={form.nbu_rate_employee} onChange={(e) => setNum('nbu_rate_employee', e.target.value)} /></div>
            <div><Label>{t('payroll_settings.bu_rate', 'BU Employer')} (%)</Label><Input type="number" step="0.01" value={form.bu_rate_employer} onChange={(e) => setNum('bu_rate_employer', e.target.value)} /></div>
            <div><Label>{t('payroll_settings.uvg_max_salary', 'UVG Max Salary (CHF)')}</Label><Input type="number" step="1" value={form.uvg_max_salary} onChange={(e) => setNum('uvg_max_salary', e.target.value)} /></div>
          </div>
        </CardContent>
      </Card>

      {/* KTG / FAK */}
      <Card>
        <CardHeader><CardTitle className="text-base">{t('payroll_settings.section_ktg_fak', 'KTG / FAK')}</CardTitle></CardHeader>
        <CardContent>
          <div className="grid gap-4 sm:grid-cols-2">
            <div><Label>{t('payroll_settings.ktg_employee', 'KTG Employee')} (%)</Label><Input type="number" step="0.01" value={form.ktg_rate_employee} onChange={(e) => setNum('ktg_rate_employee', e.target.value)} /></div>
            <div><Label>{t('payroll_settings.ktg_employer', 'KTG Employer')} (%)</Label><Input type="number" step="0.01" value={form.ktg_rate_employer} onChange={(e) => setNum('ktg_rate_employer', e.target.value)} /></div>
            <div><Label>{t('payroll_settings.fak_rate', 'FAK Employer')} (%)</Label><Input type="number" step="0.01" value={form.fak_rate_employer} onChange={(e) => setNum('fak_rate_employer', e.target.value)} /></div>
          </div>
        </CardContent>
      </Card>

      {/* Payment */}
      <Card>
        <CardHeader><CardTitle className="text-base">{t('payroll_settings.section_payment', 'Payment Configuration')}</CardTitle></CardHeader>
        <CardContent>
          <div className="grid gap-4 sm:grid-cols-2">
            <div>
              <Label>{t('payroll_settings.payment_bank_account', 'Payment Bank Account')}</Label>
              <Select
                value={form.payment_bank_account_id || ''}
                onValueChange={(v) => setForm({ ...form, payment_bank_account_id: v || null })}
              >
                <SelectTrigger>
                  <SelectValue placeholder={t('payroll_settings.select_bank_account', 'Select bank account...')} />
                </SelectTrigger>
                <SelectContent>
                  {bankAccounts?.map((ba) => (
                    <SelectItem key={ba.id} value={ba.id}>
                      {ba.name} — {ba.iban}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
            <div>
              <Label>{t('payroll_settings.clearing_number', 'Company Clearing Number (IID)')}</Label>
              <Input
                value={form.company_clearing_number || ''}
                onChange={(e) => setForm({ ...form, company_clearing_number: e.target.value || null })}
                placeholder="e.g. 00230"
              />
            </div>
          </div>
        </CardContent>
      </Card>

      <div className="flex justify-end">
        <Button onClick={handleSave} disabled={updateSettings.isPending}>
          {updateSettings.isPending ? t('common.saving') : t('common.save')}
        </Button>
      </div>
    </div>
  );
}
