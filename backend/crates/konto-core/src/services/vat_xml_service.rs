use chrono::{NaiveDate, Utc};
use konto_common::error::AppError;
use konto_db::entities::company_setting;
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use rust_decimal::Decimal;
use sea_orm::*;
use std::io::Cursor;

use super::report_service::ReportService;
use super::report_types::VatReportEntry;

const ECH0217_NS: &str = "http://www.ech.ch/xmlns/eCH-0217/2";
const XSI_NS: &str = "http://www.w3.org/2001/XMLSchema-instance";

type XmlWriter = Writer<Cursor<Vec<u8>>>;

/// Input data for eCH-0217 V2.0.0 XML generation.
pub struct VatDeclarationInput {
    pub from_date: NaiveDate,
    pub to_date: NaiveDate,
    /// 1 = Initial, 2 = Correction, 3 = Annual reconciliation
    pub type_of_submission: i32,
    /// 1 = Agreed (vereinbart), 2 = Received (vereinnahmt)
    pub form_of_reporting: i32,
    pub business_reference_id: Option<String>,
    // Turnover computation (Ziffer 200-280)
    pub total_consideration: Decimal,
    pub supplies_to_foreign: Decimal,
    pub supplies_abroad: Decimal,
    pub transfer_notification: Decimal,
    pub supplies_exempt: Decimal,
    pub reduction_of_consideration: Decimal,
    pub various_deduction: Decimal,
    // Other flows of funds
    pub subsidies: Decimal,
    pub donations: Decimal,
}

pub struct VatXmlService;

impl VatXmlService {
    /// Generate eCH-0217 V2.0.0 XML for MWST declaration.
    pub async fn generate(
        db: &DatabaseConnection,
        input: VatDeclarationInput,
    ) -> Result<Vec<u8>, AppError> {
        let settings = company_setting::Entity::find()
            .one(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::Validation("Company settings not found".into()))?;

        let vat_report = ReportService::vat_report(db, input.from_date, input.to_date).await?;

        let uid = extract_uid(&settings.vat_number);
        let is_flat_rate = settings.vat_method == "flat_rate";

        // Compute turnover deductions
        let total_deduction = input.supplies_to_foreign
            + input.supplies_abroad
            + input.transfer_notification
            + input.supplies_exempt
            + input.reduction_of_consideration
            + input.various_deduction;
        let taxable_turnover = input.total_consideration - total_deduction;

        // Build XML
        let buf = Cursor::new(Vec::new());
        let mut w = Writer::new_with_indent(buf, b' ', 2);

        // XML declaration
        w.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))
            .map_err(io_err)?;

        // Root element
        let mut root = BytesStart::new("VATDeclaration");
        root.push_attribute(("xmlns", ECH0217_NS));
        root.push_attribute(("xmlns:xsi", XSI_NS));
        w.write_event(Event::Start(root)).map_err(io_err)?;

        // generalInformation
        write_general_info(&mut w, &uid, &settings.legal_name, &input, is_flat_rate)?;

        // turnoverComputation
        write_turnover_computation(&mut w, &input, total_deduction, taxable_turnover)?;

        // Tax method section
        if is_flat_rate {
            write_flat_tax_rate_method(
                &mut w,
                &vat_report.output_entries,
                settings.flat_rate_percentage,
            )?;
        } else {
            write_effective_method(
                &mut w,
                &vat_report.output_entries,
                &vat_report.input_entries,
                vat_report.total_input_vat,
            )?;
        }

        // payableTax
        write_payable_tax(&mut w, vat_report.net_vat_owed)?;

        // otherFlowsOfFunds (optional)
        if input.subsidies != Decimal::ZERO || input.donations != Decimal::ZERO {
            write_other_flows(&mut w, input.subsidies, input.donations)?;
        }

        // Close root
        w.write_event(Event::End(BytesEnd::new("VATDeclaration")))
            .map_err(io_err)?;

        Ok(w.into_inner().into_inner())
    }
}

fn extract_uid(vat_number: &Option<String>) -> String {
    match vat_number {
        Some(vn) => {
            let cleaned: String = vn.chars().filter(|c| c.is_ascii_digit()).collect();
            if cleaned.len() >= 9 {
                format!(
                    "CHE-{}.{}.{}",
                    &cleaned[..3],
                    &cleaned[3..6],
                    &cleaned[6..9]
                )
            } else {
                vn.clone()
            }
        }
        None => String::new(),
    }
}

fn io_err(e: std::io::Error) -> AppError {
    AppError::Internal(format!("XML generation error: {e}"))
}

fn start(w: &mut XmlWriter, tag: &str) -> Result<(), AppError> {
    w.write_event(Event::Start(BytesStart::new(tag))).map_err(io_err)
}

fn end(w: &mut XmlWriter, tag: &str) -> Result<(), AppError> {
    w.write_event(Event::End(BytesEnd::new(tag))).map_err(io_err)
}

fn elem(w: &mut XmlWriter, tag: &str, value: &str) -> Result<(), AppError> {
    start(w, tag)?;
    w.write_event(Event::Text(BytesText::new(value))).map_err(io_err)?;
    end(w, tag)
}

fn dec(w: &mut XmlWriter, tag: &str, value: Decimal) -> Result<(), AppError> {
    elem(w, tag, &format!("{:.2}", value))
}

