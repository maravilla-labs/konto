use konto_common::error::AppError;
use konto_common::enums::PayrollRunStatus;
use konto_db::repository::bank_account_repo::BankAccountRepo;
use konto_db::repository::payroll_run_repo::PayrollRunRepo;
use konto_db::repository::payroll_setting_repo::PayrollSettingRepo;
use konto_db::repository::payout_entry_repo::PayoutEntryRepo;
use konto_db::repository::settings_repo::SettingsRepo;
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use rust_decimal::prelude::ToPrimitive;
use sea_orm::DatabaseConnection;
use std::io::Cursor;
use uuid::Uuid;

pub struct Pain001Service;

impl Pain001Service {
    pub async fn generate(
        db: &DatabaseConnection,
        payroll_run_id: &str,
    ) -> Result<Vec<u8>, AppError> {
        let run = PayrollRunRepo::find_by_id(db, payroll_run_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Payroll run not found".into()))?;

        if run.status != PayrollRunStatus::Approved.as_str() && run.status != PayrollRunStatus::Paid.as_str() {
            return Err(AppError::Validation(format!(
                "Cannot export pain.001 for run with status '{}'. Must be 'approved' or 'paid'.",
                run.status
            )));
        }

        let entries = PayoutEntryRepo::find_by_run_id(db, payroll_run_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        if entries.is_empty() {
            return Err(AppError::Validation(
                "No payout entries found. Generate payouts first.".into(),
            ));
        }

        let settings = SettingsRepo::find(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Company settings not found".into()))?;

        let payroll_settings = PayrollSettingRepo::find(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Payroll settings not found".into()))?;

        let bank_account_id = payroll_settings
            .payment_bank_account_id
            .as_ref()
            .ok_or_else(|| {
                AppError::Validation(
                    "Payment bank account not configured in payroll settings".into(),
                )
            })?;

        let bank_account = BankAccountRepo::find_by_id(db, bank_account_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Payment bank account not found".into()))?;

        let clearing_number = payroll_settings
            .company_clearing_number
            .as_ref()
            .cloned()
            .unwrap_or_default();

        // Calculate totals
        let total_amount: rust_decimal::Decimal = entries.iter().map(|e| e.amount).sum();
        let nb_of_txs = entries.len();

        // Calculate last business day of the payroll month
        let exec_date = last_business_day(run.period_year, run.period_month as u32);

        let msg_id = Uuid::new_v4().to_string();
        let pmt_inf_id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now();

        let mut writer = Writer::new(Cursor::new(Vec::new()));

        // XML declaration
        writer
            .write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))
            .map_err(|e| AppError::Internal(e.to_string()))?;

        // Document root
        let mut doc = BytesStart::new("Document");
        doc.push_attribute((
            "xmlns",
            "urn:iso:std:iso:20022:tech:xsd:pain.001.001.09.ch.03",
        ));
        writer
            .write_event(Event::Start(doc))
            .map_err(|e| AppError::Internal(e.to_string()))?;

        // CstmrCdtTrfInitn
        write_start(&mut writer, "CstmrCdtTrfInitn")?;

        // GrpHdr
        write_start(&mut writer, "GrpHdr")?;
        write_element(&mut writer, "MsgId", &msg_id)?;
        write_element(
            &mut writer,
            "CreDtTm",
            &now.format("%Y-%m-%dT%H:%M:%S").to_string(),
        )?;
        write_element(&mut writer, "NbOfTxs", &nb_of_txs.to_string())?;
        write_element(
            &mut writer,
            "CtrlSum",
            &format!("{:.2}", total_amount.to_f64().unwrap_or(0.0)),
        )?;
        write_start(&mut writer, "InitgPty")?;
        write_element(&mut writer, "Nm", &settings.legal_name)?;
        write_end(&mut writer, "InitgPty")?;
        write_end(&mut writer, "GrpHdr")?;

        // PmtInf
        write_start(&mut writer, "PmtInf")?;
        write_element(&mut writer, "PmtInfId", &pmt_inf_id)?;
        write_element(&mut writer, "PmtMtd", "TRF")?;
        write_element(&mut writer, "NbOfTxs", &nb_of_txs.to_string())?;
        write_element(
            &mut writer,
            "CtrlSum",
            &format!("{:.2}", total_amount.to_f64().unwrap_or(0.0)),
        )?;

        // PmtTpInf
        write_start(&mut writer, "PmtTpInf")?;
        write_start(&mut writer, "CtgyPurp")?;
        write_element(&mut writer, "Cd", "SALA")?;
        write_end(&mut writer, "CtgyPurp")?;
        write_end(&mut writer, "PmtTpInf")?;

        // ReqdExctnDt
        write_start(&mut writer, "ReqdExctnDt")?;
        write_element(&mut writer, "Dt", &exec_date)?;
        write_end(&mut writer, "ReqdExctnDt")?;

        // Dbtr (debtor = company)
        write_start(&mut writer, "Dbtr")?;
        write_element(&mut writer, "Nm", &settings.legal_name)?;
        write_start(&mut writer, "PstlAdr")?;
        write_element(&mut writer, "StrtNm", &settings.street)?;
        write_element(&mut writer, "PstCd", &settings.postal_code)?;
        write_element(&mut writer, "TwnNm", &settings.city)?;
        write_element(&mut writer, "Ctry", &settings.country)?;
        write_end(&mut writer, "PstlAdr")?;
        write_end(&mut writer, "Dbtr")?;

        // DbtrAcct
        write_start(&mut writer, "DbtrAcct")?;
        write_start(&mut writer, "Id")?;
        write_element(&mut writer, "IBAN", &bank_account.iban)?;
        write_end(&mut writer, "Id")?;
        write_end(&mut writer, "DbtrAcct")?;

        // DbtrAgt
        write_start(&mut writer, "DbtrAgt")?;
        write_start(&mut writer, "FinInstnId")?;
        if !clearing_number.is_empty() {
            write_start(&mut writer, "ClrSysMmbId")?;
            write_start(&mut writer, "ClrSysId")?;
            write_element(&mut writer, "Cd", "CHBCC")?;
            write_end(&mut writer, "ClrSysId")?;
            write_element(&mut writer, "MmbId", &clearing_number)?;
            write_end(&mut writer, "ClrSysMmbId")?;
        }
        if let Some(bic) = bank_account.bic.as_ref().filter(|b| !b.is_empty()) {
            write_element(&mut writer, "BICFI", bic)?;
        }
        write_end(&mut writer, "FinInstnId")?;
        write_end(&mut writer, "DbtrAgt")?;

        // Credit transfer transactions
        for entry in &entries {
            write_start(&mut writer, "CdtTrfTxInf")?;

            // PmtId
            write_start(&mut writer, "PmtId")?;
            write_element(&mut writer, "InstrId", &Uuid::new_v4().to_string())?;
            write_element(&mut writer, "EndToEndId", &entry.payment_reference)?;
            write_end(&mut writer, "PmtId")?;

            // Amt
            write_start(&mut writer, "Amt")?;
            let mut instd = BytesStart::new("InstdAmt");
            instd.push_attribute(("Ccy", "CHF"));
            writer
                .write_event(Event::Start(instd))
                .map_err(|e| AppError::Internal(e.to_string()))?;
            writer
                .write_event(Event::Text(BytesText::new(&format!(
                    "{:.2}",
                    entry.amount.to_f64().unwrap_or(0.0)
                ))))
                .map_err(|e| AppError::Internal(e.to_string()))?;
            writer
                .write_event(Event::End(BytesEnd::new("InstdAmt")))
                .map_err(|e| AppError::Internal(e.to_string()))?;
            write_end(&mut writer, "Amt")?;

            // CdtrAgt
            if let Some(bic) = entry.bic.as_ref().filter(|b| !b.is_empty()) {
                write_start(&mut writer, "CdtrAgt")?;
                write_start(&mut writer, "FinInstnId")?;
                write_element(&mut writer, "BICFI", bic)?;
                write_end(&mut writer, "FinInstnId")?;
                write_end(&mut writer, "CdtrAgt")?;
            }

            // Cdtr (creditor = employee)
            write_start(&mut writer, "Cdtr")?;
            write_element(&mut writer, "Nm", &entry.recipient_name)?;
            write_start(&mut writer, "PstlAdr")?;
            write_element(&mut writer, "StrtNm", &entry.recipient_street)?;
            write_element(&mut writer, "PstCd", &entry.recipient_postal_code)?;
            write_element(&mut writer, "TwnNm", &entry.recipient_city)?;
            write_element(&mut writer, "Ctry", &entry.recipient_country)?;
            write_end(&mut writer, "PstlAdr")?;
            write_end(&mut writer, "Cdtr")?;

            // CdtrAcct
            write_start(&mut writer, "CdtrAcct")?;
            write_start(&mut writer, "Id")?;
            write_element(&mut writer, "IBAN", &entry.iban)?;
            write_end(&mut writer, "Id")?;
            write_end(&mut writer, "CdtrAcct")?;

            // RmtInf
            write_start(&mut writer, "RmtInf")?;
            write_element(
                &mut writer,
                "Ustrd",
                &format!("Lohn {:02} {}", run.period_month, run.period_year),
            )?;
            write_end(&mut writer, "RmtInf")?;

            write_end(&mut writer, "CdtTrfTxInf")?;
        }

        write_end(&mut writer, "PmtInf")?;
        write_end(&mut writer, "CstmrCdtTrfInitn")?;
        write_end(&mut writer, "Document")?;

        let result = writer.into_inner().into_inner();
        Ok(result)
    }
}

fn write_start(writer: &mut Writer<Cursor<Vec<u8>>>, tag: &str) -> Result<(), AppError> {
    writer
        .write_event(Event::Start(BytesStart::new(tag)))
        .map_err(|e| AppError::Internal(e.to_string()))
}

fn write_end(writer: &mut Writer<Cursor<Vec<u8>>>, tag: &str) -> Result<(), AppError> {
    writer
        .write_event(Event::End(BytesEnd::new(tag)))
        .map_err(|e| AppError::Internal(e.to_string()))
}

fn write_element(
    writer: &mut Writer<Cursor<Vec<u8>>>,
    tag: &str,
    value: &str,
) -> Result<(), AppError> {
    writer
        .write_event(Event::Start(BytesStart::new(tag)))
        .map_err(|e| AppError::Internal(e.to_string()))?;
    writer
        .write_event(Event::Text(BytesText::new(value)))
        .map_err(|e| AppError::Internal(e.to_string()))?;
    writer
        .write_event(Event::End(BytesEnd::new(tag)))
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(())
}

#[allow(clippy::expect_used)]
fn last_business_day(year: i32, month: u32) -> String {
    use chrono::{Datelike, NaiveDate};

    let fallback = NaiveDate::from_ymd_opt(year, month, 28)
        .unwrap_or_else(|| NaiveDate::from_ymd_opt(year, 1, 28).expect("Jan 28 is always valid"));

    let next_month = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
    };

    let last_day = next_month
        .unwrap_or(fallback)
        .pred_opt()
        .unwrap_or(fallback);

    // Walk backwards from the last day to find a weekday
    let mut d = last_day;
    while d.weekday() == chrono::Weekday::Sat || d.weekday() == chrono::Weekday::Sun {
        d = d.pred_opt().unwrap_or(d);
    }

    d.format("%Y-%m-%d").to_string()
}
