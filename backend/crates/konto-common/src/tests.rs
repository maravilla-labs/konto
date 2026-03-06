#[cfg(test)]
#[allow(clippy::module_inception)]
#[allow(clippy::unwrap_used)]
mod tests {
    use crate::enums::AccountType;
    use crate::types::{AccountNumber, Money};
    use rust_decimal::Decimal;
    use std::str::FromStr;

    #[test]
    fn test_money_rounding() {
        let m = Money::new(Decimal::from_str("10.555").unwrap());
        assert_eq!(m.0, Decimal::from_str("10.56").unwrap());
    }

    #[test]
    fn test_money_zero() {
        let m = Money::zero();
        assert_eq!(m.0, Decimal::ZERO);
    }

    #[test]
    fn test_account_type_from_number() {
        assert_eq!(AccountType::from_account_number(1000), AccountType::Asset);
        assert_eq!(AccountType::from_account_number(1521), AccountType::Asset);
        assert_eq!(AccountType::from_account_number(2000), AccountType::Liability);
        assert_eq!(AccountType::from_account_number(2800), AccountType::Equity);
        assert_eq!(AccountType::from_account_number(3200), AccountType::Revenue);
        assert_eq!(AccountType::from_account_number(4400), AccountType::Expense);
        assert_eq!(AccountType::from_account_number(6940), AccountType::Expense);
        assert_eq!(AccountType::from_account_number(9100), AccountType::System);
    }

    #[test]
    fn test_account_number_display() {
        let n = AccountNumber::new(1020);
        assert_eq!(n.to_string(), "1020");
    }

    #[test]
    fn test_money_display() {
        let m = Money::new(Decimal::from_str("1234.5").unwrap());
        assert_eq!(m.to_string(), "1234.50");
    }
}
