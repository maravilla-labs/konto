import { useState } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Skeleton } from '@/components/ui/skeleton';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import {
  Table, TableBody, TableCell, TableHead, TableHeader, TableRow,
} from '@/components/ui/table';
import { useSalaryCertificates } from '@/hooks/useApi';
import { salaryCertificatesApi } from '@/api/salary-certificates';
import { toast } from 'sonner';
import { useI18n } from '@/i18n';
import { Download, FileDown } from 'lucide-react';

function fmt(n: number) {
  return n.toLocaleString('de-CH', { minimumFractionDigits: 2, maximumFractionDigits: 2 });
}

function downloadBlob(data: Blob, filename: string) {
  const url = URL.createObjectURL(data);
  const a = document.createElement('a');
  a.href = url;
  a.download = filename;
  a.click();
  URL.revokeObjectURL(url);
}

export function SalaryCertificatesPage() {
  const { t } = useI18n();
  const [year, setYear] = useState(new Date().getFullYear());
  const { data, isLoading } = useSalaryCertificates(year);

  async function handleDownloadPdf(employeeId: string, name: string) {
    try {
      const res = await salaryCertificatesApi.downloadPdf(year, employeeId);
      downloadBlob(new Blob([res.data]), `salary-certificate-${year}-${name}.pdf`);
    } catch {
      toast.error(t('salary_certificates.download_failed'));
    }
  }

  async function handleDownloadAll() {
    try {
      const res = await salaryCertificatesApi.downloadZip(year);
      downloadBlob(new Blob([res.data]), `salary-certificates-${year}.zip`);
    } catch {
      toast.error(t('salary_certificates.download_failed'));
    }
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
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-lg font-semibold">{t('salary_certificates.title')}</h2>
          <p className="text-sm text-muted-foreground">{t('salary_certificates.subtitle')}</p>
        </div>
        <div className="flex items-center gap-3">
          <div className="flex items-center gap-2">
            <Label>{t('salary_certificates.year')}</Label>
            <Input
              type="number"
              className="w-24"
              value={year}
              onChange={(e) => setYear(Number(e.target.value))}
            />
          </div>
          {data && data.length > 0 && (
            <Button size="sm" variant="outline" onClick={handleDownloadAll}>
              <FileDown className="mr-1 h-4 w-4" /> {t('salary_certificates.download_all')}
            </Button>
          )}
        </div>
      </div>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">
            {t('salary_certificates.certificates')} {year}
          </CardTitle>
        </CardHeader>
        <CardContent>
          {(!data || data.length === 0) ? (
            <p className="text-sm text-muted-foreground py-8 text-center">
              {t('salary_certificates.no_data')}
            </p>
          ) : (
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>{t('salary_certificates.employee')}</TableHead>
                  <TableHead>{t('salary_certificates.ahv_nr')}</TableHead>
                  <TableHead className="text-right">{t('salary_certificates.months')}</TableHead>
                  <TableHead className="text-right">{t('salary_certificates.gross')}</TableHead>
                  <TableHead className="text-right">{t('salary_certificates.deductions')}</TableHead>
                  <TableHead className="text-right">{t('salary_certificates.net')}</TableHead>
                  <TableHead className="w-10" />
                </TableRow>
              </TableHeader>
              <TableBody>
                {data.map((item) => (
                  <TableRow key={item.employee.id}>
                    <TableCell className="font-medium">
                      {item.employee.first_name} {item.employee.last_name}
                    </TableCell>
                    <TableCell className="text-sm text-muted-foreground">
                      {item.employee.ahv_number}
                    </TableCell>
                    <TableCell className="text-right">{item.months_worked}</TableCell>
                    <TableCell className="text-right">{fmt(item.total_gross)}</TableCell>
                    <TableCell className="text-right">
                      {fmt(item.total_social_deductions + item.total_bvg_employee)}
                    </TableCell>
                    <TableCell className="text-right font-medium">
                      {fmt(item.total_net)}
                    </TableCell>
                    <TableCell>
                      <Button
                        variant="ghost"
                        size="icon"
                        onClick={() =>
                          handleDownloadPdf(
                            item.employee.id,
                            `${item.employee.last_name}-${item.employee.first_name}`
                          )
                        }
                      >
                        <Download className="h-4 w-4" />
                      </Button>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
