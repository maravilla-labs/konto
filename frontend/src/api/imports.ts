import client from './client';
import type { ImportBatch, ImportPreview, ImportResult, ImportType } from '@/types/imports';

export const importsApi = {
  upload(file: File, importType: ImportType) {
    const formData = new FormData();
    formData.append('file', file);
    formData.append('import_type', importType);
    return client.post<ImportBatch>('/import/upload', formData, {
      headers: { 'Content-Type': 'multipart/form-data' },
    });
  },

  preview(batchId: string) {
    return client.post<ImportPreview>(`/import/${batchId}/preview`);
  },

  execute(batchId: string) {
    return client.post<ImportResult>(`/import/${batchId}/execute`);
  },

  list() {
    return client.get<ImportBatch[]>('/import');
  },
};
