import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Skeleton } from '@/components/ui/skeleton';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import { useTopOutstanding } from '@/hooks/useApi';
import { useSettings } from '@/hooks/useSettingsApi';
import { useI18n } from '@/i18n';
import { formatCurrency } from '@/lib/locale';

export function TopOutstanding() {
  const { t } = useI18n();
  const { data: settings } = useSettings();
  const { data, isLoading } = useTopOutstanding(5);
  const numberFormat = settings?.number_format ?? 'ch';

  return (
    <Card>
      <CardHeader className="pb-2">
        <CardTitle className="text-base">
          {t('dashboard.top_outstanding_contacts', 'Top Outstanding Contacts')}
        </CardTitle>
      </CardHeader>
      <CardContent className="p-0">
        {isLoading ? (
          <div className="space-y-2 p-4">
            {Array.from({ length: 3 }).map((_, i) => (
              <Skeleton key={i} className="h-8 w-full" />
            ))}
          </div>
        ) : !data || data.length === 0 ? (
          <p className="p-4 text-sm text-muted-foreground">
            {t('dashboard.no_outstanding_invoices', 'No outstanding invoices')}
          </p>
        ) : (
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead className="w-8">#</TableHead>
                <TableHead>{t('dashboard.contact', 'Contact')}</TableHead>
                <TableHead className="text-right">{t('dashboard.outstanding_short', 'Outstanding')}</TableHead>
                <TableHead className="text-right w-20">{t('dashboard.inv_short', 'Inv.')}</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {data.map((c, i) => (
                <TableRow key={c.contact_id}>
                  <TableCell className="text-muted-foreground">{i + 1}</TableCell>
                  <TableCell className="font-medium">{c.contact_name}</TableCell>
                  <TableCell className="text-right font-mono text-sm">
                    {formatCurrency(c.outstanding_amount, 'CHF', numberFormat)}
                  </TableCell>
                  <TableCell className="text-right text-muted-foreground">
                    {c.invoice_count}
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        )}
      </CardContent>
    </Card>
  );
}
