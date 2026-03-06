import client from './client';
import type { ContactPerson, CreateContactPerson, UpdateContactPerson } from '@/types/contacts';

export const contactPersonsApi = {
  list(contactId: string) {
    return client.get<ContactPerson[]>(`/contacts/${contactId}/persons`);
  },

  create(contactId: string, data: CreateContactPerson) {
    return client.post<ContactPerson>(`/contacts/${contactId}/persons`, data);
  },

  update(contactId: string, personId: string, data: UpdateContactPerson) {
    return client.put<ContactPerson>(`/contacts/${contactId}/persons/${personId}`, data);
  },

  delete(contactId: string, personId: string) {
    return client.delete(`/contacts/${contactId}/persons/${personId}`);
  },
};
