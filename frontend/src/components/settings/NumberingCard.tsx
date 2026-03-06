import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Switch } from '@/components/ui/switch';
import { useI18n } from '@/i18n';

interface NumberingCardProps {
  title: string;
  autoField: string;
  prefixField: string;
  startField: string;
  minLengthField: string;
  yearlyField: string;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  form: any;
  setForm: (form: any) => void;
}

export function NumberingCard({
  title,
  autoField,
  prefixField,
  startField,
  minLengthField,
  yearlyField,
  form,
  setForm,
}: NumberingCardProps) {
  const { t } = useI18n();
  const isAuto = form[autoField] ?? false;

  return (
    <Card>
      <CardHeader>
        <CardTitle className="text-base">{title}</CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        <div className="flex items-center justify-between">
          <div className="space-y-0.5">
            <Label>{t('settings.numbering_auto', 'Auto-assign numbers')}</Label>
            <p className="text-xs text-muted-foreground">
              {t('settings.numbering_auto_desc', 'When enabled, numbers are automatically assigned on creation')}
            </p>
          </div>
          <Switch
            checked={!!isAuto}
            onCheckedChange={(checked) => setForm({ ...form, [autoField]: checked })}
          />
        </div>
        {isAuto && (
          <>
            <div className="grid gap-4 sm:grid-cols-2">
              <div>
                <Label>{t('settings.numbering_prefix', 'Prefix')}</Label>
                <Input
                  value={String(form[prefixField] ?? '')}
                  onChange={(e) => setForm({ ...form, [prefixField]: e.target.value })}
                />
              </div>
              <div>
                <Label>{t('settings.numbering_start', 'Start Number')}</Label>
                <Input
                  type="number"
                  value={Number(form[startField] ?? 1)}
                  onChange={(e) => setForm({ ...form, [startField]: parseInt(e.target.value) || 1 })}
                />
              </div>
            </div>
            <div className="grid gap-4 sm:grid-cols-2">
              <div>
                <Label>{t('settings.numbering_min_length', 'Minimum Length')}</Label>
                <Input
                  type="number"
                  value={Number(form[minLengthField] ?? 3)}
                  onChange={(e) => setForm({ ...form, [minLengthField]: parseInt(e.target.value) || 3 })}
                  min={1}
                  max={10}
                />
              </div>
              <div className="flex items-center gap-3 pt-5">
                <Switch
                  checked={!!form[yearlyField]}
                  onCheckedChange={(checked) => setForm({ ...form, [yearlyField]: checked })}
                />
                <Label>{t('settings.numbering_restart_yearly', 'Restart numbering yearly')}</Label>
              </div>
            </div>
            <div className="rounded-md bg-muted p-3">
              <p className="text-xs text-muted-foreground mb-1">{t('settings.numbering_preview', 'Preview')}</p>
              <p className="text-sm font-mono">
                {(() => {
                  const prefix = (form[prefixField] as string) ?? '';
                  const start = (form[startField] as number) ?? 1;
                  const minLen = (form[minLengthField] as number) ?? 3;
                  const yearly = (form[yearlyField] as boolean) ?? false;
                  const padded = String(start).padStart(minLen, '0');
                  return yearly ? `${prefix}${new Date().getFullYear()}-${padded}` : `${prefix}${padded}`;
                })()}
              </p>
            </div>
          </>
        )}
      </CardContent>
    </Card>
  );
}
