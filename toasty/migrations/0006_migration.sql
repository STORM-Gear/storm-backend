ALTER TABLE "orders" ADD COLUMN "stripe_payment_id" TEXT;
CREATE UNIQUE INDEX "index_orders_by_stripe_payment_id" ON "orders" ("stripe_payment_id");
