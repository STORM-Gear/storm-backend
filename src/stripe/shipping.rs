use std::str::FromStr;

use super::errors::PaymentInfoParsingError;

#[derive(Debug, Clone)]
pub enum ShippingMethod {
    InPerson,
    FranceStandard,
    FranceTracking,
    FranceExpressTracking,
    International,
    InternationalTracking,
}

impl FromStr for ShippingMethod {
    type Err = PaymentInfoParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "shr_1Tiu9nPB7bMAkkZ4zSCGHOUr" => Ok(Self::InPerson),
            "shr_1TiyHqPB7bMAkkZ4ndxsCgTc" => Ok(Self::FranceStandard),
            "shr_1TiyIfPB7bMAkkZ4CTkSVxKw" => Ok(Self::FranceTracking),
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
