import { Card, CardContent } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import {
  Table, TableBody, TableCell, TableHead, TableHeader, TableRow,
} from '@/components/ui/table';
import { useI18n } from '@/i18n';

interface TimeEntry {
  id: string;
  date: string;
  actual_minutes: number;
  description?: string;
  status: string;
  billed: boolean;
}

interface ProjectTimeEntriesTabProps {
  timeData?: { data: TimeEntry[] };
}

export function ProjectTimeEntriesTab({ timeData }: ProjectTimeEntriesTabProps) {
  const { t } = useI18n();
  const entries = timeData?.data ?? [];

  return (
    <Card className="mt-2">
      <CardContent className="p-0">
        {entries.length > 0 ? (
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>{t('common.date', 'Date')}</TableHead>
                <TableHead>{t('common.hours', 'Hours')}</TableHead>
                <TableHead className="hidden md:table-cell">{t('common.description', 'Description')}</TableHead>
                <TableHead>{t('common.status', 'Status')}</TableHead>
                <TableHead>{t('projects.billed', 'Billed')}</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {entries.map((te) => (
                <TableRow key={te.id}>
                  <TableCell className="font-mono">{te.date}</TableCell>
                  <TableCell className="font-mono">{(te.actual_minutes / 60).toFixed(1)}h</TableCell>
                  <TableCell className="hidden md:table-cell max-w-xs truncate">{te.description ?? '—'}</TableCell>
                  <TableCell>
                    <Badge variant={te.status === 'active' ? 'default' : 'secondary'}>{te.status}</Badge>
                  </TableCell>
                  <TableCell>
                    {te.billed
                      ? <Badge>{t('projects.billed', 'Billed')}</Badge>
                      : <Badge variant="outline">{t('projects.unbilled', 'Unbilled')}</Badge>
                    }
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        ) : (
          <p className="py-6 text-center text-sm text-muted-foreground">
            {t('projects.no_time_entries', 'No time entries for this project.')}
          </p>
        )}
      </CardContent>
    </Card>
  );
}
