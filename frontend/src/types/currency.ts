export interface Currency {
  id: string;
  code: string;
  name: string;
  symbol: string;
  is_primary: boolean;
  default_bank_account_id: string | null;
}

export interface CreateCurrency {
  code: string;
  name: string;
  symbol: string;
}

export interface UpdateCurrency {
  code: string;
  name: string;
  symbol: string;
  default_bank_account_id?: string | null;
}
