import { useState, useEffect, useRef } from 'react';
import { useParams } from 'react-router-dom';
import { invoicesApi } from '@/api/invoices';
import { Loader2 } from 'lucide-react';

/**
 * Minimal full-screen PDF viewer page.
 * Used by Tauri to show invoice PDFs in a separate native window.
 */
export function PdfViewPage() {
  const { id } = useParams<{ id: string }>();
  const [pdfUrl, setPdfUrl] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const urlRef = useRef<string | null>(null);

  // Mark this as a child window so CSS skips rounded corners / transparency
  useEffect(() => {
    document.documentElement.classList.add('tauri-child-window');
    return () => document.documentElement.classList.remove('tauri-child-window');
  }, []);

  useEffect(() => {
    if (!id) return;

    invoicesApi
      .downloadPdf(id)
      .then((res) => {
        const blob = new Blob([res.data], { type: 'application/pdf' });
        const url = URL.createObjectURL(blob);
        urlRef.current = url;
        setPdfUrl(url);
      })
      .catch((e) => setError(e?.message ?? 'Failed to load PDF'));

    return () => {
      if (urlRef.current) URL.revokeObjectURL(urlRef.current);
    };
  }, [id]);

  return (
    <div className="w-screen h-screen bg-zinc-800 overflow-hidden">
      {!pdfUrl && !error && (
        <div className="flex items-center justify-center h-full gap-3">
          <Loader2 className="h-8 w-8 animate-spin text-zinc-400" />
          <span className="text-sm text-zinc-400 font-sans">Loading PDF...</span>
        </div>
      )}
      {error && (
        <div className="flex items-center justify-center h-full">
          <span className="text-sm text-red-400 font-sans">{error}</span>
        </div>
      )}
      {pdfUrl && (
        <embed
          src={`${pdfUrl}#view=FitH&toolbar=1`}
          type="application/pdf"
          className="w-full h-full"
        />
      )}
    </div>
  );
}
