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
}

#[derive(Debug, Embed)]
pub enum OrderStatus {
    #[column(variant = 1)]
    Ordered,
    #[column(variant = 2)]
    Packaged,
    #[column(variant = 3)]
    Shipped,
    #[column(variant = 4)]
    Received,
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
    pub tracking_id: Option<String>,

    #[index]
    pub customer_id: u64,
    #[belongs_to]
    pub customer: Customer,

    #[index]
    pub shipping_method_id: u64,
    #[belongs_to]
    pub shipping_method: Option<ShippingMethod>,

    #[has_many]
    pub order_products: Deferred<Vec<OrderProduct>>,
    #[has_many(via = order_products.product)]
    pub products: Vec<Product>,
}
