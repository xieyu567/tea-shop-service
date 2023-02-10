CREATE TYPE "order".order_status AS ENUM ('unknown', 'pending', 'paid', 'canceled');
CREATE TYPE "order".order_update_type AS ENUM ('unknown', 'create', 'update', 'delete');

CREATE TABLE "order".orders
(
    order_id    uuid                 NOT NULL DEFAULT gen_random_uuid(),
    user_id     uuid                 NOT NULL,
    cart_id     uuid                 NOT NULL,
    status      "order".order_status NOT NULL DEFAULT 'pending',
    total_price money                NOT NULL,
    address     varchar(255)         NOT NULL,
    create_at   timestamp            NOT NULL,
    update_at   timestamp            NOT NULL,
    note        text,

    CONSTRAINT orders_pkey PRIMARY KEY (order_id)
);
CREATE INDEX orders_user_id_idx ON "order".orders (user_id);

CREATE TABLE "order".carts
(
    id               uuid  NOT NULL DEFAULT gen_random_uuid(),
    cart_id          uuid  NOT NULL,
    product_id       uuid  NOT NULL,
    product_quantity int   NOT NULL,
    total_price      money NOT NULL,

    CONSTRAINT carts_pkey PRIMARY KEY (id)
);
CREATE INDEX carts_cart_id_idx ON "order".carts (cart_id);


