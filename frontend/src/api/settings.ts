import client from './client';
import type { CompanySettings, UpdateCompanySettings } from '@/types/settings';

export const settingsApi = {
  get() {
    return client.get<CompanySettings>('/settings');
  },

  update(data: UpdateCompanySettings) {
    return client.put<CompanySettings>('/settings', data);
  },

  uploadLogo(file: File) {
    const formData = new FormData();
    formData.append('logo', file);
    return client.post<CompanySettings>('/settings/logo', formData, {
      headers: { 'Content-Type': 'multipart/form-data' },
    });
  },
};
