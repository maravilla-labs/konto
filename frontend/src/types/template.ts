export interface DocumentTemplate {
  id: string;
  name: string;
  template_type: string;
  content_json: string;
  header_json: string | null;
  footer_json: string | null;
  page_setup_json: string | null;
  is_default: boolean;
  created_by: string | null;
  created_at: string;
  updated_at: string;
}

export interface CreateTemplate {
  name: string;
  template_type: string;
  content_json: string;
  header_json?: string | null;
  footer_json?: string | null;
  page_setup_json?: string | null;
  is_default?: boolean;
}

export type UpdateTemplate = CreateTemplate;

export interface TemplateListParams {
  page?: number;
  per_page?: number;
  template_type?: string;
}
