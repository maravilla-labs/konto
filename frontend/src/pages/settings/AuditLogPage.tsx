import { useState } from 'react';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Badge } from '@/components/ui/badge';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { Skeleton } from '@/components/ui/skeleton';
import { StickyToolbar } from '@/components/ui/sticky-toolbar';
import { useAuditLogs } from '@/hooks/useAuditApi';
import { useI18n } from '@/i18n';
import { ChevronLeft, ChevronRight, Eye } from 'lucide-react';
import type { AuditLogParams, AuditLog } from '@/types/audit';

const entityTypes = [
  '', 'invoice', 'contact', 'account', 'journal_entry', 'project',
  'time_entry', 'company_settings', 'email_settings', 'document',
  'template', 'user', 'bank_account', 'fiscal_year', 'exchange_rate',
];
const actions = [
  '', 'create', 'update', 'delete', 'send', 'pay', 'cancel',
  'email', 'upload_logo', 'test_email', 'close',
];

const actionVariant: Record<string, 'default' | 'secondary' | 'destructive' | 'outline'> = {
  create: 'default',
  update: 'secondary',
  delete: 'destructive',
  send: 'outline',
  pay: 'outline',
  cancel: 'destructive',
  email: 'outline',
};

export function AuditLogPage() {
  const { t } = useI18n();
  const [filters, setFilters] = useState<AuditLogParams>({
    page: 1,
    per_page: 25,
  });
  const { data, isLoading } = useAuditLogs(filters);
  const [detail, setDetail] = useState<AuditLog | null>(null);

  function setFilter(field: keyof AuditLogParams, value: string | number) {
    setFilters({ ...filters, [field]: value || undefined, page: 1 });
  }

  function goToPage(page: number) {
    setFilters({ ...filters, page });
  }

  if (isLoading && !data) {
    return (
      <div className="space-y-4">
        <Skeleton className="h-8 w-48" />
        <Skeleton className="h-64 w-full" />
      </div>
    );
  }

  const logs = data?.data ?? [];
  const totalPages = data?.total_pages ?? 1;
  const currentPage = filters.page ?? 1;

  return (
    <div className="space-y-4">
      <StickyToolbar>
        <Select
          value={filters.entity_type ?? ''}
          onValueChange={(v) => setFilter('entity_type', v)}
        >
          <SelectTrigger className="h-8 w-40 text-sm">
            <SelectValue placeholder={t('audit.all_types', 'All types')} />
          </SelectTrigger>
          <SelectContent>
            {entityTypes.map((et) => (
              <SelectItem key={et || '__all'} value={et}>
                {et || t('audit.all_types', 'All types')}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
        <Select
          value={filters.action ?? ''}
          onValueChange={(v) => setFilter('action', v)}
        >
          <SelectTrigger className="h-8 w-36 text-sm">
            <SelectValue placeholder={t('audit.all_actions', 'All actions')} />
          </SelectTrigger>
          <SelectContent>
            {actions.map((a) => (
              <SelectItem key={a || '__all'} value={a}>
                {a || t('audit.all_actions', 'All actions')}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
        <Input
          type="date"
          value={filters.from ?? ''}
          onChange={(e) => setFilter('from', e.target.value)}
          className="h-8 w-36 text-sm"
        />
        <span className="text-xs text-muted-foreground">&ndash;</span>
        <Input
          type="date"
          value={filters.to ?? ''}
          onChange={(e) => setFilter('to', e.target.value)}
          className="h-8 w-36 text-sm"
        />
      </StickyToolbar>

      <Card>
        <CardContent className="p-0">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Timestamp</TableHead>
                <TableHead>Action</TableHead>
                <TableHead>Entity Type</TableHead>
                <TableHead className="hidden sm:table-cell">Entity ID</TableHead>
                <TableHead className="hidden md:table-cell">User</TableHead>
                <TableHead className="w-12" />
              </TableRow>
            </TableHeader>
            <TableBody>
              {logs.length === 0 && (
                <TableRow>
                  <TableCell colSpan={6} className="text-center text-muted-foreground">
                    No audit logs found
                  </TableCell>
                </TableRow>
              )}
              {logs.map((log) => (
                <TableRow key={log.id}>
                  <TableCell className="font-mono text-xs">
                    {formatTimestamp(log.created_at)}
                  </TableCell>
                  <TableCell>
                    <Badge variant={actionVariant[log.action] ?? 'secondary'}>
                      {log.action}
                    </Badge>
                  </TableCell>
                  <TableCell className="text-sm">{log.entity_type}</TableCell>
                  <TableCell className="hidden font-mono text-xs sm:table-cell">
                    {log.entity_id ? truncateId(log.entity_id) : '—'}
                  </TableCell>
                  <TableCell className="hidden text-xs md:table-cell">
                    {log.user_id ? truncateId(log.user_id) : 'system'}
                  </TableCell>
                  <TableCell>
                    {(log.old_values || log.new_values) && (
                      <Button
                        variant="ghost"
                        size="sm"
                        onClick={() => setDetail(log)}
                      >
                        <Eye className="h-3.5 w-3.5" />
                      </Button>
                    )}
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </CardContent>
      </Card>

      {totalPages > 1 && (
        <div className="flex items-center justify-between">
          <p className="text-sm text-muted-foreground">
            Page {currentPage} of {totalPages} ({data?.total ?? 0} entries)
          </p>
          <div className="flex gap-1">
            <Button
              variant="outline"
              size="sm"
              onClick={() => goToPage(currentPage - 1)}
              disabled={currentPage <= 1}
            >
              <ChevronLeft className="h-4 w-4" />
            </Button>
            <Button
              variant="outline"
              size="sm"
              onClick={() => goToPage(currentPage + 1)}
              disabled={currentPage >= totalPages}
            >
              <ChevronRight className="h-4 w-4" />
            </Button>
          </div>
        </div>
      )}

      <Dialog open={!!detail} onOpenChange={() => setDetail(null)}>
        <DialogContent className="max-w-2xl">
          <DialogHeader>
            <DialogTitle>
              Audit Detail — {detail?.action} {detail?.entity_type}
            </DialogTitle>
          </DialogHeader>
          {detail && <AuditDetailView log={detail} />}
        </DialogContent>
      </Dialog>
    </div>
  );
}

function AuditDetailView({ log }: { log: AuditLog }) {
  return (
    <div className="space-y-4 text-sm">
      <div className="grid gap-2 sm:grid-cols-2">
        <div>
          <Label className="text-xs text-muted-foreground">Timestamp</Label>
          <p>{log.created_at}</p>
        </div>
        <div>
          <Label className="text-xs text-muted-foreground">User ID</Label>
          <p className="font-mono text-xs">{log.user_id ?? 'system'}</p>
        </div>
        <div>
          <Label className="text-xs text-muted-foreground">Entity ID</Label>
          <p className="font-mono text-xs">{log.entity_id ?? '—'}</p>
        </div>
      </div>
      {log.old_values && (
        <div>
          <Label className="text-xs text-muted-foreground">Old Values</Label>
          <pre className="mt-1 max-h-48 overflow-auto rounded bg-muted p-2 text-xs">
            {formatJson(log.old_values)}
          </pre>
        </div>
      )}
      {log.new_values && (
        <div>
          <Label className="text-xs text-muted-foreground">New Values</Label>
          <pre className="mt-1 max-h-48 overflow-auto rounded bg-muted p-2 text-xs">
            {formatJson(log.new_values)}
          </pre>
        </div>
      )}
    </div>
  );
}

function formatJson(s: string): string {
  try {
    return JSON.stringify(JSON.parse(s), null, 2);
  } catch {
    return s;
  }
}

function formatTimestamp(s: string): string {
  return s.replace('T', ' ');
}

function truncateId(id: string): string {
  return id.length > 12 ? `${id.slice(0, 8)}...` : id;
}