fn write_general_info(
    w: &mut XmlWriter,
    uid: &str,
    org_name: &str,
    input: &VatDeclarationInput,
    is_flat_rate: bool,
) -> Result<(), AppError> {
    start(w, "generalInformation")?;

    elem(w, "uid", uid)?;
    elem(w, "organisationName", org_name)?;
    elem(w, "generationTime", &Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string())?;

    if let Some(bri) = input.business_reference_id.as_ref().filter(|b| !b.is_empty()) {
        elem(w, "businessReferenceId", bri)?;
    }

    elem(w, "typeOfSubmission", &input.type_of_submission.to_string())?;

    // sendingApplication
    start(w, "sendingApplication")?;
    elem(w, "manufacturer", "Maravilla Labs")?;
    elem(w, "product", "Maravilla Konto")?;
    elem(w, "productVersion", "1.0")?;
    end(w, "sendingApplication")?;

    // Period
    start(w, "period")?;
    elem(w, "periodFrom", &input.from_date.format("%Y-%m-%d").to_string())?;
    elem(w, "periodTo", &input.to_date.format("%Y-%m-%d").to_string())?;
    end(w, "period")?;

    elem(w, "formOfReporting", &input.form_of_reporting.to_string())?;

    let method = if is_flat_rate { "flatTaxRate" } else { "effective" };
    elem(w, "typeOfMethod", method)?;

    end(w, "generalInformation")
}

fn write_turnover_computation(
    w: &mut XmlWriter,
    input: &VatDeclarationInput,
    total_deduction: Decimal,
    taxable_turnover: Decimal,
) -> Result<(), AppError> {
    start(w, "turnoverComputation")?;

    dec(w, "totalConsideration", input.total_consideration)?;
    dec(w, "suppliesToForeignCountries", input.supplies_to_foreign)?;
    dec(w, "suppliesAbroad", input.supplies_abroad)?;
    dec(w, "transferNotificationProcedure", input.transfer_notification)?;
    dec(w, "suppliesExemptFromTax", input.supplies_exempt)?;
    dec(w, "reductionOfConsideration", input.reduction_of_consideration)?;
    dec(w, "variousDeduction", input.various_deduction)?;
    dec(w, "totalDeduction", total_deduction)?;
    dec(w, "taxableTurnover", taxable_turnover)?;

    end(w, "turnoverComputation")
}

fn write_supplies_per_tax_rate(
    w: &mut XmlWriter,
    entries: &[VatReportEntry],
) -> Result<(), AppError> {
    for entry in entries {
        start(w, "suppliesPerTaxRate")?;
        dec(w, "taxRate", entry.rate)?;
        dec(w, "turnover", entry.taxable_amount)?;
        dec(w, "tax", entry.vat_amount)?;
        end(w, "suppliesPerTaxRate")?;
    }
    Ok(())
}

fn write_flat_tax_rate_method(
    w: &mut XmlWriter,
    output_entries: &[VatReportEntry],
    flat_rate_pct: Option<Decimal>,
) -> Result<(), AppError> {
    start(w, "flatTaxRateMethod")?;

    if let Some(pct) = flat_rate_pct {
        dec(w, "flatTaxRate", pct)?;
    }

    write_supplies_per_tax_rate(w, output_entries)?;

    let total_tax: Decimal = output_entries.iter().map(|e| e.vat_amount).sum();
    dec(w, "totalTax", total_tax)?;

    end(w, "flatTaxRateMethod")
}

fn write_effective_method(
    w: &mut XmlWriter,
    output_entries: &[VatReportEntry],
    input_entries: &[VatReportEntry],
    total_input_vat: Decimal,
) -> Result<(), AppError> {
    start(w, "effectiveReportingMethod")?;
    elem(w, "grossOrNet", "gross")?;

    write_supplies_per_tax_rate(w, output_entries)?;

    let total_output_tax: Decimal = output_entries.iter().map(|e| e.vat_amount).sum();
    dec(w, "totalTax", total_output_tax)?;

    // Input tax deduction
    start(w, "inputTaxDeduction")?;
    for entry in input_entries {
        start(w, "inputTaxPerTaxRate")?;
        dec(w, "taxRate", entry.rate)?;
        dec(w, "amount", entry.vat_amount)?;
        end(w, "inputTaxPerTaxRate")?;
    }
    dec(w, "totalInputTaxDeduction", total_input_vat)?;
    end(w, "inputTaxDeduction")?;

    end(w, "effectiveReportingMethod")
}

fn write_payable_tax(w: &mut XmlWriter, net_vat_owed: Decimal) -> Result<(), AppError> {
    start(w, "payableTax")?;
    if net_vat_owed >= Decimal::ZERO {
        dec(w, "totalTaxDue", net_vat_owed)?;
    } else {
        dec(w, "totalTaxCredit", net_vat_owed.abs())?;
    }
    end(w, "payableTax")
}

fn write_other_flows(
    w: &mut XmlWriter,
    subsidies: Decimal,
    donations: Decimal,
) -> Result<(), AppError> {
    start(w, "otherFlowsOfFunds")?;
    if subsidies != Decimal::ZERO {
        dec(w, "subsidies", subsidies)?;
    }
    if donations != Decimal::ZERO {
        dec(w, "donations", donations)?;
    }
    end(w, "otherFlowsOfFunds")
}
