import { useNavigate } from 'react-router-dom';
import { Button } from '@/components/ui/button';
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { useConvertDocument } from '@/hooks/useDocumentsApi';
import { toast } from 'sonner';
import { Badge } from '@/components/ui/badge';
import { useI18n } from '@/i18n';

interface DocumentConvertDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  documentId: string;
  documentTitle: string;
  sourceType: string;
  targetType: string;
}

export function DocumentConvertDialog({
  open,
  onOpenChange,
  documentId,
  documentTitle,
  sourceType,
  targetType,
}: DocumentConvertDialogProps) {
  const { t } = useI18n();
  const navigate = useNavigate();
  const convertDoc = useConvertDocument();

  function handleConvert() {
    convertDoc.mutate(
      { id: documentId, target_type: targetType },
      {
        onSuccess: (res) => {
          toast.success(`${t('documents.convert.converted_to', 'Converted to')} ${targetType.toUpperCase()}`);
          onOpenChange(false);
          navigate(`/documents/${res.data.id}`);
        },
        onError: () => toast.error(t('documents.convert.failed', 'Failed to convert document')),
      },
    );
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{t('documents.convert.title', 'Convert Document')}</DialogTitle>
        </DialogHeader>
        <div className="space-y-4">
          <p className="text-sm text-muted-foreground">
            {t('documents.convert.description', 'Convert this document to a new type. The original document will remain unchanged.')}
          </p>
          <div className="rounded-md border p-3">
            <p className="text-sm font-medium">{documentTitle}</p>
            <div className="mt-2 flex items-center gap-2">
              <Badge variant="secondary">{sourceType.toUpperCase()}</Badge>
              <span className="text-muted-foreground">→</span>
              <Badge>{targetType.toUpperCase()}</Badge>
            </div>
          </div>
          <div className="flex gap-2 justify-end">
            <Button variant="outline" onClick={() => onOpenChange(false)}>
              {t('common.cancel', 'Cancel')}
            </Button>
            <Button onClick={handleConvert} disabled={convertDoc.isPending}>
              {convertDoc.isPending
                ? t('documents.convert.converting', 'Converting...')
                : t('documents.convert.action', 'Convert')}
            </Button>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
