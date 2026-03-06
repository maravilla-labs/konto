export type EmailTemplateType =
  | 'invoice_send'
  | 'invoice_reminder_1'
  | 'invoice_reminder_2'
  | 'invoice_reminder_3'
  | 'credit_note'
  | 'document_send';

export interface EmailTemplate {
  id: string;
  template_type: EmailTemplateType;
  subject: string;
  body_html: string;
  language: string;
  is_default: boolean;
}

export interface UpdateEmailTemplate {
  subject: string;
  body_html: string;
}

export interface EmailTemplatePreview {
  rendered_subject: string;
  rendered_body_html: string;
}
