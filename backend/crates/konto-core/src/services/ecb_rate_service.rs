use std::collections::HashMap;

use chrono::NaiveDate;
use konto_common::error::AppError;
use konto_db::entities::{currency, exchange_rate};
use konto_db::repository::currency_repo::CurrencyRepo;
use konto_db::repository::exchange_rate_repo::ExchangeRateRepo;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use rust_decimal::Decimal;
use sea_orm::{DatabaseConnection, Set};
use uuid::Uuid;

const ECB_DAILY_FEED_URL: &str = "https://www.ecb.europa.eu/stats/eurofxref/eurofxref-daily.xml";

pub struct EcbRateService;

impl EcbRateService {
    /// Fetch the ECB's public daily reference-rate feed and upsert cross rates
    /// (source: "ECB") for every ordered pair of currencies already configured
    /// in Settings > Currencies. Rates stay purely informational/manual-equivalent —
    /// this only populates the same `exchange_rates` table a user could edit by hand.
    pub async fn fetch_latest(db: &DatabaseConnection) -> Result<usize, AppError> {
        let body = reqwest::get(ECB_DAILY_FEED_URL)
            .await
            .map_err(|e| AppError::Validation(format!("Failed to reach ECB feed: {e}")))?
            .text()
            .await
            .map_err(|e| AppError::Validation(format!("Failed to read ECB feed: {e}")))?;

        let (date, per_eur) = parse_ecb_feed(&body)?;

        let currencies = CurrencyRepo::find_all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let mut count = 0usize;
        for from in &currencies {
            let Some(&from_per_eur) = per_eur.get(from.code.as_str()) else { continue };
            for to in &currencies {
                if from.id == to.id {
                    continue;
                }
                let Some(&to_per_eur) = per_eur.get(to.code.as_str()) else { continue };

                let rate = to_per_eur / from_per_eur;
                upsert_rate(db, from, to, rate, date).await?;
                count += 1;
            }
        }

        Ok(count)
    }
}

async fn upsert_rate(
    db: &DatabaseConnection,
    from: &currency::Model,
    to: &currency::Model,
    rate: Decimal,
    date: NaiveDate,
) -> Result<(), AppError> {
    let existing = ExchangeRateRepo::find_latest(db, &from.id, &to.id, Some(date))
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    if let Some(existing) = existing {
        if existing.valid_date == date {
            let mut model: exchange_rate::ActiveModel = existing.into();
            model.rate = Set(rate);
            model.source = Set(Some("ECB".to_string()));
            ExchangeRateRepo::update(db, model)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;
            return Ok(());
        }
    }

    let now = chrono::Utc::now().naive_utc();
    let model = exchange_rate::ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        from_currency_id: Set(from.id.clone()),
        to_currency_id: Set(to.id.clone()),
        rate: Set(rate),
        valid_date: Set(date),
        source: Set(Some("ECB".to_string())),
        created_at: Set(now),
        updated_at: Set(now),
    };
    ExchangeRateRepo::create(db, model)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
    Ok(())
}

/// Parse the ECB `eurofxref-daily.xml` feed into (date, {currency_code: units per 1 EUR}),
/// with "EUR" itself mapped to 1.
fn parse_ecb_feed(xml: &str) -> Result<(NaiveDate, HashMap<String, Decimal>), AppError> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut date: Option<NaiveDate> = None;
    let mut rates = HashMap::new();
    rates.insert("EUR".to_string(), Decimal::ONE);

    loop {
        match reader.read_event() {
            Ok(Event::Empty(e)) | Ok(Event::Start(e)) if e.local_name().as_ref() == b"Cube" => {
                let mut time: Option<String> = None;
                let mut ccy: Option<String> = None;
                let mut rate: Option<Decimal> = None;
                for attr in e.attributes().flatten() {
                    let value = attr.unescape_value().unwrap_or_default().to_string();
                    match attr.key.as_ref() {
                        b"time" => time = Some(value),
                        b"currency" => ccy = Some(value),
                        b"rate" => rate = value.parse::<Decimal>().ok(),
                        _ => {}
                    }
                }
                if let Some(t) = time {
                    date = NaiveDate::parse_from_str(&t, "%Y-%m-%d").ok();
                }
                if let (Some(ccy), Some(rate)) = (ccy, rate) {
                    rates.insert(ccy, rate);
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(AppError::Validation(format!("Invalid ECB feed XML: {e}"))),
            _ => {}
        }
    }

    let date = date.ok_or_else(|| AppError::Validation("ECB feed missing rate date".to_string()))?;
    Ok((date, rates))
}

#[cfg(test)]
mod tests {
    use super::*;

    // A trimmed real sample of https://www.ecb.europa.eu/stats/eurofxref/eurofxref-daily.xml
    const SAMPLE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<gesmes:Envelope xmlns:gesmes="http://www.gesmes.org/xml/2002-08-01" xmlns="http://www.ecb.int/vocabulary/2002-08-01/eurofxref">
	<gesmes:subject>Reference rates</gesmes:subject>
	<gesmes:Sender>
		<gesmes:name>European Central Bank</gesmes:name>
	</gesmes:Sender>
	<Cube>
		<Cube time='2026-07-15'>
			<Cube currency='USD' rate='1.1406'/>
			<Cube currency='CHF' rate='0.9256'/>
			<Cube currency='GBP' rate='0.85093'/>
		</Cube>
	</Cube>
</gesmes:Envelope>"#;

    #[test]
    fn parses_date_and_rates() {
        let (date, rates) = parse_ecb_feed(SAMPLE).expect("should parse");
        assert_eq!(date, NaiveDate::from_ymd_opt(2026, 7, 15).unwrap());
        assert_eq!(rates.get("EUR"), Some(&Decimal::ONE));
        assert_eq!(rates.get("USD"), Some(&"1.1406".parse().unwrap()));
        assert_eq!(rates.get("CHF"), Some(&"0.9256".parse().unwrap()));
        assert_eq!(rates.get("GBP"), Some(&"0.85093".parse().unwrap()));
    }

    #[test]
    fn cross_rate_eur_to_chf_and_usd_to_chf() {
        let (_, rates) = parse_ecb_feed(SAMPLE).expect("should parse");
        let eur_per_eur = rates["EUR"];
        let chf_per_eur = rates["CHF"];
        let usd_per_eur = rates["USD"];

        // EUR -> CHF: units of CHF per 1 EUR
        let eur_to_chf = chf_per_eur / eur_per_eur;
        assert_eq!(eur_to_chf, "0.9256".parse().unwrap());

        // USD -> CHF: units of CHF per 1 USD
        let usd_to_chf = chf_per_eur / usd_per_eur;
        assert!((usd_to_chf - Decimal::new(8116, 4)).abs() < Decimal::new(1, 3));
    }
}
