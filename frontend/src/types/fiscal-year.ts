export interface FiscalYear {
  id: string;
  name: string;
  start_date: string;
  end_date: string;
  status: 'open' | 'closed';
  created_at: string;
  updated_at: string;
}

export interface FiscalPeriod {
  id: string;
  fiscal_year_id: string;
  name: string;
  start_date: string;
  end_date: string;
  period_number: number;
  status: 'open' | 'closed';
}

export interface CreateFiscalYear {
  name: string;
  start_date: string;
  end_date: string;
}

export interface UpdateFiscalYear {
  name?: string;
  start_date?: string;
  end_date?: string;
}
