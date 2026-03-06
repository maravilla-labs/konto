import client from './client';
import type {
  EmailTemplate,
  UpdateEmailTemplate,
  EmailTemplatePreview,
} from '@/types/email-template';

export const emailTemplateApi = {
  list() {
    return client.get<EmailTemplate[]>('/email-templates');
  },

  get(id: string) {
    return client.get<EmailTemplate>(`/email-templates/${id}`);
  },

  update(id: string, data: UpdateEmailTemplate) {
    return client.put<EmailTemplate>(`/email-templates/${id}`, data);
  },

  preview(id: string) {
    return client.post<EmailTemplatePreview>(
      `/email-templates/${id}/preview`,
    );
  },
};
