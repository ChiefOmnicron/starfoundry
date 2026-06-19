# EVE API Wrapper

Handles the auth side and wrapping the EVE-API for all services within the StarFoundry eco system.

## Configuration example

### Config file

The file supports one or more domains to be configured.

``` toml
# config.toml

[domains."my.domain.space"]
# list of character ids, that act as admins within the application
admins = [
    2117441999,
]
# whitelist of either an alliance_id, corporation_id or character_id
# leave empty to allow all
whitelist = [
    98024275, # RCI
]
# list of all scopes that should be requested on character login
character_scopes = [
    "publicData",
    "esi-assets.read_assets.v1",
    "esi-characters.read_blueprints.v1",
    "esi-contracts.read_character_contracts.v1",
    "esi-fittings.write_fittings.v1",
    "esi-industry.read_character_jobs.v1",
    "esi-markets.read_character_orders.v1",
    "esi-markets.structure_markets.v1",
    "esi-search.search_structures.v1",
    "esi-skills.read_skills.v1",
    "esi-universe.read_structures.v1",
]
# list of additional corporation scopes
corporation_scopes = [
    "publicData",
    "esi-assets.read_corporation_assets.v1",
    "esi-contracts.read_corporation_contracts.v1",
    "esi-corporations.read_blueprints.v1",
    "esi-industry.read_corporation_jobs.v1",
    "esi-markets.read_corporation_orders.v1",
]
# redirect after successful login
# not using the same domain as above will cause problems
# the /api/auth/callback should be configured to end at the gateway service, as
# it will handle the finalization of the login process
redirect = "https://my.domain.space/api/auth/login/callback"
```

### Environment

``` env
# logging level, recommended configuration for prod
RUST_LOG=warn,sqlx::query=error

# URL of the database, format: `postgres://{username}:{password}@{host}:{port}/{database}`
STARFOUNDRY_EVE_GATEWAY_DATABASE_URL=postgresql://postgres:postgres@localhost:5432/dev-sf-eve-gateway
# Address the actual application should listen on
STARFOUNDRY_EVE_GATEWAY_APP_ADDRESS=0.0.0.0:9998
# Address the service application for health checks and metrics should listen on
STARFOUNDRY_EVE_GATEWAY_SERVICE_ADDRESS=0.0.0.0:9999

# User-Agent that is used to communicate with the EVE-API
# For example your in-game name, or another identifier that the EVE-Devs can
# reach you
STARFOUNDRY_USER_AGENT="{SOME_IDENTIFIER}"

# EVE-API Client API, can be obtained under https://developers.eveonline.com/applications
STARFOUNDRY_EVE_GATEWAY_EVE_CLIENT_ID=
STARFOUNDRY_EVE_GATEWAY_EVE_SECRET_KEY=

# ECDSA private key for JWT
# Generate one using `openssl ecparam -name secp256k1 -genkey -noout -out ec-secp256k1-priv-key.pem`
STARFOUNDRY_EVE_GATEWAY_JWT_ECDSA_PRIVATE="-----BEGIN PRIVATE KEY-----
MIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQgBENhBhOH/QMyqwMG
mfit6RoxZ4ypfjFFGD3Eb+6mVIuhRANCAASReSoyen7jUiXS26WxeFbvigQmlN2M
d+BUs+4m7ShR0z2EYVXWVaDRorZY2J+PEnc4LrCb3rY8P3zoZikbr8ut
-----END PRIVATE KEY-----"
# Public key part of the above private key `openssl ec -in ec-secp256k1-priv-key.pem -pubout > ec-secp256k1-pub-key.pem`
STARFOUNDRY_EVE_GATEWAY_JWT_ECDSA_PUBLIC="-----BEGIN PUBLIC KEY-----
MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEkXkqMnp+41Il0tulsXhW74oEJpTd
jHfgVLPuJu0oUdM9hGFV1lWg0aK2WNifjxJ3OC6wm962PD986GYpG6/LrQ==
-----END PUBLIC KEY-----"
# Issuer of the JWT token TODO:
STARFOUNDRY_EVE_GATEWAY_JWT_ISSUER_DOMAIN=http://localhost
```
