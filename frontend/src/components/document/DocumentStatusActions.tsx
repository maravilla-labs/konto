import { useState } from 'react';
import { Link } from 'react-router-dom';
import { Button } from '@/components/ui/button';
import {
  useSendDocument,
  useAcceptDocument,
  useRejectDocument,
  useDeleteDocument,
} from '@/hooks/useDocumentsApi';
import { DocumentConvertDialog } from './DocumentConvertDialog';
import { toast } from 'sonner';
import { Pencil, Send, CheckCircle, XCircle, Trash2, RefreshCw } from 'lucide-react';

interface DocumentStatusActionsProps {
  id: string;
  docType: string;
  status: string;
  title: string;
  onDeleted: () => void;
}

export function DocumentStatusActions({
  id,
  docType,
  status,
  title,
  onDeleted,
}: DocumentStatusActionsProps) {
  const sendDoc = useSendDocument();
  const acceptDoc = useAcceptDocument();
  const rejectDoc = useRejectDocument();
  const deleteDoc = useDeleteDocument();
  const [convertOpen, setConvertOpen] = useState(false);

  function handleSend() {
    sendDoc.mutate(id, {
      onSuccess: () => toast.success('Document sent'),
      onError: () => toast.error('Failed to send document'),
    });
  }

  function handleAccept() {
    acceptDoc.mutate(id, {
      onSuccess: () => toast.success('Document accepted'),
      onError: () => toast.error('Failed to accept document'),
    });
  }

  function handleReject() {
    rejectDoc.mutate(id, {
      onSuccess: () => toast.success('Document rejected'),
      onError: () => toast.error('Failed to reject document'),
    });
  }

  function handleDelete() {
    deleteDoc.mutate(id, {
      onSuccess: () => {
        toast.success('Document deleted');
        onDeleted();
      },
      onError: () => toast.error('Failed to delete document'),
    });
  }

  const targetType = getConvertTarget(docType);

  return (
    <div className="flex flex-wrap gap-2">
      {status === 'draft' && (
        <>
          <Button asChild variant="outline" size="sm">
            <Link to={`/documents/${id}/edit`}>
              <Pencil className="mr-1 h-3.5 w-3.5" /> Edit
            </Link>
          </Button>
          <Button size="sm" onClick={handleSend} disabled={sendDoc.isPending}>
            <Send className="mr-1 h-3.5 w-3.5" /> Send
          </Button>
          <Button
            variant="destructive"
            size="sm"
            onClick={handleDelete}
            disabled={deleteDoc.isPending}
          >
            <Trash2 className="mr-1 h-3.5 w-3.5" /> Delete
          </Button>
        </>
      )}

      {status === 'sent' && (
        <>
          <Button size="sm" onClick={handleAccept} disabled={acceptDoc.isPending}>
            <CheckCircle className="mr-1 h-3.5 w-3.5" /> Accept
          </Button>
          <Button
            variant="destructive"
            size="sm"
            onClick={handleReject}
            disabled={rejectDoc.isPending}
          >
            <XCircle className="mr-1 h-3.5 w-3.5" /> Reject
          </Button>
        </>
      )}

      {(status === 'accepted' || status === 'signed') && targetType && (
        <Button size="sm" variant="outline" onClick={() => setConvertOpen(true)}>
          <RefreshCw className="mr-1 h-3.5 w-3.5" /> Convert to {targetType}
        </Button>
      )}

      {targetType && (
        <DocumentConvertDialog
          open={convertOpen}
          onOpenChange={setConvertOpen}
          documentId={id}
          documentTitle={title}
          sourceType={docType}
          targetType={targetType}
        />
      )}
    </div>
  );
}

function getConvertTarget(docType: string): string | null {
  switch (docType) {
    case 'quote':
    case 'offer':
      return 'sow';
    case 'sow':
      return 'contract';
    default:
      return null;
  }
}
