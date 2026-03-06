import client from './client';

export interface SetupStatusResponse {
  setup_needed: boolean;
}

export interface SetupCompleteRequest {
  admin_email: string;
  admin_password: string;
  admin_full_name: string;
  admin_language: string;
  legal_name: string;
  trade_name?: string;
  street?: string;
  postal_code?: string;
  city?: string;
  country?: string;
  legal_entity_type?: string;
  default_currency?: string;
  vat_method?: string;
  flat_rate_percentage?: number;
  date_format?: string;
  fiscal_year_start_month?: number;
}

export interface SetupCompleteResponse {
  access_token: string;
  refresh_token: string;
  token_type: string;
}

export interface BrandingResponse {
  legal_name: string | null;
  trade_name: string | null;
  logo_url: string | null;
}

export const setupApi = {
  getStatus() {
    return client.get<SetupStatusResponse>('/setup/status');
  },

  complete(data: SetupCompleteRequest) {
    return client.post<SetupCompleteResponse>('/setup/complete', data);
  },

  getBranding() {
    return client.get<BrandingResponse>('/setup/branding');
  },
};
