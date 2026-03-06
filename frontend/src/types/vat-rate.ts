export interface VatRate {
  id: string;
  code: string;
  name: string;
  rate: number;
  vat_type: string;
  is_active: boolean;
  valid_from: string | null;
  valid_to: string | null;
}

export interface CreateVatRate {
  code: string;
  name: string;
  rate: number;
  vat_type: string;
  valid_from?: string;
  valid_to?: string;
}

export interface UpdateVatRate {
  code: string;
  name: string;
  rate: number;
  vat_type: string;
  is_active: boolean;
  valid_from?: string;
  valid_to?: string;
}
