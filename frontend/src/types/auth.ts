export type Role = 'admin' | 'accountant' | 'employee' | 'auditor';

export interface User {
  id: string;
  email: string;
  full_name: string;
  language: string;
  avatar_url: string | null;
  role: string;
}

export interface LoginRequest {
  email: string;
  password: string;
}

export interface LoginResponse {
  access_token: string;
  refresh_token: string;
  token_type: string;
}
