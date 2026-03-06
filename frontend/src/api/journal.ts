import client from './client';
import type { JournalEntry, JournalDetail, CreateJournalEntry, JournalAttachment } from '@/types/journal';
import type { PaginatedResponse, ListParams } from '@/types/common';

export interface JournalListParams extends ListParams {
  date_from?: string;
  date_to?: string;
}

export const journalApi = {
  list(params?: JournalListParams) {
    return client.get<PaginatedResponse<JournalEntry>>('/journal', { params });
  },

  get(id: string) {
    return client.get<JournalDetail>(`/journal/${id}`);
  },

  create(data: CreateJournalEntry) {
    return client.post<JournalDetail>('/journal', data);
  },

  post(id: string) {
    return client.post<JournalEntry>(`/journal/${id}/post`);
  },

  reverse(id: string) {
    return client.post<JournalDetail>(`/journal/${id}/reverse`);
  },

  bulkPost(entryIds?: string[], allDrafts?: boolean) {
    return client.post<{ posted: number }>('/journal/bulk-post', {
      entry_ids: entryIds,
      all_drafts: allDrafts ?? false,
    });
  },

  listAttachments(entryId: string) {
    return client.get<JournalAttachment[]>(`/journal/${entryId}/attachments`);
  },

  uploadAttachment(entryId: string, file: File) {
    const formData = new FormData();
    formData.append('file', file);
    return client.post<JournalAttachment>(`/journal/${entryId}/attachments`, formData, {
      headers: { 'Content-Type': 'multipart/form-data' },
    });
  },

  deleteAttachment(id: string) {
    return client.delete(`/journal/attachments/${id}`);
  },
};
