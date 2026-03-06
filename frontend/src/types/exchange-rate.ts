export interface ExchangeRate {
  id: string;
  from_currency_id: string;
  to_currency_id: string;
  rate: string;
  valid_date: string;
  source: string | null;
  created_at: string;
  updated_at: string;
}

export interface CreateExchangeRate {
  from_currency_id: string;
  to_currency_id: string;
  rate: string;
  valid_date: string;
  source?: string;
}
