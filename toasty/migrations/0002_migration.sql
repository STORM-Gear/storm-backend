CREATE TYPE "order_origin" AS ENUM ('stripe', 'leboncoin', 'vinted');
CREATE TYPE "order_status" AS ENUM ('ordered', 'shipped', 'received');

ALTER TABLE "orders" ALTER COLUMN "status" DROP DEFAULT;
ALTER TABLE "orders"
  ALTER COLUMN "status" TYPE order_status
  USING (
    CASE status
      WHEN 1 THEN 'ordered'
      WHEN 2 THEN 'shipped'   -- old "Packaged" merged into shipped
      WHEN 3 THEN 'shipped'
      WHEN 4 THEN 'received'
    END
  )::order_status;

ALTER TABLE "orders" ADD COLUMN "origin" order_origin NOT NULL DEFAULT 'stripe';
ALTER TABLE "orders" ADD COLUMN "shipping" BOOLEAN;
