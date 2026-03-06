use rust_decimal::Decimal;

use super::report_types::TrialBalanceRow;

/// Defines a Swiss KMU account group by number range.
pub struct AccountRange {
    pub label: &'static str,
    pub total_label: &'static str,
    pub from: i32,
    pub to: i32,
}

/// Grouped accounts with subtotal.
#[derive(Debug, Clone, serde::Serialize)]
pub struct AccountGroupResult {
    pub label: String,
    pub total_label: String,
    pub accounts: Vec<TrialBalanceRow>,
    pub subtotal: Decimal,
}

/// A top-level section (e.g., Umlaufvermögen, Anlagevermögen).
#[derive(Debug, Clone, serde::Serialize)]
pub struct GroupedSection {
    pub key: String,
    pub label: String,
    pub groups: Vec<AccountGroupResult>,
    pub total: Decimal,
}

// --- Bilanz Aktiven ---

pub fn ch_balance_sheet_assets() -> Vec<(&'static str, &'static str, Vec<AccountRange>)> {
    vec![
        (
            "current_assets",
            "Umlaufvermögen",
            vec![
                AccountRange { label: "Flüssige Mittel", total_label: "Total Flüssige Mittel", from: 1000, to: 1099 },
                AccountRange { label: "Forderungen aus Lieferungen und Leistungen", total_label: "Total Forderungen L&L", from: 1100, to: 1199 },
                AccountRange { label: "Andere kurzfristige Forderungen", total_label: "Total Andere kfr. Ford.", from: 1200, to: 1299 },
                AccountRange { label: "Vorräte und nicht fakturierte Dienstleistungen", total_label: "Total Vorräte", from: 1200, to: 1299 },
                AccountRange { label: "Aktive Rechnungsabgrenzungen", total_label: "Total Aktive RA", from: 1300, to: 1399 },
            ],
        ),
        (
            "fixed_assets",
            "Anlagevermögen",
            vec![
                AccountRange { label: "Finanzanlagen", total_label: "Total Finanzanlagen", from: 1400, to: 1499 },
                AccountRange { label: "Mobile Sachanlagen", total_label: "Total Mobile Sachanlagen", from: 1500, to: 1599 },
                AccountRange { label: "Immobile Sachanlagen", total_label: "Total Immobile Sachanlagen", from: 1600, to: 1699 },
                AccountRange { label: "Immaterielle Anlagen", total_label: "Total Immaterielle Anlagen", from: 1700, to: 1799 },
            ],
        ),
    ]
}

// --- Bilanz Passiven ---

pub fn ch_balance_sheet_liabilities() -> Vec<(&'static str, &'static str, Vec<AccountRange>)> {
    vec![
        (
            "current_liabilities",
            "Kurzfristiges Fremdkapital",
            vec![
                AccountRange { label: "Verbindlichkeiten aus Lieferungen und Leistungen", total_label: "Total Kreditoren", from: 2000, to: 2099 },
                AccountRange { label: "Kurzfristige Finanzverbindlichkeiten", total_label: "Total Kfr. Finanzverb.", from: 2100, to: 2199 },
                AccountRange { label: "Übrige kurzfristige Verbindlichkeiten", total_label: "Total Übrige kfr. Verb.", from: 2200, to: 2299 },
                AccountRange { label: "Passive Rechnungsabgrenzungen und kurzfristige Rückstellungen", total_label: "Total Passive RA", from: 2300, to: 2399 },
            ],
        ),
        (
            "long_term_liabilities",
            "Langfristiges Fremdkapital",
            vec![
                AccountRange { label: "Langfristige Finanzverbindlichkeiten", total_label: "Total Lfr. Finanzverb.", from: 2400, to: 2499 },
                AccountRange { label: "Übrige langfristige Verbindlichkeiten", total_label: "Total Übrige lfr. Verb.", from: 2500, to: 2599 },
                AccountRange { label: "Rückstellungen und Wertberichtigungen", total_label: "Total Rückstellungen", from: 2600, to: 2799 },
            ],
        ),
        (
            "equity",
            "Eigenkapital",
            vec![
                AccountRange { label: "Grund-/Stammkapital", total_label: "Total Stammkapital", from: 2800, to: 2899 },
                AccountRange { label: "Reserven", total_label: "Total Reserven", from: 2900, to: 2969 },
                AccountRange { label: "Gewinn-/Verlustvortrag", total_label: "Total Gewinnvortrag", from: 2970, to: 2978 },
                AccountRange { label: "Jahresergebnis", total_label: "Jahresergebnis", from: 2979, to: 2979 },
            ],
        ),
    ]
}

// --- Erfolgsrechnung ---

