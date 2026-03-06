export interface ProjectDocument {
  id: string;
  project_id: string;
  project_item_id?: string;
  file_name: string;
  file_path: string;
  file_size: number;
  content_type?: string;
  uploaded_by?: string;
  created_at: string;
}
