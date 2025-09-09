INSERT INTO structure (
    structure_id,
    system_id,
    type_id,
    security,
    name,
    owner
)
VALUES (1, 30000142, 0,  'HIGHSEC', 'test', 1);

INSERT INTO market_order_latest
(
    structure_id,
    order_id,
    is_buy,
    type_id,
    remaining,
    price,
    expires,
    region_id
)
VALUES (1, 1, false, 34, 500, 1, NOW() + '3 days', 1);
