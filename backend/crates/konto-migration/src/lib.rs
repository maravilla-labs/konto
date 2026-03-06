pub use sea_orm_migration::prelude::*;

mod m20240101_000001_create_tables;
mod m20240101_000002_create_accounting;
mod m20240101_000003_create_contacts;
mod m20240101_000004_create_projects;
mod m20240101_000005_create_import;
mod m20240101_000006_seed_data;
mod m20240101_000007_add_account_description;
mod m20240101_000008_create_fiscal_exchange;
mod m20240101_000009_seed_vat_codes;
mod m20240101_000010_create_invoices;
mod m20240101_000011_create_settings;
mod m20240101_000012_create_templates;
mod m20240101_000013_create_documents;
mod m20240101_000014_add_invoice_template;
mod m20240101_000015_seed_templates;
mod m20240101_000016_create_email_settings;
mod m20240101_000017_create_recurring_invoices;
mod m20240101_000018_create_credit_notes;
mod m20240101_000019_create_dunning;
mod m20240101_000020_create_expenses;
mod m20240101_000021_create_bank_transactions;
mod m20240101_000022_create_email_templates;
mod m20240101_000023_create_contact_tags;
mod m20240101_000026_add_time_entry_billed;
mod m20240101_000027_create_invoice_payments;
mod m20240101_000028_create_default_accounts;
mod m20240101_000029_create_journal_attachments;
mod m20240101_000030_unified_expenses;
mod m20240101_000031_company_settings_locale;
mod m20240101_000032_company_settings_jurisdiction;
mod m20240101_000033_create_shareholders;
mod m20240101_000034_create_annual_report_notes;
mod m20240101_000035_create_annual_reports;
mod m20240101_000036_seed_shareholders;
mod m20240101_000037_alter_notes_ordering;
mod m20240101_000038_add_audit_optout;
mod m20240101_000039_fix_journal_attachment_fk;
mod m20240101_000040_add_multilang_fields;
mod m20240101_000041_add_user_language;
mod m20240101_000042_add_user_avatar;
mod m20240101_000043_create_contact_relationships;
mod m20240101_000044_alter_contacts;
mod m20240101_000045_create_rate_functions;
mod m20240101_000046_create_project_members;
mod m20240101_000047_alter_projects;
mod m20240101_000048_create_project_items;
mod m20240101_000049_create_project_milestones;
mod m20240101_000050_create_project_documents;
mod m20240101_000051_alter_time_entries;
mod m20240101_000052_create_timesheets;
mod m20240101_000053_alter_time_entries_timesheet;
mod m20240101_000054_alter_activity_types;
mod m20240101_000055_create_project_activity_types;
mod m20240101_000056_alter_time_entries_quantity;
mod m20240101_000057_alter_vat_rates_type;
mod m20240101_000058_add_flat_rate_percentage;
mod m20240101_000059_alter_invoices_form;
mod m20240101_000060_add_qr_iban;
mod m20240101_000061_create_fixed_assets;
mod m20240101_000062_create_depreciation_entries;
pub mod m20240101_000063_create_employees;
pub mod m20240101_000064_create_payroll_settings;
mod m20240101_000065_seed_payroll_settings;
mod m20240101_000066_create_payroll_runs;
mod m20240101_000067_create_payroll_run_lines;
mod m20240101_000068_create_payout_entries;
mod m20240101_000069_alter_employees_extra_fields;
mod m20240101_000070_alter_projects_conditions;
mod m20240101_000071_alter_project_members_budget;
mod m20240101_000072_alter_project_activity_types_budget;
mod m20240101_000073_alter_time_entries_status_workflow;
mod m20240101_000074_alter_company_settings_project_numbering;
mod m20240101_000075_create_project_sub_statuses;
mod m20240101_000076_seed_project_sub_statuses;
mod m20240101_000077_alter_projects_sub_status;
mod m20240101_000078_remove_solutas_seeds;
mod m20240101_000079_add_contact_vat_mode;
mod m20240101_000080_add_vat_category;
mod m20240101_000081_migrate_contact_persons;
mod m20240101_000082_backfill_contact_category;
mod m20240101_000083_alter_company_settings_numbering;
mod m20240101_000084_alter_contacts_customer_number;
mod m20240101_000085_alter_employees_number;
mod m20240101_000086_alter_projects_owner_and_users_employee;
mod m20240101_000087_backfill_contact_customer_number;
mod m20240101_000088_html_to_markdown;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240101_000001_create_tables::Migration),
            Box::new(m20240101_000002_create_accounting::Migration),
            Box::new(m20240101_000003_create_contacts::Migration),
            Box::new(m20240101_000004_create_projects::Migration),
            Box::new(m20240101_000005_create_import::Migration),
            Box::new(m20240101_000006_seed_data::Migration),
            Box::new(m20240101_000007_add_account_description::Migration),
            Box::new(m20240101_000008_create_fiscal_exchange::Migration),
            Box::new(m20240101_000009_seed_vat_codes::Migration),
            Box::new(m20240101_000010_create_invoices::Migration),
            Box::new(m20240101_000011_create_settings::Migration),
            Box::new(m20240101_000012_create_templates::Migration),
            Box::new(m20240101_000013_create_documents::Migration),
            Box::new(m20240101_000014_add_invoice_template::Migration),
            Box::new(m20240101_000015_seed_templates::Migration),
            Box::new(m20240101_000016_create_email_settings::Migration),
            Box::new(m20240101_000017_create_recurring_invoices::Migration),
            Box::new(m20240101_000018_create_credit_notes::Migration),
            Box::new(m20240101_000019_create_dunning::Migration),
            Box::new(m20240101_000020_create_expenses::Migration),
            Box::new(m20240101_000021_create_bank_transactions::Migration),
            Box::new(m20240101_000022_create_email_templates::Migration),
            Box::new(m20240101_000023_create_contact_tags::Migration),
            Box::new(m20240101_000026_add_time_entry_billed::Migration),
            Box::new(m20240101_000027_create_invoice_payments::Migration),
            Box::new(m20240101_000028_create_default_accounts::Migration),
            Box::new(m20240101_000029_create_journal_attachments::Migration),
            Box::new(m20240101_000030_unified_expenses::Migration),
            Box::new(m20240101_000031_company_settings_locale::Migration),
            Box::new(m20240101_000032_company_settings_jurisdiction::Migration),
            Box::new(m20240101_000033_create_shareholders::Migration),
            Box::new(m20240101_000034_create_annual_report_notes::Migration),
            Box::new(m20240101_000035_create_annual_reports::Migration),
            Box::new(m20240101_000036_seed_shareholders::Migration),
            Box::new(m20240101_000037_alter_notes_ordering::Migration),
            Box::new(m20240101_000038_add_audit_optout::Migration),
            Box::new(m20240101_000039_fix_journal_attachment_fk::Migration),
            Box::new(m20240101_000040_add_multilang_fields::Migration),
            Box::new(m20240101_000041_add_user_language::Migration),
            Box::new(m20240101_000042_add_user_avatar::Migration),
            Box::new(m20240101_000043_create_contact_relationships::Migration),
            Box::new(m20240101_000044_alter_contacts::Migration),
            Box::new(m20240101_000045_create_rate_functions::Migration),
            Box::new(m20240101_000046_create_project_members::Migration),
            Box::new(m20240101_000047_alter_projects::Migration),
            Box::new(m20240101_000048_create_project_items::Migration),
            Box::new(m20240101_000049_create_project_milestones::Migration),
            Box::new(m20240101_000050_create_project_documents::Migration),
            Box::new(m20240101_000051_alter_time_entries::Migration),
            Box::new(m20240101_000052_create_timesheets::Migration),
            Box::new(m20240101_000053_alter_time_entries_timesheet::Migration),
            Box::new(m20240101_000054_alter_activity_types::Migration),
            Box::new(m20240101_000055_create_project_activity_types::Migration),
            Box::new(m20240101_000056_alter_time_entries_quantity::Migration),
            Box::new(m20240101_000057_alter_vat_rates_type::Migration),
            Box::new(m20240101_000058_add_flat_rate_percentage::Migration),
            Box::new(m20240101_000059_alter_invoices_form::Migration),
            Box::new(m20240101_000060_add_qr_iban::Migration),
            Box::new(m20240101_000061_create_fixed_assets::Migration),
            Box::new(m20240101_000062_create_depreciation_entries::Migration),
            Box::new(m20240101_000063_create_employees::Migration),
            Box::new(m20240101_000064_create_payroll_settings::Migration),
            Box::new(m20240101_000065_seed_payroll_settings::Migration),
            Box::new(m20240101_000066_create_payroll_runs::Migration),
            Box::new(m20240101_000067_create_payroll_run_lines::Migration),
            Box::new(m20240101_000068_create_payout_entries::Migration),
            Box::new(m20240101_000069_alter_employees_extra_fields::Migration),
            Box::new(m20240101_000070_alter_projects_conditions::Migration),
            Box::new(m20240101_000071_alter_project_members_budget::Migration),
            Box::new(m20240101_000072_alter_project_activity_types_budget::Migration),
            Box::new(m20240101_000073_alter_time_entries_status_workflow::Migration),
            Box::new(m20240101_000074_alter_company_settings_project_numbering::Migration),
            Box::new(m20240101_000075_create_project_sub_statuses::Migration),
            Box::new(m20240101_000076_seed_project_sub_statuses::Migration),
            Box::new(m20240101_000077_alter_projects_sub_status::Migration),
            Box::new(m20240101_000078_remove_solutas_seeds::Migration),
            Box::new(m20240101_000079_add_contact_vat_mode::Migration),
            Box::new(m20240101_000080_add_vat_category::Migration),
            Box::new(m20240101_000081_migrate_contact_persons::Migration),
            Box::new(m20240101_000082_backfill_contact_category::Migration),
            Box::new(m20240101_000083_alter_company_settings_numbering::Migration),
            Box::new(m20240101_000084_alter_contacts_customer_number::Migration),
            Box::new(m20240101_000085_alter_employees_number::Migration),
            Box::new(m20240101_000086_alter_projects_owner_and_users_employee::Migration),
            Box::new(m20240101_000087_backfill_contact_customer_number::Migration),
            Box::new(m20240101_000088_html_to_markdown::Migration),
        ]
    }
}
