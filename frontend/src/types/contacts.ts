export interface Contact {
  id: string;
  contact_type: string;
  category?: string;
  name1: string;
  name2?: string;
  salutation?: string;
  title?: string;
  salutation_form?: string;
  email?: string;
  email2?: string;
  phone?: string;
  phone2?: string;
  mobile?: string;
  fax?: string;
  address?: string;
  postal_code?: string;
  city?: string;
  country?: string;
  website?: string;
  vat_number?: string;
  language?: string;
  industry?: string;
  birthday?: string;
  employee_count?: number;
  trade_register_number?: string;
  notes?: string;
  customer_number?: string;
  vat_mode?: string;
  is_active: boolean;
}

export interface CreateContact {
  contact_type: string;
  category?: string;
  name1: string;
  name2?: string;
  salutation?: string;
  title?: string;
  salutation_form?: string;
  email?: string;
  email2?: string;
  phone?: string;
  phone2?: string;
  mobile?: string;
  fax?: string;
  address?: string;
  postal_code?: string;
  city?: string;
  country?: string;
  website?: string;
  vat_number?: string;
  language?: string;
  industry?: string;
  birthday?: string;
  employee_count?: number;
  trade_register_number?: string;
  notes?: string;
  customer_number?: string;
  vat_mode?: string;
}

export interface UpdateContact {
  name1?: string;
  contact_type?: string;
  name2?: string | null;
  salutation?: string | null;
  title?: string | null;
  salutation_form?: string | null;
  email?: string | null;
  email2?: string | null;
  phone?: string | null;
  phone2?: string | null;
  mobile?: string | null;
  fax?: string | null;
  address?: string | null;
  postal_code?: string | null;
  city?: string | null;
  country?: string | null;
  language?: string | null;
  website?: string | null;
  vat_number?: string | null;
  industry?: string | null;
  birthday?: string | null;
  employee_count?: number | null;
  trade_register_number?: string | null;
  notes?: string | null;
  customer_number?: string | null;
  vat_mode?: string | null;
  is_active?: boolean;
}

export interface ContactTag {
  id: string;
  name: string;
  color: string;
}

export interface CreateContactTag {
  name: string;
  color: string;
}

export interface ContactPerson {
  id: string;
  contact_id: string;
  first_name: string;
  last_name: string;
  email?: string;
  phone?: string;
  department?: string;
  position?: string;
}

export interface CreateContactPerson {
  first_name: string;
  last_name: string;
  email?: string;
  phone?: string;
  department?: string;
  position?: string;
}

export interface UpdateContactPerson {
  first_name?: string;
  last_name?: string;
  email?: string;
  phone?: string;
  department?: string;
  position?: string;
}

export interface DocumentSummary {
  id: string;
  doc_type: string;
  doc_number?: string;
  title: string;
  status: string;
  total: string;
}
