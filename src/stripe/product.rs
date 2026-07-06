use std::str::FromStr;

use super::errors::PaymentInfoParsingError;

#[derive(Debug, Clone)]
pub enum Product {
    Storm,
    ChargingCable,
    Velcro,
}

impl FromStr for Product {
    type Err = PaymentInfoParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "prod_Tx78EpjfgGBXBY" => Ok(Self::Storm),
            "prod_UeYnwJRcC0Bdjd" => Ok(Self::ChargingCable),
            "prod_UeYkOsu0lehYzY" => Ok(Self::Velcro),
            _ => Err(PaymentInfoParsingError::UnknownProduct(s.to_string())),
        }
    }
}

impl ToString for Product {
    fn to_string(&self) -> String {
        match self {
            Product::Storm => "Vario STORM".into(),
            Product::ChargingCable => "Câble de charge USB-C".into(),
            Product::Velcro => "Option velcro extra".into(),
        }
    }
}
