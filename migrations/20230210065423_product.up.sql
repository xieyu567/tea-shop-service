CREATE TABLE product.products
(
    product_id   uuid        NOT NULL DEFAULT gen_random_uuid(),
    product_name varchar(64) NOT NULL,
    unit_price   money       NOT NULL,
    create_at    timestamp   NOT NULL,
    update_at    timestamp   NOT NULL,

    CONSTRAINT products_pkey PRIMARY KEY (product_id)
);