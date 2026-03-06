import { useState, useEffect } from 'react';
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import { cn } from '@/lib/utils';
import { Download, Maximize2, Minimize2, Loader2, X } from 'lucide-react';
import { invoicesApi } from '@/api/invoices';
import { saveFile, openPdfWindow } from '@/lib/native';
import { isTauri } from '@/lib/platform';
import { useI18n } from '@/i18n';

interface PdfPreviewDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  invoiceId: string;
  invoiceNumber?: string;
}

export function PdfPreviewDialog({
  open,
  onOpenChange,
  invoiceId,
  invoiceNumber,
}: PdfPreviewDialogProps) {
  const { t } = useI18n();
  const displayNumber = invoiceNumber ?? t('invoices.draft_label', 'DRAFT');

  // In Tauri, open a native window instead of the dialog
  useEffect(() => {
    if (!open || !isTauri()) return;

    openPdfWindow(invoiceId, `${displayNumber} — PDF`);
    // Close the dialog trigger immediately since we opened a native window
    onOpenChange(false);
  }, [open, invoiceId, displayNumber, onOpenChange]);

  // Web-only dialog below — Tauri users never see this
  if (isTauri()) return null;

  return (
    <WebPdfPreviewDialog
      open={open}
      onOpenChange={onOpenChange}
      invoiceId={invoiceId}
      invoiceNumber={invoiceNumber}
      displayNumber={displayNumber}
    />
  );
}

/** Web-only modal PDF preview with expand/collapse. */
function WebPdfPreviewDialog({
  open,
  onOpenChange,
  invoiceId,
  displayNumber,
}: PdfPreviewDialogProps & { displayNumber: string }) {
  const { t } = useI18n();
  const [pdfUrl, setPdfUrl] = useState<string | null>(null);
  const [pdfBlob, setPdfBlob] = useState<Blob | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [expanded, setExpanded] = useState(false);

  useEffect(() => {
    if (!open) {
      if (pdfUrl) URL.revokeObjectURL(pdfUrl);
      setPdfUrl(null);
      setPdfBlob(null);
      setError(null);
      setExpanded(false);
      return;
    }

    setLoading(true);
    setError(null);
    invoicesApi
      .downloadPdf(invoiceId)
      .then((res) => {
        const blob = new Blob([res.data], { type: 'application/pdf' });
        setPdfBlob(blob);
        setPdfUrl(URL.createObjectURL(blob));
      })
      .catch(() => setError(t('invoices.download_pdf_failed', 'Failed to load PDF')))
      .finally(() => setLoading(false));
  }, [open, invoiceId, t]);

  async function handleDownload() {
    if (!pdfBlob) return;
    const filename = `invoice-${displayNumber}.pdf`;
    await saveFile(pdfBlob, filename);
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent
        showCloseButton={false}
        className={cn(
          'flex flex-col p-0 gap-0 transition-all duration-300 ease-out',
          expanded
            ? 'max-w-[calc(100vw-2rem)] max-h-[calc(100vh-2rem)] w-full h-full'
            : 'max-w-4xl h-[85vh]',
        )}
      >
        {/* Minimal top bar */}
        <DialogHeader className="flex-row items-center justify-between space-y-0 px-4 py-2.5 border-b bg-muted/30 flex-shrink-0">
          <DialogTitle className="text-sm font-medium text-muted-foreground">
            {displayNumber}
          </DialogTitle>
          <div className="flex items-center gap-1">
            <Button
              variant="ghost"
              size="icon"
              className="h-7 w-7"
              onClick={handleDownload}
              disabled={!pdfBlob}
            >
              <Download className="h-3.5 w-3.5" />
            </Button>
            <Button
              variant="ghost"
              size="icon"
              className="h-7 w-7"
              onClick={() => setExpanded((v) => !v)}
            >
              {expanded ? <Minimize2 className="h-3.5 w-3.5" /> : <Maximize2 className="h-3.5 w-3.5" />}
            </Button>
            <Button
              variant="ghost"
              size="icon"
              className="h-7 w-7"
              onClick={() => onOpenChange(false)}
            >
              <X className="h-3.5 w-3.5" />
            </Button>
          </div>
        </DialogHeader>

        {/* PDF viewport — dark background like professional viewers */}
        <div className="flex-1 min-h-0 bg-zinc-800 dark:bg-zinc-900 overflow-hidden">
          {loading && (
            <div className="flex flex-col items-center justify-center h-full gap-3">
              <Loader2 className="h-8 w-8 animate-spin text-zinc-400" />
              <p className="text-sm text-zinc-400">{t('invoices.generating_pdf', 'Generating PDF...')}</p>
            </div>
          )}
          {error && (
            <div className="flex items-center justify-center h-full">
              <p className="text-sm text-red-400">{error}</p>
            </div>
          )}
          {pdfUrl && (
            <iframe
              src={`${pdfUrl}#toolbar=0&navpanes=0`}
              className="h-full w-full border-0"
              title="Invoice PDF"
            />
          )}
        </div>
      </DialogContent>
    </Dialog>
  );
}
