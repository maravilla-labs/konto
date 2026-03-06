import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { useI18n } from '@/i18n';

interface Props {
  value: string;
  onChange: (value: string) => void;
}

const HINTS: Record<string, string> = {
  auto: 'contact_vat.hint_auto',
  normal: 'contact_vat.hint_normal',
  reverse_charge: 'contact_vat.hint_reverse_charge',
  export_exempt: 'contact_vat.hint_export_exempt',
};

const HINT_DEFAULTS: Record<string, string> = {
  auto: 'VAT mode will be determined from the contact country.',
  normal: 'Standard domestic VAT rates apply.',
  reverse_charge: '0% VAT — reverse charge procedure (EU B2B).',
  export_exempt: '0% VAT — export delivery outside EU.',
};

export function VatModeSelector({ value, onChange }: Props) {
  const { t } = useI18n();

  return (
    <div className="space-y-1">
      <Select value={value || 'auto'} onValueChange={onChange}>
        <SelectTrigger>
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="auto">{t('contact_vat.auto', 'Auto (detect from country)')}</SelectItem>
          <SelectItem value="normal">{t('contact_vat.normal', 'Normal')}</SelectItem>
          <SelectItem value="reverse_charge">{t('contact_vat.reverse_charge', 'Reverse Charge')}</SelectItem>
          <SelectItem value="export_exempt">{t('contact_vat.export_exempt', 'Export Exempt')}</SelectItem>
        </SelectContent>
      </Select>
      {value && HINTS[value] && (
        <p className="text-xs text-muted-foreground">
          {t(HINTS[value], HINT_DEFAULTS[value])}
        </p>
      )}
    </div>
  );
}
