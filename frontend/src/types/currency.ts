export interface Currency {
  id: string;
  code: string;
  name: string;
  symbol: string;
  is_primary: boolean;
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
}
