export interface DunningLevel {
  id: string;
  level: number;
  days_after_due: number;
  fee_amount: string;
  subject_template: string;
  body_template: string;
  is_active: boolean;
}

export interface UpdateDunningLevel {
  days_after_due: number;
  fee_amount: number;
  subject_template: string;
  body_template: string;
  is_active: boolean;
}

export interface DunningEntry {
  id: string;
  invoice_id: string;
  dunning_level_id: string;
  level_name: string;
  level_number: number;
  sent_at: string;
  fee_amount: string;
  email_sent: boolean;
  journal_entry_id: string | null;
  notes: string | null;
}

export interface SendReminderData {
  dunning_level_id: string;
  send_email: boolean;
}

export interface DunningRunResult {
  reminders_sent: number;
  errors: string[];
}
