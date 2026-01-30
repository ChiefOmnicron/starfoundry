CREATE TABLE IF NOT EXISTS item (
    type_id        INTEGER NOT NULL,
    category_id    INTEGER NOT NULL,
    group_id       INTEGER NOT NULL,

    meta_group_id  INTEGER,

    volume         REAL    NOT NULL,

    name           VARCHAR NOT NULL,

    repackaged     INTEGER,

    PRIMARY KEY(type_id)
);

INSERT INTO public.item VALUES (40, 4, 18, NULL, 0.01, 'Megacyte', NULL);
INSERT INTO public.item VALUES (2865, 7, 55, 2, 20, '1200mm Artillery Cannon II', NULL);
INSERT INTO public.item VALUES (16274, 4, 423, NULL, 0.03, 'Helium Isotopes', NULL);
INSERT INTO public.item VALUES (17736, 6, 27, 4, 470000, 'Nightmare', 50000);
INSERT INTO public.item VALUES (19720, 6, 485, 1, 1.85e+07, 'Revelation', 1300000);
INSERT INTO public.item VALUES (21023, 17, 873, NULL, 2000, 'Capital Shield Emitter', NULL);
INSERT INTO public.item VALUES (29986, 6, 963, 14, 118000, 'Legion', 5000);
INSERT INTO public.item VALUES (30376, 2, 711, NULL, 5, 'Fullerite-C32', NULL);
INSERT INTO public.item VALUES (37300, 7, 74, 2, 4000, 'Triple Neutron Blaster Cannon II', 1000);
INSERT INTO public.item VALUES (40351, 7, 329, 2, 4000, '25000mm Steel Plates II', 1000);
INSERT INTO public.item VALUES (41322, 8, 377, 2, 0.125, 'Void XL', NULL);
INSERT INTO public.item VALUES (41489, 8, 87, 1, 128, 'Cap Booster 3200', NULL);
INSERT INTO public.item VALUES (45511, 25, 1923, NULL, 10, 'Monazite', NULL);
INSERT INTO public.item VALUES (46315, 25, 1923, NULL, 10, 'Shining Monazite', NULL);
INSERT INTO public.item VALUES (73787, 6, 485, 4, 1.55e+07, 'Naglfar Fleet Issue', 1300000);
INSERT INTO public.item VALUES (73790, 6, 485, 4, 1.85e+07, 'Revelation Navy Issue', 1300000);
INSERT INTO public.item VALUES (73792, 6, 485, 4, 1.755e+07, 'Moros Navy Issue', 1300000);
INSERT INTO public.item VALUES (73793, 6, 485, 4, 1.625e+07, 'Phoenix Navy Issue', 1300000);
INSERT INTO public.item VALUES (77284, 6, 4594, 2, 1.625e+07, 'Karura', 1300000);
