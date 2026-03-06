use axum::middleware;
use axum::routing::{delete, get, post, put};
use axum::Router;
use konto_api::handlers::*;
use konto_api::middleware::auth::{auth_middleware, require_admin, require_admin_or_auditor};
use konto_api::state::AppState;
use tower_http::services::ServeDir;

pub fn build_router(state: AppState) -> Router {
    // Public routes (no auth required)
    let public = Router::new()
        .route("/api/v1/auth/login", post(auth_handler::login))
        .route("/api/v1/auth/refresh", post(auth_handler::refresh))
        .route("/api/v1/setup/status", get(setup_handler::setup_status))
        .route("/api/v1/setup/complete", post(setup_handler::setup_complete))
        .route("/api/v1/setup/branding", get(setup_handler::get_branding));

    // Admin-only routes (require role "admin")
    let admin_routes = Router::new()
        // Users
        .route("/api/v1/users", get(user_handler::list_users).post(user_handler::create_user))
        .route(
            "/api/v1/users/{id}",
            get(user_handler::get_user).put(user_handler::update_user),
        )
        .route("/api/v1/users/{id}/password", put(user_handler::change_password))
        .route("/api/v1/roles", get(user_handler::list_roles))
        // Settings (write)
        .route("/api/v1/settings", put(settings_handler::update_settings))
        .route("/api/v1/settings/logo", post(settings_handler::upload_logo))
        // Default Accounts (write)
        .route("/api/v1/settings/default-accounts", put(default_account_handler::update_default_accounts))
        // Email Settings (write + test)
        .route("/api/v1/settings/email", put(email_handler::update_email_settings))
        .route("/api/v1/settings/email/test", post(email_handler::send_test_email))
        // Email Templates (write)
        .route("/api/v1/email-templates/{id}", put(email_template_handler::update_email_template))
        // Import
        .route("/api/v1/import/upload", post(import_handler::upload_import))
        .route("/api/v1/import/{id}/preview", post(import_handler::preview_import))
        .route("/api/v1/import/{id}/execute", post(import_handler::execute_import))
        // Shareholders (write)
        .route("/api/v1/shareholders", post(annual_report_handler::create_shareholder))
        .route("/api/v1/shareholders/{id}", put(annual_report_handler::update_shareholder).delete(annual_report_handler::delete_shareholder))
        // Employees (write)
        .route("/api/v1/employees", post(employee_handler::create_employee))
        .route("/api/v1/employees/{id}", put(employee_handler::update_employee).delete(employee_handler::delete_employee))
        // Payroll Settings (write)
        .route("/api/v1/payroll-settings", put(payroll_settings_handler::update_payroll_settings))
        // Payroll Runs (create + state transitions)
        .route("/api/v1/payroll-runs", post(payroll_run_handler::create_payroll_run))
        .route("/api/v1/payroll-runs/{id}", delete(payroll_run_handler::delete_payroll_run))
        .route("/api/v1/payroll-runs/{id}/calculate", post(payroll_run_handler::calculate_payroll_run))
        .route("/api/v1/payroll-runs/{id}/approve", post(payroll_run_handler::approve_payroll_run))
        .route("/api/v1/payroll-runs/{id}/pay", post(payroll_run_handler::mark_payroll_run_paid))
        .route("/api/v1/payroll-runs/{id}/generate-payouts", post(payout_handler::generate_payouts))
        .route("/api/v1/payroll-runs/{id}/export-pain001", post(payout_handler::export_pain001))
        .route("/api/v1/payroll-runs/{id}/mark-all-paid", post(payout_handler::mark_all_payouts_paid))
        .route("/api/v1/payout-entries/{id}/mark-paid", put(payout_handler::mark_payout_paid))
        // Salary Certificates
        .route("/api/v1/salary-certificates/{year}", get(salary_certificate_handler::list_salary_certificates))
        .route("/api/v1/salary-certificates/{year}/{employee_id}/pdf", get(salary_certificate_handler::download_salary_certificate_pdf))
        .route("/api/v1/salary-certificates/{year}/zip", get(salary_certificate_handler::download_salary_certificates_zip))
        // Dunning (config)
        .route("/api/v1/dunning/levels/{id}", put(dunning_handler::update_dunning_level))
        .route("/api/v1/dunning/run", post(dunning_handler::run_dunning))
        // VAT Rates (write)
        .route("/api/v1/vat-rates", post(vat_rate_handler::create_vat_rate))
        .route("/api/v1/vat-rates/{id}", put(vat_rate_handler::update_vat_rate).delete(vat_rate_handler::deactivate_vat_rate))
        // Currencies (write)
        .route("/api/v1/currencies", post(currency_handler::create_currency))
        .route("/api/v1/currencies/{id}", put(currency_handler::update_currency))
        // Fiscal Year (close)
        .route("/api/v1/fiscal-years/{id}/close", post(fiscal_year_handler::close_fiscal_year))
        // Fixed Assets (depreciation run)
        .route("/api/v1/fixed-assets/run-depreciation", post(fixed_asset_handler::run_depreciation))
        // Project Sub-Statuses (write)
        .route("/api/v1/project-sub-statuses", post(project_sub_status_handler::create_project_sub_status))
        .route("/api/v1/project-sub-statuses/{id}", put(project_sub_status_handler::update_project_sub_status).delete(project_sub_status_handler::delete_project_sub_status))
        // Annual Report (generate/finalize)
        .route("/api/v1/fiscal-years/{id}/annual-report/generate", post(annual_report_handler::generate_pdf))
        .route("/api/v1/fiscal-years/{id}/annual-report/finalize", post(annual_report_handler::finalize_report))
        .layer(middleware::from_fn(require_admin));

    // Auditor routes (require role "admin" or "auditor")
    let auditor_routes = Router::new()
        .route("/api/v1/audit-log", get(audit_handler::list_audit_logs))
        .layer(middleware::from_fn(require_admin_or_auditor));

    // Authenticated routes (any valid JWT, no role check)
    let authenticated_routes = Router::new()
        // Auth profile
        .route("/api/v1/auth/me", get(auth_handler::me).put(auth_handler::update_my_profile))
        .route("/api/v1/auth/me/language", put(auth_handler::update_my_language))
        .route("/api/v1/auth/me/avatar", post(auth_handler::upload_my_avatar))
        // Accounts
        .route("/api/v1/accounts", get(account_handler::list_accounts).post(account_handler::create_account))
        .route("/api/v1/accounts/tree", get(account_handler::accounts_tree))
        .route("/api/v1/accounts/tree-with-balances", get(account_handler::accounts_tree_with_balances))
        .route(
            "/api/v1/accounts/{id}",
            get(account_handler::get_account)
                .put(account_handler::update_account)
                .delete(account_handler::delete_account),
        )
        // Contacts
        .route("/api/v1/contacts", get(contact_handler::list_contacts).post(contact_handler::create_contact))
        .route(
            "/api/v1/contacts/{id}",
            get(contact_handler::get_contact)
                .put(contact_handler::update_contact)
                .delete(contact_handler::delete_contact),
        )
        .route("/api/v1/contacts/{id}/vat-info", get(contact_vat_handler::get_vat_info))
        .route("/api/v1/contacts/{id}/persons-via-relationships", get(contact_handler::list_contact_persons_via_relationships))
        // Contact Persons
        .route("/api/v1/contacts/{id}/persons", get(contact_person_handler::list_by_contact).post(contact_person_handler::create_person))
        .route(
            "/api/v1/contacts/{id}/persons/{person_id}",
            put(contact_person_handler::update_person)
                .delete(contact_person_handler::delete_person),
        )
        // Contact Tags
        .route("/api/v1/contact-tags", get(contact_tag_handler::list_tags).post(contact_tag_handler::create_tag))
        .route("/api/v1/contact-tags/{id}", delete(contact_tag_handler::delete_tag))
        .route("/api/v1/contacts/{id}/tags/{tag_id}", put(contact_tag_handler::assign_tag_to_contact).delete(contact_tag_handler::remove_tag_from_contact))
        // Contact Sub-resources
        .route("/api/v1/contacts/{id}/invoices", get(contact_sub_handler::list_contact_invoices))
        .route("/api/v1/contacts/{id}/documents", get(contact_sub_handler::list_contact_documents))
        .route("/api/v1/contacts/{id}/time-entries", get(contact_sub_handler::list_contact_time_entries))
        // Journal
        .route("/api/v1/journal", get(journal_handler::list_journal).post(journal_handler::create_journal_entry))
        .route("/api/v1/journal/bulk-post", post(journal_handler::bulk_post_journal))
        .route("/api/v1/journal/{id}", get(journal_handler::get_journal_entry))
        .route("/api/v1/journal/{id}/post", post(journal_handler::post_journal_entry))
        .route("/api/v1/journal/{id}/reverse", post(journal_handler::reverse_journal_entry))
        .route("/api/v1/journal/{id}/attachments", get(journal_attachment_handler::list_attachments).post(journal_attachment_handler::upload_attachment))
        .route("/api/v1/journal/attachments/{id}/download", get(journal_attachment_handler::download_attachment))
        .route("/api/v1/journal/attachments/{id}/preview", get(journal_attachment_handler::preview_attachment))
        .route("/api/v1/journal/attachments/{id}", delete(journal_attachment_handler::delete_attachment))
        // Projects
        .route("/api/v1/projects", get(project_handler::list_projects).post(project_handler::create_project))
        .route(
            "/api/v1/projects/{id}",
            get(project_handler::get_project)
                .put(project_handler::update_project)
                .delete(project_handler::delete_project),
        )
        .route("/api/v1/projects/{id}/summary", get(project_handler::get_project_summary))
        .route("/api/v1/projects/{id}/budget-analytics", get(project_handler::get_budget_analytics))
        // Time Entries
        .route("/api/v1/time-entries", get(time_entry_handler::list_time_entries).post(time_entry_handler::create_time_entry))
        .route(
            "/api/v1/time-entries/{id}",
            get(time_entry_handler::get_time_entry)
                .put(time_entry_handler::update_time_entry)
                .delete(time_entry_handler::delete_time_entry),
        )
        .route("/api/v1/time-entries/{id}/transition", put(time_entry_handler::transition_time_entry))
        // Fiscal Years (read + create)
        .route("/api/v1/fiscal-years", get(fiscal_year_handler::list_fiscal_years).post(fiscal_year_handler::create_fiscal_year))
        .route(
            "/api/v1/fiscal-years/{id}",
            get(fiscal_year_handler::get_fiscal_year)
                .put(fiscal_year_handler::update_fiscal_year),
        )
        // Exchange Rates
        .route("/api/v1/exchange-rates", get(exchange_rate_handler::list_exchange_rates).post(exchange_rate_handler::create_exchange_rate))
        .route("/api/v1/exchange-rates/latest", get(exchange_rate_handler::get_latest_rate))
        .route(
            "/api/v1/exchange-rates/{id}",
            get(exchange_rate_handler::get_exchange_rate)
                .put(exchange_rate_handler::update_exchange_rate)
                .delete(exchange_rate_handler::delete_exchange_rate),
        )
        // Invoices
        .route("/api/v1/invoices", get(invoice_handler::list_invoices).post(invoice_handler::create_invoice))
        .route("/api/v1/invoices/from-time-entries", post(invoice_handler::create_invoice_from_time_entries))
        .route(
            "/api/v1/invoices/{id}",
            get(invoice_handler::get_invoice)
                .put(invoice_handler::update_invoice)
                .delete(invoice_handler::delete_invoice),
        )
        .route("/api/v1/invoices/{id}/send", post(invoice_handler::send_invoice))
        .route("/api/v1/invoices/{id}/pay", post(invoice_handler::pay_invoice))
        .route("/api/v1/invoices/{id}/cancel", post(invoice_handler::cancel_invoice))
        .route("/api/v1/invoices/{id}/pdf", get(invoice_handler::download_invoice_pdf))
        .route("/api/v1/invoices/{id}/email", post(invoice_handler::email_invoice))
        .route("/api/v1/invoices/{id}/duplicate", post(invoice_handler::duplicate_invoice))
        .route("/api/v1/invoices/{id}/payments", get(invoice_handler::list_payments).post(invoice_handler::record_payment))
        .route("/api/v1/invoices/{id}/dunning", get(dunning_handler::get_invoice_dunning_history).post(dunning_handler::send_reminder))
        // Recurring Invoices
        .route("/api/v1/recurring-invoices", get(recurring_invoice_handler::list_recurring_invoices).post(recurring_invoice_handler::create_recurring_invoice))
        .route("/api/v1/recurring-invoices/trigger", post(recurring_invoice_handler::trigger_recurring_invoices))
        .route(
            "/api/v1/recurring-invoices/{id}",
            get(recurring_invoice_handler::get_recurring_invoice)
                .put(recurring_invoice_handler::update_recurring_invoice)
                .delete(recurring_invoice_handler::delete_recurring_invoice),
        )
        // Credit Notes
        .route("/api/v1/credit-notes", get(credit_note_handler::list_credit_notes).post(credit_note_handler::create_credit_note))
        .route(
            "/api/v1/credit-notes/{id}",
            get(credit_note_handler::get_credit_note)
                .put(credit_note_handler::update_credit_note)
                .delete(credit_note_handler::delete_credit_note),
        )
        .route("/api/v1/credit-notes/{id}/issue", post(credit_note_handler::issue_credit_note))
        .route("/api/v1/credit-notes/{id}/apply", post(credit_note_handler::apply_credit_note))
        .route("/api/v1/credit-notes/{id}/cancel", post(credit_note_handler::cancel_credit_note))
        .route("/api/v1/credit-notes/{id}/pdf", get(credit_note_handler::download_credit_note_pdf))
        // Expenses
        .route("/api/v1/expenses", get(expense_handler::list_expenses).post(expense_handler::create_expense))
        .route(
            "/api/v1/expenses/{id}",
            get(expense_handler::get_expense)
                .put(expense_handler::update_expense)
                .delete(expense_handler::delete_expense),
        )
        .route("/api/v1/expenses/{id}/approve", post(expense_handler::approve_expense))
        .route("/api/v1/expenses/{id}/pay", post(expense_handler::pay_expense))
        .route("/api/v1/expenses/{id}/cancel", post(expense_handler::cancel_expense))
        .route("/api/v1/expenses/{id}/receipt", post(expense_handler::upload_receipt))
        .route("/api/v1/expenses/{id}/receipts", get(expense_receipt_handler::list_receipts).post(expense_receipt_handler::upload_receipt))
        .route("/api/v1/expenses/receipts/{id}/download", get(expense_receipt_handler::download_receipt))
        .route("/api/v1/expenses/receipts/{id}", delete(expense_receipt_handler::delete_receipt))
        // Expense Categories (read)
        .route("/api/v1/expense-categories", get(expense_category_handler::list_expense_categories).post(expense_category_handler::create_expense_category))
        .route(
            "/api/v1/expense-categories/{id}",
            put(expense_category_handler::update_expense_category)
                .delete(expense_category_handler::delete_expense_category),
        )
        // Dunning (read)
        .route("/api/v1/dunning/levels", get(dunning_handler::list_dunning_levels))
        // Email Settings (read)
        .route("/api/v1/settings/email", get(email_handler::get_email_settings))
        // Email Templates (read + preview)
        .route("/api/v1/email-templates", get(email_template_handler::list_email_templates))
        .route("/api/v1/email-templates/{id}", get(email_template_handler::get_email_template))
        .route("/api/v1/email-templates/{id}/preview", post(email_template_handler::preview_email_template))
        // Company Settings (read)
        .route("/api/v1/settings", get(settings_handler::get_settings))
        // Default Accounts (read)
        .route("/api/v1/settings/default-accounts", get(default_account_handler::get_default_accounts))
        // Bank Accounts
        .route("/api/v1/bank-accounts", get(bank_account_handler::list_bank_accounts).post(bank_account_handler::create_bank_account))
        .route(
            "/api/v1/bank-accounts/{id}",
            put(bank_account_handler::update_bank_account)
                .delete(bank_account_handler::delete_bank_account),
        )
        // Templates
        .route("/api/v1/templates", get(template_handler::list_templates).post(template_handler::create_template))
        .route(
            "/api/v1/templates/{id}",
            get(template_handler::get_template)
                .put(template_handler::update_template)
                .delete(template_handler::delete_template),
        )
        .route("/api/v1/templates/{id}/duplicate", post(template_handler::duplicate_template))
        // Documents
        .route("/api/v1/documents", get(document_handler::list_documents).post(document_handler::create_document))
        .route(
            "/api/v1/documents/{id}",
            get(document_handler::get_document)
                .put(document_handler::update_document)
                .delete(document_handler::delete_document),
        )
        .route("/api/v1/documents/{id}/send", post(document_handler::send_document))
        .route("/api/v1/documents/{id}/accept", post(document_handler::accept_document))
        .route("/api/v1/documents/{id}/reject", post(document_handler::reject_document))
        .route("/api/v1/documents/{id}/convert", post(document_handler::convert_document))
        .route("/api/v1/documents/{id}/pdf", get(document_handler::get_document_pdf))
        // Reports
        .route("/api/v1/reports/trial-balance", get(report_handler::trial_balance))
        .route("/api/v1/reports/balance-sheet", get(report_handler::balance_sheet))
        .route("/api/v1/reports/profit-loss", get(report_handler::profit_loss))
        .route("/api/v1/reports/ledger/{account_id}", get(report_handler::account_ledger))
        .route("/api/v1/reports/vat", get(report_handler::vat_report))
        .route("/api/v1/reports/vat/payment", post(report_handler::create_vat_payment))
        .route("/api/v1/reports/vat/xml", post(report_handler::export_vat_xml))
        // Dashboard
        .route("/api/v1/dashboard", get(report_handler::dashboard_stats))
        .route("/api/v1/dashboard/monthly-revenue", get(dashboard_charts_handler::monthly_revenue))
        .route("/api/v1/dashboard/monthly-expenses", get(dashboard_charts_handler::monthly_expenses))
        .route("/api/v1/dashboard/invoice-aging", get(dashboard_charts_handler::invoice_aging))
        .route("/api/v1/dashboard/top-outstanding", get(dashboard_charts_handler::top_outstanding))
        .route("/api/v1/dashboard/overview", get(dashboard_charts_handler::dashboard_overview))
        // Additional Reports
        .route("/api/v1/reports/cash-flow", get(dashboard_charts_handler::cash_flow))
        .route("/api/v1/reports/cash-flow/monthly", get(dashboard_charts_handler::cash_flow_monthly))
        .route("/api/v1/reports/ar-aging", get(dashboard_charts_handler::ar_aging))
        .route("/api/v1/reports/ap-aging", get(dashboard_charts_handler::ap_aging))
        // Bank Transactions
        .route("/api/v1/bank-transactions", get(bank_transaction_handler::list_bank_transactions))
        .route("/api/v1/bank-transactions/import/{bank_account_id}", post(bank_transaction_handler::import_camt053))
        .route("/api/v1/bank-transactions/auto-match/{bank_account_id}", post(bank_transaction_handler::auto_match))
        .route("/api/v1/bank-transactions/{id}/match", post(bank_transaction_handler::manual_match))
        .route("/api/v1/bank-transactions/{id}/journal", post(bank_transaction_handler::create_journal_from_tx))
        .route("/api/v1/bank-transactions/{id}/ignore", post(bank_transaction_handler::ignore_transaction))
        // VAT Rates (read)
        .route("/api/v1/vat-rates", get(vat_rate_handler::list_vat_rates))
        // Currencies (read)
        .route("/api/v1/currencies", get(currency_handler::list_currencies))
        // Activity Types
        .route("/api/v1/activity-types", get(activity_type_handler::list_activity_types).post(activity_type_handler::create_activity_type))
        .route("/api/v1/activity-types/{id}", put(activity_type_handler::update_activity_type).delete(activity_type_handler::delete_activity_type))
        // Shareholders (read)
        .route("/api/v1/shareholders", get(annual_report_handler::list_shareholders))
        // Annual Report Notes
        .route("/api/v1/fiscal-years/{id}/notes", get(annual_report_handler::list_notes).post(annual_report_handler::create_note))
        .route("/api/v1/fiscal-years/{id}/notes/{section}", get(annual_report_handler::get_note).put(annual_report_handler::update_note).delete(annual_report_handler::delete_note))
        // Annual Report (read)
        .route("/api/v1/fiscal-years/{id}/annual-report", get(annual_report_handler::get_annual_report))
        .route("/api/v1/fiscal-years/{id}/annual-report/pdf", get(annual_report_handler::download_pdf))
        // Contact Relationships
        .route("/api/v1/contacts/{id}/relationships", get(contact_relationship_handler::list_relationships).post(contact_relationship_handler::create_relationship))
        .route("/api/v1/contact-relationships/{id}", put(contact_relationship_handler::update_relationship).delete(contact_relationship_handler::delete_relationship))
        // Rate Functions
        .route("/api/v1/rate-functions", get(rate_function_handler::list_rate_functions).post(rate_function_handler::create_rate_function))
        .route("/api/v1/rate-functions/{id}", put(rate_function_handler::update_rate_function).delete(rate_function_handler::delete_rate_function))
        // Project Members
        .route("/api/v1/projects/{id}/members", get(project_member_handler::list_project_members).post(project_member_handler::add_project_member))
        .route("/api/v1/projects/{id}/members/{mid}", put(project_member_handler::update_project_member).delete(project_member_handler::remove_project_member))
        // Project Activity Types
        .route("/api/v1/projects/{id}/activity-types", get(project_activity_type_handler::list_project_activity_types).post(project_activity_type_handler::add_project_activity_type))
        .route("/api/v1/projects/{id}/activity-types/{pat_id}", put(project_activity_type_handler::update_project_activity_type).delete(project_activity_type_handler::remove_project_activity_type))
        // Project Items (WBS)
        .route("/api/v1/projects/{id}/items", get(project_item_handler::list_project_items).post(project_item_handler::create_project_item))
        .route("/api/v1/project-items/{id}", get(project_item_handler::get_project_item).put(project_item_handler::update_project_item).delete(project_item_handler::delete_project_item))
        .route("/api/v1/project-items/{id}/reorder", put(project_item_handler::reorder_project_item))
        // Project Milestones
        .route("/api/v1/projects/{id}/milestones", get(project_milestone_handler::list_project_milestones).post(project_milestone_handler::create_project_milestone))
        .route("/api/v1/project-milestones/{id}", put(project_milestone_handler::update_project_milestone).delete(project_milestone_handler::delete_project_milestone))
        .route("/api/v1/project-milestones/{id}/reach", post(project_milestone_handler::reach_project_milestone))
        // Project Documents
        .route("/api/v1/projects/{id}/files", get(project_document_handler::list_project_files).post(project_document_handler::upload_project_file))
        .route("/api/v1/project-files/{id}/download", get(project_document_handler::download_project_file))
        .route("/api/v1/project-files/{id}", delete(project_document_handler::delete_project_file))
        // Timesheets
        .route("/api/v1/timesheets", get(timesheet_handler::list_timesheets).post(timesheet_handler::create_timesheet))
        .route("/api/v1/timesheets/{id}", get(timesheet_handler::get_timesheet).put(timesheet_handler::update_timesheet).delete(timesheet_handler::delete_timesheet))
        .route("/api/v1/timesheets/{id}/submit", post(timesheet_handler::submit_timesheet))
        .route("/api/v1/timesheets/{id}/approve", post(timesheet_handler::approve_timesheet))
        .route("/api/v1/timesheets/{id}/reject", post(timesheet_handler::reject_timesheet))
        // Swiss Reports
        .route("/api/v1/reports/swiss-balance-sheet", get(annual_report_handler::swiss_balance_sheet))
        .route("/api/v1/reports/swiss-income-statement", get(annual_report_handler::swiss_income_statement))
        // Fixed Assets
        .route("/api/v1/fixed-assets", get(fixed_asset_handler::list_fixed_assets).post(fixed_asset_handler::create_fixed_asset))
        .route("/api/v1/fixed-assets/{id}", get(fixed_asset_handler::get_fixed_asset).put(fixed_asset_handler::update_fixed_asset).delete(fixed_asset_handler::delete_fixed_asset))
        .route("/api/v1/fixed-assets/{id}/schedule", get(fixed_asset_handler::get_depreciation_schedule))
        // Employees (read)
        .route("/api/v1/employees", get(employee_handler::list_employees))
        .route("/api/v1/employees/{id}", get(employee_handler::get_employee))
        // Payroll Settings (read)
        .route("/api/v1/payroll-settings", get(payroll_settings_handler::get_payroll_settings))
        // Payroll Runs (read + payslips)
        .route("/api/v1/payroll-runs", get(payroll_run_handler::list_payroll_runs))
        .route("/api/v1/payroll-runs/{id}", get(payroll_run_handler::get_payroll_run))
        .route("/api/v1/payroll-runs/{id}/payslip/{employee_id}", get(payroll_run_handler::download_payslip))
        .route("/api/v1/payroll-runs/{id}/payslips", get(payroll_run_handler::download_payslips_zip))
        // Payout Entries (read)
        .route("/api/v1/payroll-runs/{id}/payout-entries", get(payout_handler::list_payout_entries))
        // Project Sub-Statuses (read)
        .route("/api/v1/project-sub-statuses", get(project_sub_status_handler::list_project_sub_statuses));

    // Combine: auth middleware wraps all protected groups
    let protected = Router::new()
        .merge(admin_routes)
        .merge(auditor_routes)
        .merge(authenticated_routes)
        .layer(middleware::from_fn_with_state(state.clone(), auth_middleware));

    Router::new()
        .merge(public)
        .merge(protected)
        .nest_service("/uploads", ServeDir::new("uploads"))
        .with_state(state)
}
