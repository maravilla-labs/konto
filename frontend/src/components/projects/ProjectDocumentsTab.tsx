import { useRef } from 'react';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import {
  Table, TableBody, TableCell, TableHead, TableHeader, TableRow,
} from '@/components/ui/table';
import { useProjectDocuments, useUploadProjectDocument, useDeleteProjectDocument } from '@/hooks/useApi';
import { projectDocumentsApi } from '@/api/project-documents';
import { Upload, Download, Trash2, FileIcon } from 'lucide-react';
import { toast } from 'sonner';
import { useI18n } from '@/i18n';

interface ProjectDocumentsTabProps {
  projectId: string;
}

function formatFileSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

export function ProjectDocumentsTab({ projectId }: ProjectDocumentsTabProps) {
  const { t } = useI18n();
  const { data: documents, isLoading } = useProjectDocuments(projectId);
  const uploadDoc = useUploadProjectDocument();
  const deleteDoc = useDeleteProjectDocument();
  const fileRef = useRef<HTMLInputElement>(null);

  const list = documents ?? [];

  function handleUpload(e: React.ChangeEvent<HTMLInputElement>) {
    const file = e.target.files?.[0];
    if (!file) return;
    uploadDoc.mutate(
      { projectId, file },
      {
        onSuccess: () => {
          toast.success(t('projects.file_uploaded', 'File uploaded'));
          if (fileRef.current) fileRef.current.value = '';
        },
        onError: () => toast.error(t('projects.file_upload_failed', 'Failed to upload file')),
      },
    );
  }

  async function handleDownload(fileId: string, fileName: string) {
    try {
      const response = await projectDocumentsApi.download(fileId);
      const blob = new Blob([response.data]);
      const url = window.URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = fileName;
      document.body.appendChild(a);
      a.click();
      window.URL.revokeObjectURL(url);
      document.body.removeChild(a);
    } catch {
      toast.error(t('projects.file_download_failed', 'Failed to download file'));
    }
  }

  function handleDelete(fileId: string) {
    if (!confirm(t('projects.confirm_delete_file', 'Delete this file?'))) return;
    deleteDoc.mutate(fileId, {
      onSuccess: () => toast.success(t('projects.file_deleted', 'File deleted')),
      onError: () => toast.error(t('projects.file_delete_failed', 'Failed to delete file')),
    });
  }

  if (isLoading) return <p className="text-sm text-muted-foreground py-4">{t('common.loading', 'Loading...')}</p>;

  return (
    <div className="space-y-4">
      <div className="flex justify-end">
        <input ref={fileRef} type="file" className="hidden" onChange={handleUpload} />
        <Button size="sm" onClick={() => fileRef.current?.click()} disabled={uploadDoc.isPending}>
          <Upload className="mr-1 h-4 w-4" />
          {uploadDoc.isPending ? t('projects.uploading', 'Uploading...') : t('projects.upload_file', 'Upload File')}
        </Button>
      </div>

      <Card>
        <CardContent className="p-0">
          {list.length > 0 ? (
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>{t('projects.file_name', 'File Name')}</TableHead>
                  <TableHead>{t('projects.file_size', 'Size')}</TableHead>
                  <TableHead className="hidden md:table-cell">{t('projects.uploaded_by', 'Uploaded By')}</TableHead>
                  <TableHead className="hidden md:table-cell">{t('common.date', 'Date')}</TableHead>
                  <TableHead className="w-24">{t('common.actions', 'Actions')}</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {list.map((doc) => (
                  <TableRow key={doc.id}>
                    <TableCell>
                      <div className="flex items-center gap-2">
                        <FileIcon className="h-4 w-4 text-muted-foreground shrink-0" />
                        <span className="font-medium truncate max-w-xs">{doc.file_name}</span>
                      </div>
                    </TableCell>
                    <TableCell className="font-mono text-sm">{formatFileSize(doc.file_size)}</TableCell>
                    <TableCell className="hidden md:table-cell">{doc.uploaded_by ?? '—'}</TableCell>
                    <TableCell className="hidden md:table-cell font-mono text-sm">
                      {doc.created_at ? new Date(doc.created_at).toLocaleDateString() : '—'}
                    </TableCell>
                    <TableCell>
                      <div className="flex gap-1">
                        <Button variant="ghost" size="icon" className="h-8 w-8" onClick={() => handleDownload(doc.id, doc.file_name)} title={t('projects.download', 'Download')}>
                          <Download className="h-3.5 w-3.5" />
                        </Button>
                        <Button variant="ghost" size="icon" className="h-8 w-8" onClick={() => handleDelete(doc.id)} title={t('common.delete', 'Delete')}>
                          <Trash2 className="h-3.5 w-3.5" />
                        </Button>
                      </div>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          ) : (
            <p className="py-6 text-center text-sm text-muted-foreground">
              {t('projects.no_documents', 'No files uploaded. Upload your first document.')}
            </p>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
