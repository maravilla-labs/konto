import { useParams, useNavigate, useLocation } from 'react-router-dom';
import { useRef } from 'react';
import {
  useJournalDetail,
  usePostJournal,
  useReverseJournal,
  useJournalAttachments,
  useUploadJournalAttachment,
  useDeleteJournalAttachment,
  useAccountTreeWithBalances,
} from '@/hooks/useApi';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
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
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from '@/components/ui/alert-dialog';
import { ArrowLeft, CheckCircle, Undo2, Upload, Paperclip } from 'lucide-react';
import { toast } from 'sonner';
import { extractErrorMessage } from '@/api/client';
import type { AccountTreeWithBalance } from '@/types/accounts';
import { AttachmentCard } from '@/components/journal/AttachmentViewer';
import { useMemo } from 'react';

const statusColors: Record<string, string> = {
  draft: 'bg-yellow-100 text-yellow-800',
  posted: 'bg-green-100 text-green-800',
  reversed: 'bg-red-100 text-red-800',
};

function flattenTree(nodes: AccountTreeWithBalance[]): Map<string, AccountTreeWithBalance> {
  const map = new Map<string, AccountTreeWithBalance>();
  for (const n of nodes) {
    map.set(n.id, n);
    if (n.children.length > 0) {
      for (const [k, v] of flattenTree(n.children)) map.set(k, v);
    }
  }
  return map;
}

