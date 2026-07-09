use toasty::{Deferred, Model};

#[derive(Debug, Model)]
pub struct Product {
    #[key]
    #[auto]
    id: u64,

    #[unique]
    stripe_id: String,

    name: String,
    stock: u32,
}

#[derive(Debug, Model)]
pub struct Customer {
    #[key]
    #[auto]
    id: u64,
    #[auto]
    created_at: jiff::Timestamp,
    #[auto]
    updated_at: jiff::Timestamp,

    name: String,
    #[unique]
    email: String,

    #[has_many]
    orders: Deferred<Vec<Order>>,
}

#[derive(Debug, Model)]
pub struct ShippingMethod {
    #[key]
    #[auto]
    id: u64,

    name: String,
}

#[derive(Debug, Model)]
pub struct OrderProducts {
    #[key]
    #[auto]
    id: u64,

    #[index]
    order_id: u64,
    #[belongs_to]
    order: Deferred<Order>,
    #[index]
    product_id: u64,
    #[belongs_to]
    product: Deferred<Product>,
}

#[derive(Debug, Model)]
pub struct Order {
    #[key]
    #[auto]
    id: u64,
    #[auto]
    created_at: jiff::Timestamp,
    #[auto]
    updated_at: jiff::Timestamp,

    amount: f32,

    #[index]
    customer_id: u64,
    #[belongs_to]
    customer: Customer,

    #[index]
    shipping_method_id: u64,
    #[belongs_to]
    shipping_method: Option<ShippingMethod>,

    #[has_many]
    order_products: Deferred<Vec<OrderProducts>>,
    #[has_many(via = order_products.product)]
    products: Vec<Product>,
}
