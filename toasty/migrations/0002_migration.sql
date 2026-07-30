CREATE TYPE "order_origin" AS ENUM ('stripe', 'leboncoin', 'vinted');
CREATE TYPE "order_status" AS ENUM ('ordered', 'shipped', 'received');
ALTER TABLE "orders" ALTER COLUMN "status" TYPE order_status;
ALTER TABLE "orders" ADD COLUMN "origin" order_origin NOT NULL;
ALTER TABLE "orders" ADD COLUMN "shipping" BOOLEAN;
