import { useI18n } from '@/i18n';
import { Label } from '@/components/ui/label';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { Input } from '@/components/ui/input';
import { motion, AnimatePresence } from 'framer-motion';

interface AccountingData {
  default_currency: string;
  vat_method: string;
  flat_rate_percentage: string;
  date_format: string;
  fiscal_year_start_month: string;
}

interface AccountingConfigStepProps {
  value: AccountingData;
  onChange: (data: AccountingData) => void;
}

const DATE_FORMATS = [
  { value: 'dd.MM.yyyy', label: 'dd.MM.yyyy (31.12.2026)' },
  { value: 'dd/MM/yyyy', label: 'dd/MM/yyyy (31/12/2026)' },
  { value: 'yyyy-MM-dd', label: 'yyyy-MM-dd (2026-12-31)' },
  { value: 'MM/dd/yyyy', label: 'MM/dd/yyyy (12/31/2026)' },
];

const MONTHS = [
  { value: '1', label: 'January' },
  { value: '2', label: 'February' },
  { value: '3', label: 'March' },
  { value: '4', label: 'April' },
  { value: '5', label: 'May' },
  { value: '6', label: 'June' },
  { value: '7', label: 'July' },
  { value: '8', label: 'August' },
  { value: '9', label: 'September' },
  { value: '10', label: 'October' },
  { value: '11', label: 'November' },
  { value: '12', label: 'December' },
];

const fieldVariants = {
  hidden: { opacity: 0, y: 10 },
  visible: (i: number) => ({
    opacity: 1,
    y: 0,
    transition: { delay: i * 0.05, duration: 0.3, ease: 'easeOut' as const },
  }),
};

const selectTriggerClassName =
  'h-10 rounded-xl border-[#E8E6E1] bg-[#FAFAF8] px-4 text-sm text-[#1B2B4B] focus:border-[#1B2B4B] focus:ring-[#1B2B4B]/10';

export function AccountingConfigStep({ value, onChange }: AccountingConfigStepProps) {
  const { t } = useI18n();

  const update = (field: keyof AccountingData, val: string) => {
    onChange({ ...value, [field]: val });
  };

  return (
    <div className="space-y-5">
      <div className="text-center">
        <h2
          className="text-xl sm:text-2xl text-[#1B2B4B]"
          style={{ fontFamily: "'DM Serif Display', serif" }}
        >
          {t('setup.accounting_config', 'Accounting Configuration')}
        </h2>
        <p className="mt-1.5 text-sm text-[#1B2B4B]/40 font-light">
          {t('setup.accounting_config_desc', 'Configure your accounting defaults. You can change these later in settings.')}
        </p>
      </div>

      <div className="space-y-4">
        <motion.div custom={0} variants={fieldVariants} initial="hidden" animate="visible" className="grid grid-cols-1 sm:grid-cols-2 gap-4">
          <div className="space-y-1.5">
            <Label className="text-xs font-semibold uppercase tracking-wider text-[#1B2B4B]/50">
              {t('setup.default_currency', 'Default Currency')}
            </Label>
            <Select value={value.default_currency} onValueChange={(v) => update('default_currency', v)}>
              <SelectTrigger className={selectTriggerClassName}>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="cur-chf">CHF — Swiss Franc</SelectItem>
                <SelectItem value="cur-eur">EUR — Euro</SelectItem>
                <SelectItem value="cur-usd">USD — US Dollar</SelectItem>
              </SelectContent>
            </Select>
          </div>

          <div className="space-y-1.5">
            <Label className="text-xs font-semibold uppercase tracking-wider text-[#1B2B4B]/50">
              {t('setup.vat_method', 'VAT Method')}
            </Label>
            <Select value={value.vat_method} onValueChange={(v) => update('vat_method', v)}>
              <SelectTrigger className={selectTriggerClassName}>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="effective">{t('setup.vat_effective', 'Effective Method')}</SelectItem>
                <SelectItem value="flat_rate">{t('setup.vat_flat_rate', 'Flat Rate Method')}</SelectItem>
                <SelectItem value="exempt">{t('setup.vat_exempt', 'VAT Exempt')}</SelectItem>
              </SelectContent>
            </Select>
          </div>
        </motion.div>

        <AnimatePresence>
          {value.vat_method === 'flat_rate' && (
            <motion.div
              className="space-y-1.5"
              initial={{ opacity: 0, height: 0 }}
              animate={{ opacity: 1, height: 'auto' }}
              exit={{ opacity: 0, height: 0 }}
              transition={{ duration: 0.2, ease: 'easeInOut' }}
            >
              <Label className="text-xs font-semibold uppercase tracking-wider text-[#1B2B4B]/50">
                {t('setup.flat_rate_percentage', 'Flat Rate Percentage (%)')}
              </Label>
              <Input
                type="number"
                step="0.1"
                min="0"
                max="100"
                value={value.flat_rate_percentage}
                onChange={(e) => update('flat_rate_percentage', e.target.value)}
                placeholder="6.2"
                className="h-10 rounded-xl border-[#E8E6E1] bg-[#FAFAF8] px-4 text-sm text-[#1B2B4B] placeholder:text-[#1B2B4B]/20 focus:border-[#1B2B4B] focus:ring-[#1B2B4B]/10"
              />
            </motion.div>
          )}
        </AnimatePresence>

        <motion.div custom={1} variants={fieldVariants} initial="hidden" animate="visible" className="grid grid-cols-1 sm:grid-cols-2 gap-4">
          <div className="space-y-1.5">
            <Label className="text-xs font-semibold uppercase tracking-wider text-[#1B2B4B]/50">
              {t('setup.date_format', 'Date Format')}
            </Label>
            <Select value={value.date_format} onValueChange={(v) => update('date_format', v)}>
              <SelectTrigger className={selectTriggerClassName}>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {DATE_FORMATS.map((df) => (
                  <SelectItem key={df.value} value={df.value}>{df.label}</SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>

          <div className="space-y-1.5">
            <Label className="text-xs font-semibold uppercase tracking-wider text-[#1B2B4B]/50">
              {t('setup.fiscal_year_start', 'Fiscal Year Start')}
            </Label>
            <Select value={value.fiscal_year_start_month} onValueChange={(v) => update('fiscal_year_start_month', v)}>
              <SelectTrigger className={selectTriggerClassName}>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {MONTHS.map((m) => (
                  <SelectItem key={m.value} value={m.value}>{m.label}</SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
        </motion.div>
      </div>
    </div>
  );
}
