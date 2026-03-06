export interface User {
  id: string;
  email: string;
  full_name: string;
  language: string;
  avatar_url: string | null;
  role_id: string;
  role_name: string;
  is_active: boolean;
  created_at: string;
}

export interface Role {
  id: string;
  name: string;
}

export interface CreateUser {
  email: string;
  password: string;
  full_name: string;
  role_id: string;
  language?: string;
}

export interface UpdateUser {
  email: string;
  full_name: string;
  role_id: string;
  is_active: boolean;
  language?: string;
}

export interface ChangePassword {
  new_password: string;
}
