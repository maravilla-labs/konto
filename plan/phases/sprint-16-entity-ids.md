# Sprint 16: Entity IDs, Employee-User Link & Project Enhancements

## Goal
Add configurable auto-numbering to contacts (customer numbers) and employees (personnel numbers), bidirectional employee-user linking with provisioning, and project owner assignment.

## Migrations (5 files)

| Migration | Description |
|-----------|-------------|
| M083 | Add 10 numbering columns to company_settings (customer + employee) |
| M084 | Add customer_number to contacts with partial unique index |
| M085 | Add number to employees with partial unique index |
| M086 | Add owner_id to projects, employee_id to users |
| M087 | Backfill customer_number from legacy import ID |

## Backend Changes

### Entities
- contact.rs: `customer_number`
- employee.rs: `number`
- project.rs: `owner_id`
- user.rs: `employee_id`
- company_setting.rs: 10 numbering fields

### Services
- contact_service.rs: `auto_assign_customer_number()`
- employee_service.rs: `auto_assign_number()`
- employee_user_link_service.rs: NEW — provision_user, link, unlink
- project_service.rs: owner_id in create/update
- pdf_invoice.rs: customer_number with legacy_id fallback

### DTOs
- contact.rs, employee.rs, project.rs, user.rs, settings.rs updated
- New: CreateEmployeeResponse, ProvisionedUserInfo

## Frontend Changes

### New Components
- NumberingCard.tsx — reusable auto-numbering settings card
- TempPasswordDialog.tsx — temp password display with copy

### Updated Pages
- CompanySettingsPage: 3 NumberingCard instances (project/customer/employee)
- ContactsPage: customer_number column
- ContactOverview: customer_number field
- ContactDetailPage: customer_number in header
- EmployeesPage: number column, "Grant system access" toggle
- InvoiceForm: project number in dropdown
- ProjectEditDialog: owner Select from employees

### i18n
~20 new keys in en/de/fr/it

## Technical Decisions
- TD-054: Customer number auto-assignment
- TD-055: Employee number auto-assignment
- TD-056: Bidirectional employee-user link
- TD-057: Project owner via employee FK
