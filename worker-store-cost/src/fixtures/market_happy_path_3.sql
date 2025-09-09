INSERT INTO structure (
    structure_id,
    system_id,
    type_id,
    security,
    name,
    owner
) VALUES
(1, 30000142, 0,  'HIGHSEC', 'market_1', 1);
INSERT INTO structure (
    structure_id,
    system_id,
    type_id,
    security,
    name,
    owner
) VALUES
(2, 30000142, 0,  'HIGHSEC', 'market_2', 1);

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
) VALUES
(1, 1, false, 34, 500, 2, NOW() + '3 days', 1);
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
) VALUES
(2, 2, false, 34, 200, 0.9, NOW() + '3 days', 1),
(2, 3, false, 34, 200, 1.0, NOW() + '3 days', 1);
