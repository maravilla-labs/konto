import { Info } from 'lucide-react';
import { useI18n } from '@/i18n';

interface Props {
  vatMode: string;
}

export function VatModeBanner({ vatMode }: Props) {
  const { t } = useI18n();

  if (vatMode === 'normal' || vatMode === 'auto') return null;

  const message =
    vatMode === 'reverse_charge'
      ? t('contact_vat.banner_reverse_charge', 'Reverse Charge — 0% VAT applies. VAT liability transferred to recipient.')
      : t('contact_vat.banner_export_exempt', 'Export Exempt — 0% VAT applies. Export delivery.');

  return (
    <div className="flex items-start gap-2 rounded-md border border-blue-200 bg-blue-50 p-3 text-sm text-blue-800 dark:border-blue-800 dark:bg-blue-950/50 dark:text-blue-200">
      <Info className="mt-0.5 h-4 w-4 shrink-0" />
      <span>{message}</span>
    </div>
  );
}
