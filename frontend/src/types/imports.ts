export type ImportType = 'accounts' | 'contacts' | 'time_entries' | 'projects' | 'journal';

export interface ImportBatch {
  id: string;
  import_type: string;
  file_name: string;
  status: string;
  total_rows: number | null;
  imported_rows: number | null;
  error_rows: number | null;
}

export interface ImportPreview {
  total: number;
  preview: Record<string, unknown>[];
}

export interface ImportResult {
  id: string;
  import_type: string;
  file_name: string;
  status: string;
  total_rows: number | null;
  imported_rows: number | null;
  error_rows: number | null;
  error_log: string[] | null;
}
