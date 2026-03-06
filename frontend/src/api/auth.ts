import client from './client';
import type { LoginRequest, LoginResponse, User } from '@/types/auth';

export const authApi = {
  login(data: LoginRequest) {
    return client.post<LoginResponse>('/auth/login', data);
  },

  refresh(refreshToken: string) {
    return client.post<{ access_token: string; refresh_token: string }>(
      '/auth/refresh',
      { refresh_token: refreshToken }
    );
  },

  me() {
    return client.get<User>('/auth/me');
  },

  updateProfile(data: { full_name?: string; language?: string }) {
    return client.put<User>('/auth/me', data);
  },

  updateLanguage(language: string) {
    return client.put<User>('/auth/me/language', { language });
  },

  uploadAvatar(file: File) {
    const formData = new FormData();
    formData.append('file', file);
    return client.post<User>('/auth/me/avatar', formData, {
      headers: { 'Content-Type': 'multipart/form-data' },
    });
  },
};
