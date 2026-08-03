use toasty::{Deferred, Embed, Model};

#[derive(Debug, Model)]
pub struct Product {
    #[key]
    #[auto]
    pub id: u64,

    #[unique]
    pub stripe_id: String,

    pub name: String,
    pub stock: u32,
}

#[derive(Debug, Model)]
pub struct ShippingMethod {
    #[key]
    #[auto]
    pub id: u64,

    #[unique]
    pub stripe_id: String,

    pub name: String,
}

#[derive(Debug, Model)]
pub struct Customer {
    #[key]
    #[auto]
    pub id: u64,
    #[auto]
    pub created_at: jiff::Timestamp,
    #[auto]
    pub updated_at: jiff::Timestamp,

    pub name: String,
    #[unique]
    pub email: String,

    #[has_many]
    pub orders: Deferred<Vec<Order>>,
}

#[derive(Debug, Model)]
pub struct OrderProduct {
    #[key]
    #[auto]
    pub id: u64,

    #[index]
    pub order_id: u64,
    #[belongs_to]
    pub order: Deferred<Order>,
    #[index]
    pub product_id: u64,
    #[belongs_to]
    pub product: Deferred<Product>,

    #[default(1)]
    pub quantity: u32,
}

#[derive(Debug, Embed)]
#[column(type = varchar(255))]
pub enum OrderStatus {
    Ordered,
    Shipped,
    Received,
}

#[derive(Debug, Embed)]
#[column(type = varchar(255))]
pub enum OrderOrigin {
    Amazon,
    Leboncoin,
    Stripe,
    Vinted,
}

#[derive(Debug, toasty::Embed)]
pub struct ShippingDetails {
    pub name: String,
    pub city: Option<String>,
    pub country: Option<String>,
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub postal_code: Option<String>,
    pub state: Option<String>,
}

#[derive(Debug, Model)]
pub struct Order {
    #[key]
    #[auto]
    pub id: u64,
    #[auto]
    pub created_at: jiff::Timestamp,
    #[auto]
    pub updated_at: jiff::Timestamp,

    pub amount: f64,
    pub status: OrderStatus,
    pub origin: OrderOrigin,
    pub tracking_id: Option<String>,

    pub shipping: Option<ShippingDetails>,

    #[index]
    pub customer_id: u64,
    #[belongs_to]
    pub customer: Deferred<Customer>,

    #[index]
    pub shipping_method_id: u64,
    #[belongs_to]
    pub shipping_method: Deferred<Option<ShippingMethod>>,

    #[has_many]
    pub order_products: Deferred<Vec<OrderProduct>>,
    #[has_many(via = order_products.product)]
    pub products: Deferred<Vec<Product>>,
}
