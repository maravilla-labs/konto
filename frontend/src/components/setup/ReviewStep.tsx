import { useI18n } from '@/i18n';
import { Button } from '@/components/ui/button';
import { Loader2, Pencil, Check } from 'lucide-react';
import { motion } from 'framer-motion';

interface ReviewData {
  language: string;
  admin: { full_name: string; email: string };
  company: {
    legal_name: string;
    trade_name: string;
    street: string;
    postal_code: string;
    city: string;
    country: string;
    legal_entity_type: string;
  };
  accounting: {
    default_currency: string;
    vat_method: string;
    flat_rate_percentage: string;
    date_format: string;
    fiscal_year_start_month: string;
  };
}

interface ReviewStepProps {
  data: ReviewData;
  onGoToStep: (step: number) => void;
  onComplete: () => void;
  isSubmitting: boolean;
  error: string | null;
}

const LANG_NAMES: Record<string, string> = {
  en: 'English',
  de: 'Deutsch',
  fr: 'Français',
  it: 'Italiano',
};

const CURRENCY_NAMES: Record<string, string> = {
  'cur-chf': 'CHF',
  'cur-eur': 'EUR',
  'cur-usd': 'USD',
};

const MONTH_NAMES = [
  '', 'January', 'February', 'March', 'April', 'May', 'June',
  'July', 'August', 'September', 'October', 'November', 'December',
];

const sectionVariants = {
  hidden: { opacity: 0, y: 12 },
  visible: (i: number) => ({
    opacity: 1,
    y: 0,
    transition: { delay: i * 0.06, duration: 0.3, ease: 'easeOut' as const },
  }),
};

function Section({
  title,
  step,
  index,
  onEdit,
  children,
}: {
  title: string;
  step: number;
  index: number;
  onEdit: (step: number) => void;
  children: React.ReactNode;
}) {
  return (
    <motion.div
      custom={index}
      variants={sectionVariants}
      initial="hidden"
      animate="visible"
      className="rounded-xl border border-[#E8E6E1]/80 bg-[#FAFAF8]/60 p-4"
    >
      <div className="mb-2 flex items-center justify-between">
        <div className="flex items-center gap-2">
          <div className="flex h-5 w-5 items-center justify-center rounded-full bg-[#1B2B4B]">
            <Check className="h-2.5 w-2.5 text-white" strokeWidth={3} />
          </div>
          <h3
            className="text-sm font-medium text-[#1B2B4B]"
            style={{ fontFamily: "'DM Serif Display', serif" }}
          >
            {title}
          </h3>
        </div>
        <button
          type="button"
          onClick={() => onEdit(step)}
          className="flex items-center gap-1 rounded-lg px-2 py-1 text-xs font-medium text-[#1B2B4B]/30 transition-all hover:bg-[#1B2B4B]/5 hover:text-[#1B2B4B]/60"
        >
          <Pencil className="h-3 w-3" />
          Edit
        </button>
      </div>
      <div className="space-y-1 pl-7 text-sm">{children}</div>
    </motion.div>
  );
}

function Row({ label, value }: { label: string; value: string }) {
  return (
    <div className="flex justify-between py-0.5">
      <span className="text-[#1B2B4B]/35 font-light">{label}</span>
      <span className="font-medium text-[#1B2B4B]">{value || '—'}</span>
    </div>
  );
}

export function ReviewStep({ data, onGoToStep, onComplete, isSubmitting, error }: ReviewStepProps) {
  const { t } = useI18n();

  return (
    <div className="space-y-5">
      <div className="text-center">
        <h2
          className="text-xl sm:text-2xl text-[#1B2B4B]"
          style={{ fontFamily: "'DM Serif Display', serif" }}
        >
          {t('setup.review_title', 'Review & Complete')}
        </h2>
        <p className="mt-1.5 text-sm text-[#1B2B4B]/40 font-light">
          {t('setup.review_desc', 'Review your settings before completing the setup.')}
        </p>
      </div>

      <div className="space-y-2.5">
        <Section title={t('setup.step_language', 'Language')} step={0} index={0} onEdit={onGoToStep}>
          <Row label={t('common.language', 'Language')} value={LANG_NAMES[data.language] ?? data.language} />
        </Section>

        <Section title={t('setup.step_account', 'Admin Account')} step={1} index={1} onEdit={onGoToStep}>
          <Row label={t('setup.full_name', 'Full Name')} value={data.admin.full_name} />
          <Row label={t('common.email', 'Email')} value={data.admin.email} />
        </Section>

        <Section title={t('setup.step_company', 'Company')} step={2} index={2} onEdit={onGoToStep}>
          <Row label={t('setup.legal_name', 'Legal Name')} value={data.company.legal_name} />
          <Row label={t('setup.trade_name', 'Trade Name')} value={data.company.trade_name} />
          <Row
            label={t('setup.address', 'Address')}
            value={[data.company.street, `${data.company.postal_code} ${data.company.city}`.trim()].filter(Boolean).join(', ')}
          />
          <Row label={t('setup.country', 'Country')} value={data.company.country} />
          <Row label={t('setup.legal_entity_type', 'Entity Type')} value={data.company.legal_entity_type} />
        </Section>

        <Section title={t('setup.step_accounting', 'Accounting')} step={3} index={3} onEdit={onGoToStep}>
          <Row label={t('setup.default_currency', 'Currency')} value={CURRENCY_NAMES[data.accounting.default_currency] ?? data.accounting.default_currency} />
          <Row label={t('setup.vat_method', 'VAT Method')} value={data.accounting.vat_method} />
          {data.accounting.vat_method === 'flat_rate' && (
            <Row label={t('setup.flat_rate_percentage', 'Flat Rate')} value={`${data.accounting.flat_rate_percentage}%`} />
          )}
          <Row label={t('setup.date_format', 'Date Format')} value={data.accounting.date_format} />
          <Row
            label={t('setup.fiscal_year_start', 'FY Start')}
            value={MONTH_NAMES[parseInt(data.accounting.fiscal_year_start_month)] ?? ''}
          />
        </Section>
      </div>

      {error && (
        <motion.div
          className="rounded-xl border border-[#C1272D]/20 bg-[#C1272D]/5 p-3 text-sm text-[#C1272D]"
          initial={{ opacity: 0, y: 6 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.25 }}
        >
          {error}
        </motion.div>
      )}

      <motion.div
        initial={{ opacity: 0, y: 10 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.3, duration: 0.3 }}
      >
        <Button
          type="button"
          onClick={onComplete}
          disabled={isSubmitting}
          className="w-full bg-[#1B2B4B] hover:bg-[#0F1D33] text-white h-10 text-sm rounded-lg font-medium transition-colors"
        >
          {isSubmitting ? (
            <>
              <Loader2 className="mr-2 h-4 w-4 animate-spin" />
              {t('setup.completing', 'Setting up your system...')}
            </>
          ) : (
            t('setup.complete', 'Complete Setup')
          )}
        </Button>
      </motion.div>
    </div>
  );
}
