export interface DefaultAccount {
  id: string;
  setting_key: string;
  account_id: string | null;
  account_name: string | null;
  account_number: string | null;
  description: string | null;
}

export interface DefaultAccountUpdate {
  setting_key: string;
  account_id: string | null;
}
