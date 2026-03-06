use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::openapi::schema::{ObjectBuilder, SchemaType, Type};
use utoipa::openapi::RefOr;
use utoipa::{PartialSchema, ToSchema};

/// Wrapper around Decimal for money amounts ensuring correct precision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Money(pub Decimal);

impl PartialSchema for Money {
    fn schema() -> RefOr<utoipa::openapi::Schema> {
        ObjectBuilder::new()
            .schema_type(SchemaType::Type(Type::String))
            .description(Some("Decimal money amount"))
            .build()
            .into()
    }
}

impl ToSchema for Money {}

impl Money {
    pub fn new(amount: Decimal) -> Self {
        Self(amount.round_dp(2))
    }

    pub fn zero() -> Self {
        Self(Decimal::ZERO)
    }

    pub fn inner(&self) -> Decimal {
        self.0
    }
}

impl From<Decimal> for Money {
    fn from(d: Decimal) -> Self {
        Self::new(d)
    }
}

impl std::fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}", self.0)
    }
}

/// Account number (e.g. 1020, 2000, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
#[serde(transparent)]
pub struct AccountNumber(pub i32);

impl AccountNumber {
    pub fn new(num: i32) -> Self {
        Self(num)
    }
}

impl std::fmt::Display for AccountNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
