use chrono::Utc;
use konto_common::error::AppError;
use konto_db::entities::email_setting;
use konto_db::repository::email_settings_repo::EmailSettingsRepo;
use lettre::message::{Attachment, MultiPart, SinglePart, header::ContentType};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use sea_orm::{DatabaseConnection, Set};
use uuid::Uuid;

pub struct EmailService;

impl EmailService {
    pub async fn get_settings(
        db: &DatabaseConnection,
    ) -> Result<Option<email_setting::Model>, AppError> {
        EmailSettingsRepo::find_first(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn update_settings(
        db: &DatabaseConnection,
        smtp_host: &str,
        smtp_port: i32,
        smtp_username: &str,
        smtp_password: Option<&str>,
        smtp_encryption: &str,
        from_email: &str,
        from_name: &str,
        reply_to_email: Option<String>,
        bcc_email: Option<String>,
        is_active: bool,
    ) -> Result<email_setting::Model, AppError> {
        let now = Utc::now().naive_utc();
        let existing = EmailSettingsRepo::find_first(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let (id, password) = match &existing {
            Some(ex) => {
                let pwd = match smtp_password {
                    Some(p) if p != "********" && !p.is_empty() => p.to_string(),
                    _ => ex.smtp_password.clone(),
                };
                (ex.id.clone(), pwd)
            }
            None => {
                let pwd = smtp_password.unwrap_or_default().to_string();
                (Uuid::new_v4().to_string(), pwd)
            }
        };

        let model = email_setting::ActiveModel {
            id: Set(id),
            smtp_host: Set(smtp_host.to_string()),
            smtp_port: Set(smtp_port),
            smtp_username: Set(smtp_username.to_string()),
            smtp_password: Set(password),
            smtp_encryption: Set(smtp_encryption.to_string()),
            from_email: Set(from_email.to_string()),
            from_name: Set(from_name.to_string()),
            reply_to_email: Set(reply_to_email),
            bcc_email: Set(bcc_email),
            is_active: Set(is_active),
            created_at: Set(existing
                .as_ref()
                .map(|e| e.created_at)
                .unwrap_or(now)),
            updated_at: Set(now),
        };

        EmailSettingsRepo::upsert(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn send_test_email(
        db: &DatabaseConnection,
    ) -> Result<(), AppError> {
        let settings = Self::get_settings(db)
            .await?
            .ok_or_else(|| AppError::BadRequest("Email not configured".into()))?;

        if !settings.is_active {
            return Err(AppError::BadRequest("Email is not active".into()));
        }

        Self::send_email_smtp(
            &settings,
            &settings.from_email,
            "Hope Test Email",
            "This is a test email from Hope Accounting.",
            vec![],
        )
        .await
    }

    pub async fn send_email(
        db: &DatabaseConnection,
        to: &str,
        subject: &str,
        body: &str,
        attachments: Vec<(&str, &[u8])>,
    ) -> Result<(), AppError> {
        let settings = Self::get_settings(db)
            .await?
            .ok_or_else(|| AppError::BadRequest("Email not configured".into()))?;

        if !settings.is_active {
            return Err(AppError::BadRequest("Email is not active".into()));
        }

        Self::send_email_smtp(&settings, to, subject, body, attachments).await
    }

    async fn send_email_smtp(
        settings: &email_setting::Model,
        to: &str,
        subject: &str,
        body: &str,
        attachments: Vec<(&str, &[u8])>,
    ) -> Result<(), AppError> {
        let from = format!("{} <{}>", settings.from_name, settings.from_email)
            .parse()
            .map_err(|e| AppError::BadRequest(format!("Invalid from address: {e}")))?;

        let to_addr = to
            .parse()
            .map_err(|e| AppError::BadRequest(format!("Invalid to address: {e}")))?;

        let mut builder = Message::builder()
            .from(from)
            .to(to_addr)
            .subject(subject);

        if let Some(reply_to) = settings.reply_to_email.as_ref().filter(|r| !r.is_empty()) {
            builder = builder.reply_to(
                reply_to
                    .parse()
                    .map_err(|e| AppError::BadRequest(format!("Invalid reply-to: {e}")))?,
            );
        }

        if let Some(bcc) = settings.bcc_email.as_ref().filter(|b| !b.is_empty()) {
            builder = builder.bcc(
                bcc.parse()
                    .map_err(|e| AppError::BadRequest(format!("Invalid BCC: {e}")))?,
            );
        }

        let email = if attachments.is_empty() {
            builder
                .body(body.to_string())
                .map_err(|e| AppError::Internal(format!("Failed to build email: {e}")))?
        } else {
            let text_part = SinglePart::builder()
                .content_type(ContentType::TEXT_PLAIN)
                .body(body.to_string());

            let mut multi = MultiPart::mixed().singlepart(text_part);

            for (filename, data) in &attachments {
                let ct_str = if filename.ends_with(".pdf") { "application/pdf" } else { "application/octet-stream" };
                let content_type = ContentType::parse(ct_str)
                    .map_err(|e| AppError::Internal(format!("Invalid content type: {e}")))?;
                let attachment =
                    Attachment::new(filename.to_string()).body(data.to_vec(), content_type);
                multi = multi.singlepart(attachment);
            }

            builder
                .multipart(multi)
                .map_err(|e| AppError::Internal(format!("Failed to build email: {e}")))?
        };

        let transport = build_transport(settings)?;

        transport
            .send(email)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to send email: {e}")))?;

        Ok(())
    }
}

fn build_transport(
    settings: &email_setting::Model,
) -> Result<AsyncSmtpTransport<Tokio1Executor>, AppError> {
    let creds = Credentials::new(
        settings.smtp_username.clone(),
        settings.smtp_password.clone(),
    );

    let transport = match settings.smtp_encryption.as_str() {
        "ssl" | "tls" => AsyncSmtpTransport::<Tokio1Executor>::relay(&settings.smtp_host)
            .map_err(|e| AppError::Internal(format!("SMTP relay error: {e}")))?
            .port(settings.smtp_port as u16)
            .credentials(creds)
            .build(),
        "starttls" => {
            AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&settings.smtp_host)
                .map_err(|e| AppError::Internal(format!("SMTP STARTTLS error: {e}")))?
                .port(settings.smtp_port as u16)
                .credentials(creds)
                .build()
        }
        _ => AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&settings.smtp_host)
            .map_err(|e| AppError::Internal(format!("SMTP error: {e}")))?
            .port(settings.smtp_port as u16)
            .credentials(creds)
            .build(),
    };

    Ok(transport)
}
