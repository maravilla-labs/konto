export interface ContactRelationship {
  id: string;
  person_contact_id: string;
  org_contact_id: string;
  person_name?: string;
  org_name?: string;
  role?: string;
  position?: string;
  department?: string;
  is_primary: boolean;
  notes?: string;
  created_at: string;
  updated_at: string;
}

export interface CreateContactRelationshipRequest {
  person_contact_id?: string;
  org_contact_id?: string;
  role?: string;
  position?: string;
  department?: string;
  is_primary?: boolean;
  notes?: string;
}

export interface UpdateContactRelationshipRequest {
  role?: string;
  position?: string;
  department?: string;
  is_primary?: boolean;
  notes?: string;
}
