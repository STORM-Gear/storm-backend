use std::str::FromStr;

use stripe_checkout::PaymentPagesCheckoutSessionCheckoutAddressDetails;

use super::errors::PaymentInfoParsingError;

#[derive(Debug, Clone)]
pub struct ShippingDetails {
    pub name: String,
    pub city: Option<String>,
    pub country: Option<String>,
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub postal_code: Option<String>,
    pub state: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ShippingMethod {
    InPerson,
    FranceStandard,
    FranceTracking,
    FranceExpressTracking,
    International,
    InternationalTracking,
}

impl TryFrom<PaymentPagesCheckoutSessionCheckoutAddressDetails> for ShippingDetails {
    type Error = PaymentInfoParsingError;

    fn try_from(
        value: PaymentPagesCheckoutSessionCheckoutAddressDetails,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            city: value.address.city,
            country: value.address.country,
            line1: value.address.line1,
            line2: value.address.line2,
            postal_code: value.address.postal_code,
            state: value.address.state,
        })
    }
}

impl ShippingMethod {
    pub fn stripe_id(&self) -> &str {
        match self {
            Self::InPerson => "shr_1Tiu9nPB7bMAkkZ4zSCGHOUr",
            Self::FranceStandard => "shr_1TiyHqPB7bMAkkZ4ndxsCgTc",
            Self::FranceTracking => "shr_1TiyIfPB7bMAkkZ4CTkSVxKw",
            Self::FranceExpressTracking => "shr_1TiyJFPB7bMAkkZ4XEZdfomw",
            Self::International => "shr_1TiyJpPB7bMAkkZ4LdAkJKwu",
            Self::InternationalTracking => "shr_1TiyKOPB7bMAkkZ4k81e2V4f",
        }
    }
}

impl FromStr for ShippingMethod {
    type Err = PaymentInfoParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "shr_1Tiu9nPB7bMAkkZ4zSCGHOUr" => Ok(Self::InPerson),
            "shr_1TiyHqPB7bMAkkZ4ndxsCgTc" => Ok(Self::FranceStandard),
            "shr_1TiyIfPB7bMAkkZ4CTkSVxKw" | "shr_1UDJy1PB7bMAkkZ4nTNP2cTe" => {
                Ok(Self::FranceTracking)
            }
            "shr_1TiyJFPB7bMAkkZ4XEZdfomw" => Ok(Self::FranceExpressTracking),
            "shr_1TiyJpPB7bMAkkZ4LdAkJKwu" => Ok(Self::International),
            "shr_1TiyKOPB7bMAkkZ4k81e2V4f" => Ok(Self::InternationalTracking),
            _ => Err(PaymentInfoParsingError::UnknownShippingRate(s.to_string())),
        }
    }
}

impl ToString for ShippingMethod {
    fn to_string(&self) -> String {
        match self {
            ShippingMethod::InPerson => "Remise en main propre".into(),
            ShippingMethod::FranceStandard => "France standard".into(),
            ShippingMethod::FranceTracking => "France suivi".into(),
            ShippingMethod::FranceExpressTracking => "France express + suivi".into(),
            ShippingMethod::International => "Hors France".into(),
            ShippingMethod::InternationalTracking => "Hors France + suivi".into(),
        }
    }
}
