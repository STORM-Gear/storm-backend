use toasty::Db;
use tracing::info;

use crate::{stripe::PaymentInfo, utils::get_env_var};

pub mod models;

pub struct DbService {
    db: Db,
}

#[derive(Debug)]
pub enum InsertPaymentError {
    StartTx,
    CommitTx,
    CreateOrder,
    AlreadyExists,
    QueryCustomer,
    CreateCustomer,
    CreateOrderProduct,
    ProductNotFound(String),
    ShippingMethodNotFound(String),
}

impl DbService {
    pub async fn connect() -> Self {
        let db_url = get_env_var("DB_URL");

        let db = toasty::Db::builder()
            .models(toasty::models!(crate::*))
            .connect(&db_url)
            .await
            .unwrap();

        Self { db }
    }

    pub fn pop_db(self) -> Db {
        self.db
    }

    pub async fn insert_payment(&mut self, payment: PaymentInfo) -> Result<(), InsertPaymentError> {
        let mut tx = self
            .db
            .transaction()
            .await
            .map_err(|_| InsertPaymentError::StartTx)?;

        let exists = models::Order::get_by_stripe_payment_id(&mut tx, &payment.payment_id)
            .await
            .is_ok();
        if exists {
            return Err(InsertPaymentError::AlreadyExists);
        }

        let customer = match models::Customer::get_by_email(&mut tx, &payment.customer_email).await
        {
            Ok(customer) => Ok(customer),
            Err(e) => {
                if e.is_record_not_found() {
                    info!("Inserting new customer in DB");

                    toasty::create!(models::Customer {
                        name: payment.customer_name,
                        email: payment.customer_email
                    })
                    .exec(&mut tx)
                    .await
                    .map_err(|_| InsertPaymentError::CreateCustomer)
                } else {
                    Err(InsertPaymentError::QueryCustomer)
                }
            }
        }?;

        let shipping_method_stripe_id = payment.shipping_method.stripe_id();
        let shipping_method =
            models::ShippingMethod::get_by_stripe_id(&mut tx, shipping_method_stripe_id)
                .await
                .map_err(|_| {
                    InsertPaymentError::ShippingMethodNotFound(
                        shipping_method_stripe_id.to_string(),
                    )
                })?;

        let order = toasty::create!(models::Order {
            stripe_payment_id: payment.payment_id,
            amount: payment.revenue,
            status: models::OrderStatus::Ordered,
            origin: models::OrderOrigin::Stripe,
            customer_id: customer.id,
            shipping_method_id: shipping_method.id,

            shipping: models::ShippingDetails {
                name: payment.shipping_details.name,
                city: payment.shipping_details.city,
                country: payment.shipping_details.country,
                line1: payment.shipping_details.line1,
                line2: payment.shipping_details.line2,
                postal_code: payment.shipping_details.postal_code,
                state: payment.shipping_details.state,
            },
        })
        .exec(&mut tx)
        .await
        .map_err(|_| InsertPaymentError::CreateOrder)?;

        for product in payment.products.iter() {
            let product = models::Product::get_by_stripe_id(&mut tx, product.stripe_id())
                .await
                .map_err(|_| {
                    InsertPaymentError::ProductNotFound(product.stripe_id().to_string())
                })?;

            toasty::create!(models::OrderProduct {
                order_id: order.id,
                product_id: product.id,
            })
            .exec(&mut tx)
            .await
            .map_err(|_| InsertPaymentError::CreateOrderProduct)?;
        }

        tx.commit().await.map_err(|_| InsertPaymentError::CommitTx)
    }
}
