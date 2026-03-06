import { Link } from 'react-router-dom';
import { Card, CardContent } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import {
  Table, TableBody, TableCell, TableHead, TableHeader, TableRow,
} from '@/components/ui/table';
import { useI18n } from '@/i18n';

interface Invoice {
  id: string;
  invoice_number?: string | null;
  status: string;
  issue_date: string;
  total: string | number;
}

interface ProjectInvoicesTabProps {
  invoices: Invoice[];
}

export function ProjectInvoicesTab({ invoices }: ProjectInvoicesTabProps) {
  const { t } = useI18n();

  return (
    <Card className="mt-2">
      <CardContent className="p-0">
        {invoices.length > 0 ? (
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>{t('common.number', 'Number')}</TableHead>
                <TableHead>{t('common.status', 'Status')}</TableHead>
                <TableHead>{t('common.date', 'Date')}</TableHead>
                <TableHead className="text-right">{t('common.total', 'Total')}</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {invoices.map((inv) => (
                <TableRow key={inv.id}>
                  <TableCell>
                    <Link to={`/invoices/${inv.id}`} className="text-primary hover:underline font-medium">
                      {inv.invoice_number ?? t('invoices.draft', 'Draft')}
                    </Link>
                  </TableCell>
                  <TableCell><Badge variant="secondary">{inv.status}</Badge></TableCell>
                  <TableCell className="font-mono">{inv.issue_date}</TableCell>
                  <TableCell className="text-right font-mono">{Number(inv.total).toFixed(2)}</TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        ) : (
          <p className="py-6 text-center text-sm text-muted-foreground">
            {t('projects.no_invoices', 'No invoices for this project.')}
          </p>
        )}
      </CardContent>
    </Card>
  );
}
