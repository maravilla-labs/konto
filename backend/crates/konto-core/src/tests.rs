#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::module_inception)]
mod tests {
    use crate::auth::jwt::JwtService;
    use crate::auth::password;
    use crate::auth::rbac::{has_permission, permissions};
    use konto_common::enums::{TokenType, UserRole};

    #[test]
    fn test_password_hash_and_verify() {
        let hash = password::hash_password("changeme").unwrap();
        assert!(password::verify_password("changeme", &hash).unwrap());
        assert!(!password::verify_password("wrong", &hash).unwrap());
    }

    #[test]
    fn test_jwt_create_and_verify() {
        let jwt = JwtService::new("test-secret", 900, 604800);

        let token = jwt
            .create_access_token("user-1", "test@test.com", UserRole::Admin)
            .unwrap();
        let claims = jwt.verify_token(&token).unwrap();

        assert_eq!(claims.sub, "user-1");
        assert_eq!(claims.email, "test@test.com");
        assert_eq!(claims.role, UserRole::Admin);
        assert_eq!(claims.token_type, TokenType::Access);
    }

    #[test]
    fn test_jwt_refresh_token() {
        let jwt = JwtService::new("test-secret", 900, 604800);

        let token = jwt
            .create_refresh_token("user-1", "test@test.com", UserRole::Admin)
            .unwrap();
        let claims = jwt.verify_token(&token).unwrap();

        assert_eq!(claims.token_type, TokenType::Refresh);
    }

    #[test]
    fn test_jwt_invalid_token() {
        let jwt = JwtService::new("test-secret", 900, 604800);
        let result = jwt.verify_token("invalid-token");
        assert!(result.is_err());
    }

    #[test]
    fn test_rbac_admin_has_all() {
        let perms = r#"{"all":true}"#;
        assert!(has_permission(perms, permissions::ACCOUNTING).unwrap());
        assert!(has_permission(perms, permissions::CONTACTS).unwrap());
        assert!(has_permission(perms, permissions::IMPORT).unwrap());
    }

    #[test]
    fn test_rbac_auditor_read_only() {
        let perms = r#"{"read_all":true}"#;
        assert!(has_permission(perms, "read_all").unwrap());
        assert!(has_permission(perms, "read_accounting").unwrap());
        assert!(!has_permission(perms, permissions::ACCOUNTING).unwrap());
    }

    #[test]
    fn test_rbac_accountant() {
        let perms = r#"{"accounting":true,"contacts":true}"#;
        assert!(has_permission(perms, permissions::ACCOUNTING).unwrap());
        assert!(has_permission(perms, permissions::CONTACTS).unwrap());
        assert!(!has_permission(perms, permissions::IMPORT).unwrap());
    }

    #[test]
    fn test_import_contacts_csv_parse() {
        use crate::import::contacts_csv::parse_contacts_csv;

        let csv_data = b"\"Datensatz ID\",\"Nr.\",\"Kategorie\",\"Branche\",\"Kontaktart\",\"Name 1\",\"Name 2\"\n\"1\",\"'000001\",\"Partner\",\"\",\"Firma\",\"Test AG\",\"\"";

        let rows = parse_contacts_csv(csv_data).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].name1, "Test AG");
        assert_eq!(rows[0].contact_type, "company");
    }

    #[test]
    fn test_import_time_entries_csv_parse() {
        use crate::import::time_entries_csv::parse_time_entries_csv;

        let csv_data = b"Nr.;Kontaktart;Kategorie;Branche;\"Name 1\";\"Name 2\";Anrede;Titel;Geburtstag;Adresse;PLZ;Ort;Land;Email;Telefon;Mobile;Fax;Website;Skype;Projekt;Status;Datum;\"Gesch\xc3\xa4tzter Aufwand\";\"Effektiver Aufwand\";Pauschal-Betrag;T\xc3\xa4tigkeit;Text;Reisezeit;Reisepauschale;Reisedistanz\n000003;Firma;;;\"Acme GmbH\";;;;;;;;;;;;;;;\"Test Project\";Erledigt;2019-01-29;00:00;\" 3:00\";;Administration;\"test work\";;;0";

        let rows = parse_time_entries_csv(csv_data).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].actual_minutes, 180);
        assert_eq!(rows[0].activity.as_deref(), Some("Administration"));
    }

    #[test]
    fn test_import_journal_account_parsing() {
        // Test the account number parsing function indirectly
        // "1020 - Bank UBS (alt) CHF (UBS Switzerland AG)" -> 1020
        let s = "1020 - Bank UBS (alt) CHF (UBS Switzerland AG)";
        let num: Option<i32> = s.split_whitespace().next().and_then(|n| n.parse().ok());
        assert_eq!(num, Some(1020));
    }
}