export function JournalDetailPage() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const location = useLocation();
  const hasHistory = location.key !== 'default';
  const goBack = () => hasHistory ? navigate(-1) : navigate('/journal');
  const { data: detail, isLoading } = useJournalDetail(id);
  const { data: attachments } = useJournalAttachments(id);
  const { data: tree } = useAccountTreeWithBalances();
  const postJournal = usePostJournal();
  const reverseJournal = useReverseJournal();
  const uploadAttachment = useUploadJournalAttachment();
  const deleteAttachment = useDeleteJournalAttachment();
  const fileInputRef = useRef<HTMLInputElement>(null);

  const accountMap = useMemo(() => (tree ? flattenTree(tree) : new Map()), [tree]);

  if (isLoading) {
    return (
      <div className="space-y-4">
        <Skeleton className="h-8 w-48" />
        <Skeleton className="h-64 w-full" />
      </div>
    );
  }

  if (!detail) {
    return <p className="text-muted-foreground">Journal entry not found.</p>;
  }

  const { entry, lines } = detail;
  const totalDebit = lines.reduce((s, l) => s + parseFloat(l.debit_amount), 0);
  const totalCredit = lines.reduce((s, l) => s + parseFloat(l.credit_amount), 0);

  function handlePost() {
    postJournal.mutate(entry.id, {
      onSuccess: () => toast.success('Entry posted'),
      onError: (err) => toast.error(extractErrorMessage(err)),
    });
  }

  function handleReverse() {
    reverseJournal.mutate(entry.id, {
      onSuccess: () => toast.success('Entry reversed'),
      onError: (err) => toast.error(extractErrorMessage(err)),
    });
  }

  function handleFileUpload(e: React.ChangeEvent<HTMLInputElement>) {
    const file = e.target.files?.[0];
    if (!file) return;
    uploadAttachment.mutate(
      { entryId: entry.id, file },
      {
        onSuccess: () => toast.success('Attachment uploaded'),
        onError: () => toast.error('Upload failed'),
      }
    );
    e.target.value = '';
  }

  function handleDeleteAttachment(attId: string) {
    deleteAttachment.mutate(attId, {
      onSuccess: () => toast.success('Attachment deleted'),
      onError: () => toast.error('Delete failed'),
    });
  }

  function resolveAccountName(accountId: string): string {
    const acc = accountMap.get(accountId);
    return acc ? `${acc.number} ${acc.name}` : accountId;
  }

  return (
    <div className="space-y-4">
      <div className="flex items-center gap-3">
        <Button variant="ghost" size="icon" onClick={goBack}>
          <ArrowLeft className="h-4 w-4" />
        </Button>
        <div className="flex-1">
          <h2 className="text-lg font-semibold">{entry.description}</h2>
          <p className="text-sm text-muted-foreground">
            {entry.date} {entry.reference && `· ${entry.reference}`}
          </p>
        </div>
        <Badge variant="secondary" className={statusColors[entry.status] ?? ''}>
          {entry.status}
        </Badge>
        {entry.status === 'draft' && (
          <AlertDialog>
            <AlertDialogTrigger asChild>
              <Button size="sm" variant="outline">
                <CheckCircle className="mr-1 h-4 w-4" /> Post
              </Button>
            </AlertDialogTrigger>
            <AlertDialogContent>
              <AlertDialogHeader>
                <AlertDialogTitle>Post Entry?</AlertDialogTitle>
                <AlertDialogDescription>
                  This will finalize the journal entry.
                </AlertDialogDescription>
              </AlertDialogHeader>
              <AlertDialogFooter>
                <AlertDialogCancel>Cancel</AlertDialogCancel>
                <AlertDialogAction onClick={handlePost}>Post</AlertDialogAction>
              </AlertDialogFooter>
            </AlertDialogContent>
          </AlertDialog>
        )}
        {entry.status === 'posted' && (
          <AlertDialog>
            <AlertDialogTrigger asChild>
              <Button size="sm" variant="outline">
                <Undo2 className="mr-1 h-4 w-4" /> Reverse
              </Button>
            </AlertDialogTrigger>
            <AlertDialogContent>
              <AlertDialogHeader>
                <AlertDialogTitle>Reverse Entry?</AlertDialogTitle>
                <AlertDialogDescription>
                  A reversing entry will be created.
                </AlertDialogDescription>
              </AlertDialogHeader>
              <AlertDialogFooter>
                <AlertDialogCancel>Cancel</AlertDialogCancel>
                <AlertDialogAction onClick={handleReverse}>Reverse</AlertDialogAction>
              </AlertDialogFooter>
            </AlertDialogContent>
          </AlertDialog>
        )}
      </div>

      {/* Lines */}
      <Card>
        <CardHeader className="pb-3">
          <CardTitle className="text-sm font-medium text-muted-foreground">
            Journal Lines
          </CardTitle>
        </CardHeader>
        <CardContent className="p-0">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Account</TableHead>
                <TableHead className="text-right">Debit</TableHead>
                <TableHead className="text-right">Credit</TableHead>
                <TableHead>Description</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {lines.map((line) => (
                <TableRow key={line.id}>
                  <TableCell className="font-mono text-sm">
                    {resolveAccountName(line.account_id)}
                  </TableCell>
                  <TableCell className="text-right font-mono">
                    {parseFloat(line.debit_amount) > 0
                      ? parseFloat(line.debit_amount).toFixed(2)
                      : ''}
                  </TableCell>
                  <TableCell className="text-right font-mono">
                    {parseFloat(line.credit_amount) > 0
                      ? parseFloat(line.credit_amount).toFixed(2)
                      : ''}
                  </TableCell>
                  <TableCell className="text-sm text-muted-foreground">
                    {line.description}
                  </TableCell>
                </TableRow>
              ))}
              <TableRow className="font-semibold border-t-2">
                <TableCell>Total</TableCell>
                <TableCell className="text-right font-mono">{totalDebit.toFixed(2)}</TableCell>
                <TableCell className="text-right font-mono">{totalCredit.toFixed(2)}</TableCell>
                <TableCell />
              </TableRow>
            </TableBody>
          </Table>
        </CardContent>
      </Card>

      {/* Attachments */}
      <Card>
        <CardHeader className="flex flex-row items-center justify-between pb-3">
          <CardTitle className="text-sm font-medium text-muted-foreground">
            <Paperclip className="mr-1 inline h-4 w-4" />
            Attachments ({attachments?.length ?? 0})
          </CardTitle>
          <div>
            <input
              type="file"
              ref={fileInputRef}
              className="hidden"
              onChange={handleFileUpload}
            />
            <Button
              size="sm"
              variant="outline"
              onClick={() => fileInputRef.current?.click()}
              disabled={uploadAttachment.isPending}
            >
              <Upload className="mr-1 h-4 w-4" /> Upload
            </Button>
          </div>
        </CardHeader>
        <CardContent>
          {attachments && attachments.length > 0 ? (
            <div className="space-y-2">
              {attachments.map((att) => (
                <AttachmentCard
                  key={att.id}
                  attachment={att}
                  onDelete={handleDeleteAttachment}
                />
              ))}
            </div>
          ) : (
            <p className="text-sm text-muted-foreground">No attachments yet.</p>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
