import client from './client';
import type { Employee, CreateEmployee, UpdateEmployee } from '@/types/employee';

export const employeesApi = {
  list() {
    return client.get<Employee[]>('/employees');
  },
  get(id: string) {
    return client.get<Employee>(`/employees/${id}`);
  },
  create(data: CreateEmployee) {
    return client.post<Employee>('/employees', data);
  },
  update(id: string, data: UpdateEmployee) {
    return client.put<Employee>(`/employees/${id}`, data);
  },
  delete(id: string) {
    return client.delete(`/employees/${id}`);
  },
};
