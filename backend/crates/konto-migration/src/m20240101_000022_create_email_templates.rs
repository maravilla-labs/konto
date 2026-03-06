use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

fn now_str() -> String {
    chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(EmailTemplates::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(EmailTemplates::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(EmailTemplates::TemplateType)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(EmailTemplates::Subject).text().not_null())
                    .col(ColumnDef::new(EmailTemplates::BodyHtml).text().not_null())
                    .col(
                        ColumnDef::new(EmailTemplates::Language)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EmailTemplates::IsDefault)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(EmailTemplates::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EmailTemplates::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Unique constraint on (template_type, language)
        manager
            .create_index(
                Index::create()
                    .name("idx_email_templates_type_lang")
                    .table(EmailTemplates::Table)
                    .col(EmailTemplates::TemplateType)
                    .col(EmailTemplates::Language)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Seed default templates
        let now = now_str();
        for (id, ttype, lang, subject, body) in seed_templates() {
            let sql = format!(
                "INSERT INTO email_templates \
                 (id, template_type, subject, body_html, language, \
                  is_default, created_at, updated_at) \
                 VALUES ('{id}', '{ttype}', '{subj}', '{body}', \
                  '{lang}', 1, '{now}', '{now}')",
                subj = subject.replace('\'', "''"),
                body = body.replace('\'', "''"),
            );
            manager
                .get_connection()
                .execute_unprepared(&sql)
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(EmailTemplates::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum EmailTemplates {
    Table,
    Id,
    TemplateType,
    Subject,
    BodyHtml,
    Language,
    IsDefault,
    CreatedAt,
    UpdatedAt,
}

fn seed_templates() -> Vec<(&'static str, &'static str, &'static str, String, String)> {
    vec![
        // invoice_send
        (
            "etpl-invoice-send-de",
            "invoice_send",
            "de",
            "Rechnung {{invoice_number}}".into(),
            "Guten Tag {{contact_name}},\n\n\
             Anbei erhalten Sie die Rechnung {{invoice_number}} \
             über {{amount}} {{currency}}.\n\n\
             Zahlbar bis: {{due_date}}\n\n\
             Freundliche Grüsse\n{{company_name}}"
                .into(),
        ),
        (
            "etpl-invoice-send-en",
            "invoice_send",
            "en",
            "Invoice {{invoice_number}}".into(),
            "Dear {{contact_name}},\n\n\
             Please find attached invoice {{invoice_number}} \
             for {{amount}} {{currency}}.\n\n\
             Due date: {{due_date}}\n\n\
             Best regards,\n{{company_name}}"
                .into(),
        ),
        // invoice_reminder_1
        (
            "etpl-reminder1-de",
            "invoice_reminder_1",
            "de",
            "Zahlungserinnerung - {{invoice_number}}".into(),
            "Guten Tag {{contact_name}},\n\n\
             Wir möchten Sie freundlich daran erinnern, dass die \
             Rechnung {{invoice_number}} über {{amount}} {{currency}} \
             am {{due_date}} fällig war.\n\n\
             Falls die Zahlung bereits erfolgt ist, betrachten Sie \
             diese Nachricht bitte als gegenstandslos.\n\n\
             Freundliche Grüsse\n{{company_name}}"
                .into(),
        ),
        (
            "etpl-reminder1-en",
            "invoice_reminder_1",
            "en",
            "Payment Reminder - {{invoice_number}}".into(),
            "Dear {{contact_name}},\n\n\
             This is a friendly reminder that invoice \
             {{invoice_number}} for {{amount}} {{currency}} \
             was due on {{due_date}}.\n\n\
             If payment has already been made, please disregard \
             this notice.\n\n\
             Best regards,\n{{company_name}}"
                .into(),
        ),
        // invoice_reminder_2
        (
            "etpl-reminder2-de",
            "invoice_reminder_2",
            "de",
            "2. Mahnung - {{invoice_number}}".into(),
            "Guten Tag {{contact_name}},\n\n\
             Trotz unserer ersten Erinnerung ist die Zahlung der \
             Rechnung {{invoice_number}} über {{amount}} {{currency}} \
             noch nicht eingegangen. Die Zahlung war am {{due_date}} fällig.\n\n\
             Wir bitten Sie, den ausstehenden Betrag umgehend zu \
             überweisen. Bei weiterem Verzug behalten wir uns vor, \
             Mahngebühren zu erheben.\n\n\
             Freundliche Grüsse\n{{company_name}}"
                .into(),
        ),
        (
            "etpl-reminder2-en",
            "invoice_reminder_2",
            "en",
            "Second Reminder - {{invoice_number}}".into(),
            "Dear {{contact_name}},\n\n\
             Despite our previous reminder, we have not yet received \
             payment for invoice {{invoice_number}} of \
             {{amount}} {{currency}}, which was due on {{due_date}}.\n\n\
             Please arrange payment immediately. Late fees may apply \
             if the balance remains unpaid.\n\n\
             Best regards,\n{{company_name}}"
                .into(),
        ),
        // invoice_reminder_3
        (
            "etpl-reminder3-de",
            "invoice_reminder_3",
            "de",
            "Letzte Mahnung - {{invoice_number}}".into(),
            "Guten Tag {{contact_name}},\n\n\
             Dies ist unsere letzte Mahnung bezüglich der Rechnung \
             {{invoice_number}} über {{amount}} {{currency}} \
             (fällig am {{due_date}}).\n\n\
             Ohne Zahlung innert 10 Tagen sehen wir uns gezwungen, \
             weitere Schritte einzuleiten.\n\n\
             Freundliche Grüsse\n{{company_name}}"
                .into(),
        ),
        (
            "etpl-reminder3-en",
            "invoice_reminder_3",
            "en",
            "Final Notice - {{invoice_number}}".into(),
            "Dear {{contact_name}},\n\n\
             This is our final notice regarding invoice \
             {{invoice_number}} for {{amount}} {{currency}} \
             (due on {{due_date}}).\n\n\
             If payment is not received within 10 days, we will \
             be forced to take further action.\n\n\
             Best regards,\n{{company_name}}"
                .into(),
        ),
        // credit_note
        (
            "etpl-credit-note-de",
            "credit_note",
            "de",
            "Gutschrift {{credit_note_number}}".into(),
            "Guten Tag {{contact_name}},\n\n\
             Anbei erhalten Sie die Gutschrift {{credit_note_number}} \
             über {{amount}} {{currency}}.\n\n\
             Freundliche Grüsse\n{{company_name}}"
                .into(),
        ),
        (
            "etpl-credit-note-en",
            "credit_note",
            "en",
            "Credit Note {{credit_note_number}}".into(),
            "Dear {{contact_name}},\n\n\
             Please find attached credit note {{credit_note_number}} \
             for {{amount}} {{currency}}.\n\n\
             Best regards,\n{{company_name}}"
                .into(),
        ),
        // document_send
        (
            "etpl-document-de",
            "document_send",
            "de",
            "Dokument {{document_number}}".into(),
            "Guten Tag {{contact_name}},\n\n\
             Anbei erhalten Sie das Dokument {{document_number}}.\n\n\
             Freundliche Grüsse\n{{company_name}}"
                .into(),
        ),
        (
            "etpl-document-en",
            "document_send",
            "en",
            "Document {{document_number}}".into(),
            "Dear {{contact_name}},\n\n\
             Please find attached document {{document_number}}.\n\n\
             Best regards,\n{{company_name}}"
                .into(),
        ),
    ]
}
