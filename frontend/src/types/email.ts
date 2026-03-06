export interface EmailSettings {
  id: string;
  smtp_host: string;
  smtp_port: number;
  smtp_username: string;
  smtp_password: string;
  smtp_encryption: string;
  from_email: string;
  from_name: string;
  reply_to_email: string | null;
  bcc_email: string | null;
  is_active: boolean;
}

export interface UpdateEmailSettings {
  smtp_host: string;
  smtp_port: number;
  smtp_username: string;
  smtp_password?: string;
  smtp_encryption: string;
  from_email: string;
  from_name: string;
  reply_to_email?: string;
  bcc_email?: string;
  is_active: boolean;
}

export interface TestEmailRequest {
  to_email?: string;
}
