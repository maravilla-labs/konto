import client from './client';
import type { User, Role, CreateUser, UpdateUser, ChangePassword } from '@/types/user';

export const usersApi = {
  list() {
    return client.get<User[]>('/users');
  },

  get(id: string) {
    return client.get<User>(`/users/${id}`);
  },

  create(data: CreateUser) {
    return client.post<User>('/users', data);
  },

  update(id: string, data: UpdateUser) {
    return client.put<User>(`/users/${id}`, data);
  },

  changePassword(id: string, data: ChangePassword) {
    return client.put(`/users/${id}/password`, data);
  },

  listRoles() {
    return client.get<Role[]>('/roles');
  },
};
