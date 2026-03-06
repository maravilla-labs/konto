import client from './client';
import type { ContactTag, CreateContactTag } from '@/types/contacts';

export const contactTagsApi = {
  list() {
    return client.get<ContactTag[]>('/contact-tags');
  },

  create(data: CreateContactTag) {
    return client.post<ContactTag>('/contact-tags', data);
  },

  delete(id: string) {
    return client.delete(`/contact-tags/${id}`);
  },

  assign(contactId: string, tagId: string) {
    return client.put(`/contacts/${contactId}/tags/${tagId}`);
  },

  remove(contactId: string, tagId: string) {
    return client.delete(`/contacts/${contactId}/tags/${tagId}`);
  },
};
