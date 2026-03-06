import client from './client';
import type {
  ContactRelationship,
  CreateContactRelationshipRequest,
  UpdateContactRelationshipRequest,
} from '@/types/contact-relationship';

export const contactRelationshipsApi = {
  list(contactId: string) {
    return client.get<ContactRelationship[]>(
      `/contacts/${contactId}/relationships`,
    );
  },

  create(contactId: string, data: CreateContactRelationshipRequest) {
    return client.post<ContactRelationship>(
      `/contacts/${contactId}/relationships`,
      data,
    );
  },

  update(id: string, data: UpdateContactRelationshipRequest) {
    return client.put<ContactRelationship>(
      `/contact-relationships/${id}`,
      data,
    );
  },

  delete(id: string) {
    return client.delete(`/contact-relationships/${id}`);
  },
};