pub fn ch_income_statement_sections() -> Vec<(&'static str, &'static str, Vec<AccountRange>)> {
    vec![
        (
            "operating_revenue",
            "Betriebsertrag aus Lieferungen und Leistungen",
            vec![
                AccountRange { label: "Produktionserlöse", total_label: "Total Produktionserlöse", from: 3000, to: 3199 },
                AccountRange { label: "Dienstleistungsertrag", total_label: "Total Dienstleistungsertrag", from: 3200, to: 3499 },
                AccountRange { label: "Handelserlöse", total_label: "Total Handelserlöse", from: 3500, to: 3699 },
                AccountRange { label: "Übrige Erlöse", total_label: "Total Übrige Erlöse", from: 3700, to: 3899 },
                AccountRange { label: "Erlösminderungen", total_label: "Total Erlösminderungen", from: 3900, to: 3999 },
            ],
        ),
        (
            "material_expense",
            "Aufwand für Material, Handelsware und Drittleistungen",
            vec![
                AccountRange { label: "Materialaufwand", total_label: "Total Materialaufwand", from: 4000, to: 4499 },
                AccountRange { label: "Drittleistungen", total_label: "Total Drittleistungen", from: 4500, to: 4999 },
            ],
        ),
        (
            "personnel_expense",
            "Personalaufwand",
            vec![
                AccountRange { label: "Löhne und Gehälter", total_label: "Total Löhne", from: 5000, to: 5499 },
                AccountRange { label: "Sozialversicherungsaufwand", total_label: "Total Sozialvers.", from: 5500, to: 5799 },
                AccountRange { label: "Übriger Personalaufwand", total_label: "Total Übriger Personalaufw.", from: 5800, to: 5999 },
            ],
        ),
        (
            "other_opex",
            "Übriger betrieblicher Aufwand",
            vec![
                AccountRange { label: "Raumaufwand", total_label: "Total Raumaufwand", from: 6000, to: 6099 },
                AccountRange { label: "Unterhalt und Reparaturen", total_label: "Total Unterhalt", from: 6100, to: 6199 },
                AccountRange { label: "Fahrzeugaufwand", total_label: "Total Fahrzeugaufwand", from: 6200, to: 6299 },
                AccountRange { label: "Sachversicherungen", total_label: "Total Sachversicherungen", from: 6300, to: 6399 },
                AccountRange { label: "Verwaltungsaufwand", total_label: "Total Verwaltungsaufwand", from: 6500, to: 6599 },
                AccountRange { label: "Informatikaufwand", total_label: "Total Informatikaufwand", from: 6600, to: 6699 },
                AccountRange { label: "Werbeaufwand", total_label: "Total Werbeaufwand", from: 6700, to: 6799 },
            ],
        ),
        (
            "depreciation",
            "Abschreibungen und Wertberichtigungen",
            vec![
                AccountRange { label: "Abschreibungen", total_label: "Total Abschreibungen", from: 6800, to: 6899 },
            ],
        ),
        (
            "financial_result",
            "Finanzergebnis",
            vec![
                AccountRange { label: "Finanzaufwand", total_label: "Total Finanzaufwand", from: 6900, to: 6949 },
                AccountRange { label: "Finanzertrag", total_label: "Total Finanzertrag", from: 6950, to: 6999 },
            ],
        ),
        (
            "extraordinary",
            "Betriebsfremder, ausserordentlicher Erfolg",
            vec![
                AccountRange { label: "Betriebsfremder Aufwand", total_label: "Total Betriebsfr. Aufw.", from: 8000, to: 8099 },
                AccountRange { label: "Betriebsfremder Ertrag", total_label: "Total Betriebsfr. Ertr.", from: 8100, to: 8499 },
                AccountRange { label: "Ausserordentlicher Aufwand", total_label: "Total A.o. Aufwand", from: 8500, to: 8599 },
                AccountRange { label: "Ausserordentlicher Ertrag", total_label: "Total A.o. Ertrag", from: 8600, to: 8799 },
            ],
        ),
        (
            "taxes",
            "Steuern",
            vec![
                AccountRange { label: "Direkte Steuern", total_label: "Total Steuern", from: 8900, to: 8999 },
            ],
        ),
    ]
}

/// Groups trial balance rows by Swiss KMU account ranges.
pub fn group_accounts_by_range(
    rows: &[TrialBalanceRow],
    ranges: &[AccountRange],
    negate: bool,
) -> Vec<AccountGroupResult> {
    ranges
        .iter()
        .map(|range| {
            let accounts: Vec<TrialBalanceRow> = rows
                .iter()
                .filter(|r| r.account_number >= range.from && r.account_number <= range.to)
                .cloned()
                .collect();

            let subtotal: Decimal = if negate {
                accounts.iter().map(|a| -a.balance).sum()
            } else {
                accounts.iter().map(|a| a.balance).sum()
            };

            AccountGroupResult {
                label: range.label.to_string(),
                total_label: range.total_label.to_string(),
                accounts,
                subtotal,
            }
        })
        .filter(|g| !g.accounts.is_empty() || !g.subtotal.is_zero())
        .collect()
}

/// Builds grouped sections from section definitions and trial balance data.
pub fn build_grouped_sections(
    definitions: &[(&str, &str, Vec<AccountRange>)],
    rows: &[TrialBalanceRow],
    negate: bool,
) -> Vec<GroupedSection> {
    definitions
        .iter()
        .map(|(key, label, ranges)| {
            let groups = group_accounts_by_range(rows, ranges, negate);
            let total: Decimal = groups.iter().map(|g| g.subtotal).sum();
            GroupedSection {
                key: key.to_string(),
                label: label.to_string(),
                groups,
                total,
            }
        })
        .collect()
}
