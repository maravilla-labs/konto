import { useI18n } from '@/i18n';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { motion } from 'framer-motion';

interface CompanyData {
  legal_name: string;
  trade_name: string;
  street: string;
  postal_code: string;
  city: string;
  country: string;
  legal_entity_type: string;
}

interface CompanyInfoStepProps {
  value: CompanyData;
  onChange: (data: CompanyData) => void;
  errors: Record<string, string>;
}

const ENTITY_TYPES = [
  { value: 'GmbH', label: 'GmbH' },
  { value: 'AG', label: 'AG' },
  { value: 'Einzelfirma', label: 'Einzelfirma' },
  { value: 'Verein', label: 'Verein' },
  { value: 'Genossenschaft', label: 'Genossenschaft' },
  { value: 'Stiftung', label: 'Stiftung' },
  { value: 'KlG', label: 'KlG' },
  { value: 'KmG', label: 'KmG' },
];

const fieldVariants = {
  hidden: { opacity: 0, y: 10 },
  visible: (i: number) => ({
    opacity: 1,
    y: 0,
    transition: { delay: i * 0.05, duration: 0.3, ease: 'easeOut' as const },
  }),
};

const inputClassName =
  'h-10 rounded-xl border-[#E8E6E1] bg-[#FAFAF8] px-4 text-sm text-[#1B2B4B] placeholder:text-[#1B2B4B]/20 focus:border-[#1B2B4B] focus:ring-[#1B2B4B]/10';

const selectTriggerClassName =
  'h-10 rounded-xl border-[#E8E6E1] bg-[#FAFAF8] px-4 text-sm text-[#1B2B4B] focus:border-[#1B2B4B] focus:ring-[#1B2B4B]/10';

export function CompanyInfoStep({ value, onChange, errors }: CompanyInfoStepProps) {
  const { t } = useI18n();

  const update = (field: keyof CompanyData, val: string) => {
    onChange({ ...value, [field]: val });
  };

  return (
    <div className="space-y-5">
      <div className="text-center">
        <h2
          className="text-xl sm:text-2xl text-[#1B2B4B]"
          style={{ fontFamily: "'DM Serif Display', serif" }}
        >
          {t('setup.company_info', 'Company Information')}
        </h2>
        <p className="mt-1.5 text-sm text-[#1B2B4B]/40 font-light">
          {t('setup.company_info_desc', 'Enter your company details. These appear on invoices and documents.')}
        </p>
      </div>

      <div className="space-y-4">
        <motion.div custom={0} variants={fieldVariants} initial="hidden" animate="visible" className="grid grid-cols-1 sm:grid-cols-2 gap-4">
          <div className="space-y-1.5">
            <Label htmlFor="legal_name" className="text-xs font-semibold uppercase tracking-wider text-[#1B2B4B]/50">
              {t('setup.legal_name', 'Legal Name')} *
            </Label>
            <Input
              id="legal_name"
              value={value.legal_name}
              onChange={(e) => update('legal_name', e.target.value)}
              placeholder="Muster GmbH"
              className={inputClassName}
            />
            {errors.legal_name && <p className="text-xs text-[#C1272D]">{errors.legal_name}</p>}
          </div>

          <div className="space-y-1.5">
            <Label htmlFor="trade_name" className="text-xs font-semibold uppercase tracking-wider text-[#1B2B4B]/50">
              {t('setup.trade_name', 'Trade Name')}
              <span className="ml-1 font-normal normal-case tracking-normal text-[#1B2B4B]/25">({t('common.optional', 'optional')})</span>
            </Label>
            <Input
              id="trade_name"
              value={value.trade_name}
              onChange={(e) => update('trade_name', e.target.value)}
              placeholder="My Brand"
              className={inputClassName}
            />
          </div>
        </motion.div>

        <motion.div custom={1} variants={fieldVariants} initial="hidden" animate="visible" className="space-y-1.5">
          <Label htmlFor="street" className="text-xs font-semibold uppercase tracking-wider text-[#1B2B4B]/50">
            {t('setup.street', 'Street')}
          </Label>
          <Input
            id="street"
            value={value.street}
            onChange={(e) => update('street', e.target.value)}
            placeholder="Bahnhofstrasse 1"
            className={inputClassName}
          />
        </motion.div>

        <motion.div custom={2} variants={fieldVariants} initial="hidden" animate="visible" className="grid grid-cols-1 sm:grid-cols-2 gap-4">
          <div className="space-y-1.5">
            <Label htmlFor="postal_code" className="text-xs font-semibold uppercase tracking-wider text-[#1B2B4B]/50">
              {t('setup.postal_code', 'Postal Code')}
            </Label>
            <Input
              id="postal_code"
              value={value.postal_code}
              onChange={(e) => update('postal_code', e.target.value)}
              placeholder="8000"
              className={inputClassName}
            />
          </div>
          <div className="space-y-1.5">
            <Label htmlFor="city" className="text-xs font-semibold uppercase tracking-wider text-[#1B2B4B]/50">
              {t('setup.city', 'City')}
            </Label>
            <Input
              id="city"
              value={value.city}
              onChange={(e) => update('city', e.target.value)}
              placeholder="Zürich"
              className={inputClassName}
            />
          </div>
        </motion.div>

        <motion.div custom={3} variants={fieldVariants} initial="hidden" animate="visible" className="grid grid-cols-1 sm:grid-cols-2 gap-4">
          <div className="space-y-1.5">
            <Label htmlFor="country" className="text-xs font-semibold uppercase tracking-wider text-[#1B2B4B]/50">
              {t('setup.country', 'Country')}
            </Label>
            <Select value={value.country} onValueChange={(v) => update('country', v)}>
              <SelectTrigger id="country" className={selectTriggerClassName}>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="CH">Switzerland</SelectItem>
                <SelectItem value="DE">Germany</SelectItem>
                <SelectItem value="AT">Austria</SelectItem>
                <SelectItem value="LI">Liechtenstein</SelectItem>
              </SelectContent>
            </Select>
          </div>
          <div className="space-y-1.5">
            <Label htmlFor="entity_type" className="text-xs font-semibold uppercase tracking-wider text-[#1B2B4B]/50">
              {t('setup.legal_entity_type', 'Legal Entity Type')}
            </Label>
            <Select value={value.legal_entity_type} onValueChange={(v) => update('legal_entity_type', v)}>
              <SelectTrigger id="entity_type" className={selectTriggerClassName}>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {ENTITY_TYPES.map((et) => (
                  <SelectItem key={et.value} value={et.value}>{et.label}</SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
        </motion.div>
      </div>
    </div>
  );
}
