export type AccountType = 'asset' | 'liability' | 'equity' | 'revenue' | 'expense';

export interface Account {
  id: string;
  number: number;
  name: string;
  account_type: string;
  description: string | null;
  parent_id: string | null;
  currency_id: string | null;
  is_active: boolean;
}

export interface AccountTree {
  id: string;
  number: number;
  name: string;
  account_type: string;
  description: string | null;
  is_active: boolean;
  children: AccountTree[];
}

export interface AccountTreeWithBalance {
  id: string;
  number: number;
  name: string;
  account_type: string;
  parent_id: string | null;
  balance: string;
  is_active: boolean;
  children: AccountTreeWithBalance[];
}

export interface CreateAccount {
  number: number;
  name: string;
  parent_id?: string | null;
  currency_id?: string | null;
}

export interface UpdateAccount {
  name?: string;
  description?: string | null;
  is_active?: boolean;
  parent_id?: string | null;
  currency_id?: string | null;
}
