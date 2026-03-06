import { useState, useEffect } from 'react';
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import {
  Download,
  FileText,
  FileImage,
  FileSpreadsheet,
  File,
  Eye,
} from 'lucide-react';
import type { JournalAttachment } from '@/types/journal';

function previewUrl(id: string) {
  return `/api/v1/journal/attachments/${id}/preview`;
}

function downloadUrl(id: string) {
  return `/api/v1/journal/attachments/${id}/download`;
}

function getAuthHeaders(): HeadersInit {
  const tokens = localStorage.getItem('konto_tokens');
  if (!tokens) return {};
  const { access_token } = JSON.parse(tokens);
  return { Authorization: `Bearer ${access_token}` };
}

function isImage(mime: string) {
  return mime.startsWith('image/');
}

function isPdf(mime: string) {
  return mime === 'application/pdf';
}

function isText(mime: string) {
  return (
    mime.startsWith('text/') ||
    mime === 'application/json' ||
    mime === 'application/xml'
  );
}

function formatSize(bytes: number) {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

function useAuthBlobUrl(url: string | null): string | null {
  const [blobUrl, setBlobUrl] = useState<string | null>(null);

  useEffect(() => {
    if (!url) { setBlobUrl(null); return; }
    let revoked = false;
    fetch(url, { headers: getAuthHeaders() })
      .then((r) => {
        if (!r.ok) throw new Error('Failed to load');
        return r.blob();
      })
      .then((blob) => {
        if (revoked) return;
        setBlobUrl(URL.createObjectURL(blob));
      })
      .catch(() => {
        if (!revoked) setBlobUrl(null);
      });
    return () => {
      revoked = true;
      setBlobUrl((prev) => {
        if (prev) URL.revokeObjectURL(prev);
        return null;
      });
    };
  }, [url]);

  return blobUrl;
}

function handleAuthDownload(id: string, fileName: string) {
  fetch(downloadUrl(id), { headers: getAuthHeaders() })
    .then((r) => {
      if (!r.ok) throw new Error('Download failed');
      return r.blob();
    })
    .then((blob) => {
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = fileName;
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      URL.revokeObjectURL(url);
    });
}

function FileIcon({ mime }: { mime: string }) {
  const cls = 'h-5 w-5';
  if (isPdf(mime)) return <FileText className={`${cls} text-red-500`} />;
  if (isImage(mime)) return <FileImage className={`${cls} text-blue-500`} />;
  if (mime.includes('spreadsheet') || mime.includes('csv'))
    return <FileSpreadsheet className={`${cls} text-green-500`} />;
  if (isText(mime)) return <FileText className={`${cls} text-amber-500`} />;
  return <File className={`${cls} text-muted-foreground`} />;
}

export function AttachmentCard({
  attachment,
  onDelete,
}: {
  attachment: JournalAttachment;
  onDelete: (id: string) => void;
}) {
  const [previewOpen, setPreviewOpen] = useState(false);
  const canPreview = isImage(attachment.mime_type) || isPdf(attachment.mime_type) || isText(attachment.mime_type);
  const thumbUrl = useAuthBlobUrl(isImage(attachment.mime_type) ? previewUrl(attachment.id) : null);

  return (
    <>
      <div className="group flex items-center gap-3 rounded-xl border bg-card px-4 py-3 transition-colors hover:bg-muted/40">
        <div className="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-muted/60">
          {thumbUrl ? (
            <img
              src={thumbUrl}
              alt={attachment.file_name}
              className="h-10 w-10 rounded-lg object-cover"
            />
          ) : (
            <FileIcon mime={attachment.mime_type} />
          )}
        </div>

        <div className="flex-1 min-w-0">
          <p className="truncate text-sm font-medium">{attachment.file_name}</p>
          <p className="text-xs text-muted-foreground">
            {formatSize(attachment.file_size)}
            {' · '}
            {attachment.mime_type.split('/').pop()?.toUpperCase()}
          </p>
        </div>

        <div className="flex items-center gap-1 opacity-0 transition-opacity group-hover:opacity-100">
          {canPreview && (
            <Button
              variant="ghost"
              size="icon"
              className="h-8 w-8"
              onClick={() => setPreviewOpen(true)}
              title="Preview"
            >
              <Eye className="h-3.5 w-3.5" />
            </Button>
          )}
          <Button
            variant="ghost"
            size="icon"
            className="h-8 w-8"
            onClick={() => handleAuthDownload(attachment.id, attachment.file_name)}
            title="Download"
          >
            <Download className="h-3.5 w-3.5" />
          </Button>
          <Button
            variant="ghost"
            size="icon"
            className="h-8 w-8 text-muted-foreground hover:text-destructive"
            onClick={() => onDelete(attachment.id)}
            title="Delete"
          >
            <svg className="h-3.5 w-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
              <path d="M3 6h18" /><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" /><path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" />
            </svg>
          </Button>
        </div>
      </div>

      {canPreview && (
        <PreviewDialog
          attachment={attachment}
          open={previewOpen}
          onOpenChange={setPreviewOpen}
        />
      )}
    </>
  );
}

function PreviewDialog({
  attachment,
  open,
  onOpenChange,
}: {
  attachment: JournalAttachment;
  open: boolean;
  onOpenChange: (open: boolean) => void;
}) {
  const imgUrl = useAuthBlobUrl(open && isImage(attachment.mime_type) ? previewUrl(attachment.id) : null);
  const pdfUrl = useAuthBlobUrl(open && isPdf(attachment.mime_type) ? previewUrl(attachment.id) : null);

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-4xl max-h-[90vh] flex flex-col">
        <DialogHeader className="flex flex-row items-center justify-between">
          <div className="flex items-center gap-2 min-w-0 flex-1">
            <FileIcon mime={attachment.mime_type} />
            <DialogTitle className="truncate text-sm">
              {attachment.file_name}
            </DialogTitle>
            <span className="shrink-0 text-xs text-muted-foreground">
              {formatSize(attachment.file_size)}
            </span>
          </div>
          <Button
            variant="outline"
            size="sm"
            className="shrink-0 ml-2"
            onClick={() => handleAuthDownload(attachment.id, attachment.file_name)}
          >
            <Download className="mr-1 h-3.5 w-3.5" /> Download
          </Button>
        </DialogHeader>

        <div className="flex-1 overflow-auto rounded-lg border bg-muted/30 min-h-[400px]">
          {isImage(attachment.mime_type) && (
            <div className="flex items-center justify-center p-4 h-full">
              {imgUrl ? (
                <img
                  src={imgUrl}
                  alt={attachment.file_name}
                  className="max-h-[70vh] max-w-full rounded-lg object-contain"
                />
              ) : (
                <p className="text-sm text-muted-foreground">Loading...</p>
              )}
            </div>
          )}

          {isPdf(attachment.mime_type) && (
            pdfUrl ? (
              <iframe
                src={pdfUrl}
                title={attachment.file_name}
                className="h-[70vh] w-full rounded-lg"
              />
            ) : (
              <div className="flex items-center justify-center p-8 text-sm text-muted-foreground">
                Loading...
              </div>
            )
          )}

          {isText(attachment.mime_type) && (
            <TextPreview attachmentId={attachment.id} />
          )}
        </div>
      </DialogContent>
    </Dialog>
  );
}

function TextPreview({ attachmentId }: { attachmentId: string }) {
  const [content, setContent] = useState<string | null>(null);
  const [error, setError] = useState(false);

  useEffect(() => {
    fetch(previewUrl(attachmentId), { headers: getAuthHeaders() })
      .then((r) => {
        if (!r.ok) throw new Error('Failed to load');
        return r.text();
      })
      .then(setContent)
      .catch(() => setError(true));
  }, [attachmentId]);

  if (error) {
    return (
      <div className="flex items-center justify-center p-8 text-sm text-muted-foreground">
        Failed to load preview.
      </div>
    );
  }

  if (content === null) {
    return (
      <div className="flex items-center justify-center p-8 text-sm text-muted-foreground">
        Loading...
      </div>
    );
  }

  return (
    <pre className="whitespace-pre-wrap break-words p-4 text-sm font-mono leading-relaxed">
      {content}
    </pre>
  );
}
