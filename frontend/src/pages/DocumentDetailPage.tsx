import { useParams, useNavigate, Link } from 'react-router-dom';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Skeleton } from '@/components/ui/skeleton';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import { useDocument } from '@/hooks/useDocumentsApi';
import { DocumentStatusActions } from '@/components/document/DocumentStatusActions';
import { formatAmount } from '@/lib/format';

const typeVariant: Record<string, 'default' | 'secondary' | 'outline'> = {
  quote: 'secondary',
  offer: 'secondary',
  sow: 'default',
  contract: 'outline',
};

const statusVariant: Record<string, 'default' | 'secondary' | 'destructive' | 'outline'> = {
  draft: 'secondary',
  sent: 'default',
  accepted: 'outline',
  signed: 'outline',
  rejected: 'destructive',
};

export function DocumentDetailPage() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const { data, isLoading } = useDocument(id);

  if (isLoading) {
    return (
      <div className="space-y-4">
        <Skeleton className="h-8 w-48" />
        <Skeleton className="h-64 w-full" />
      </div>
    );
  }

  if (!data) {
    return <p className="text-center text-muted-foreground">Document not found.</p>;
  }

  return (
    <div className="space-y-4">
      <div className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <div className="flex items-center gap-2">
            <h2 className="text-lg font-semibold">{data.title}</h2>
            <Badge variant={typeVariant[data.doc_type] ?? 'outline'}>
              {data.doc_type}
            </Badge>
            <Badge variant={statusVariant[data.status] ?? 'outline'}>
              {data.status}
            </Badge>
          </div>
          <p className="text-sm text-muted-foreground">
            {data.doc_number && <span className="font-mono">{data.doc_number} — </span>}
            {data.contact_name ?? data.contact_id}
            {data.project_name && ` — ${data.project_name}`}
          </p>
        </div>
        <DocumentStatusActions
          id={id!}
          docType={data.doc_type}
          status={data.status}
          title={data.title}
          onDeleted={() => navigate('/documents')}
        />
      </div>

      <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
        <InfoCard label="Status">
          <Badge variant={statusVariant[data.status] ?? 'outline'}>{data.status}</Badge>
        </InfoCard>
        <InfoCard label="Issued">{data.issued_at ?? 'Not issued'}</InfoCard>
        <InfoCard label="Valid Until">{data.valid_until ?? 'N/A'}</InfoCard>
        <InfoCard label="Total">
          <span className="font-mono font-bold">{formatAmount(data.total)}</span>
        </InfoCard>
      </div>

      {data.converted_from && (
        <Card>
          <CardContent className="py-3">
            <p className="text-sm">
              Converted from:{' '}
              <Link
                to={`/documents/${data.converted_from}`}
                className="text-primary hover:underline font-mono"
              >
                {data.converted_from}
              </Link>
            </p>
          </CardContent>
        </Card>
      )}

      <Card>
        <CardHeader>
          <CardTitle className="text-base">Line Items</CardTitle>
        </CardHeader>
        <CardContent className="p-0">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead className="w-12">#</TableHead>
                <TableHead>Description</TableHead>
                <TableHead className="text-right">Qty</TableHead>
                <TableHead className="hidden sm:table-cell">Unit</TableHead>
                <TableHead className="text-right">Unit Price</TableHead>
                <TableHead className="text-right hidden md:table-cell">Discount</TableHead>
                <TableHead className="text-right">Total</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {data.lines.map((line) => (
                <TableRow key={line.id}>
                  <TableCell className="text-muted-foreground">{line.position}</TableCell>
                  <TableCell>{line.description}</TableCell>
                  <TableCell className="text-right font-mono text-sm">
                    {formatAmount(line.quantity)}
                  </TableCell>
                  <TableCell className="hidden sm:table-cell">{line.unit ?? ''}</TableCell>
                  <TableCell className="text-right font-mono text-sm">
                    {formatAmount(line.unit_price)}
                  </TableCell>
                  <TableCell className="text-right font-mono text-sm hidden md:table-cell">
                    {formatAmount(line.discount_pct)}%
                  </TableCell>
                  <TableCell className="text-right font-mono text-sm font-medium">
                    {formatAmount(line.total)}
                  </TableCell>
                </TableRow>
              ))}
              <TableRow className="bg-muted/50">
                <TableCell colSpan={6} className="text-right font-medium">
                  Subtotal
                </TableCell>
                <TableCell className="text-right font-mono text-sm font-medium">
                  {formatAmount(data.subtotal)}
                </TableCell>
              </TableRow>
              <TableRow className="bg-muted/50">
                <TableCell colSpan={6} className="text-right font-medium">
                  VAT ({formatAmount(data.vat_rate)}%)
                </TableCell>
                <TableCell className="text-right font-mono text-sm font-medium">
                  {formatAmount(data.vat_amount)}
                </TableCell>
              </TableRow>
              <TableRow className="bg-muted/50 font-bold">
                <TableCell colSpan={6} className="text-right">Total</TableCell>
                <TableCell className="text-right font-mono text-sm">
                  {formatAmount(data.total)}
                </TableCell>
              </TableRow>
            </TableBody>
          </Table>
        </CardContent>
      </Card>
    </div>
  );
}

function InfoCard({ label, children }: { label: string; children: React.ReactNode }) {
  return (
    <Card>
      <CardContent className="py-3">
        <p className="text-xs text-muted-foreground">{label}</p>
        <div className="mt-1">{children}</div>
      </CardContent>
    </Card>
  );
}
