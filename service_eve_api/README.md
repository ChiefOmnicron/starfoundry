# EVE API Wrapper

Handles the auth side and wrapping the EVE-API for all services within the StarFoundry eco system.

## Vision
- Login from all StarFoundry Services
  - Appraisal
  - Store
- One "management" interface -> industry tool
- Store and Appraisal are separate
- Single credential for all

## TODO
- API Keys
- Config file
  - Domain
  - Admins
  - Whitelist
  - Scopes
- Contain redirect link
- Validate that a Domain is actually allowed to login
- Elevate Permissions -> Default is Public Data only
- Logout function
- Rotating Refresh Token -> after every access token
- Cookie `Domain=starfoundry.space` -> configurable
- Expose JWT Signature
- Remove refresh token
- Properly validate EVE token
- Properly validate our own JWT token

## Configuration

The services uses a mix of environment variables and a config file.

### Environment

#### APP_ADDRESS
**Parameter**: `STARFOUNDRY_EVE_APP_ADDRESS`

**Default**: `0.0.0.0:8080`

**Description**: Sets the address the app server should listen to

#### SERVICE_ADDRESS
**Parameter**: `STARFOUNDRY_EVE_SERVICE_ADDRESS`

**Default**: `0.0.0.0:8081`

**Description**: Sets the address the healthcheck and metrics listen to

# TODO:
# EveClient ENVs

## STARFOUNDRY_EVE_CLIENT_API_URL

Default: https://esi.evetech.net

## STARFOUNDRY_EVE_CLIENT_OAUTH_AUTHORIZATION_URL

Default: https://login.eveonline.com/v2/oauth/authorize

## STARFOUNDRY_EVE_CLIENT_OAUTH_JWT_KEYS_URL

Default: https://login.eveonline.com/oauth/jwks

## STARFOUNDRY_EVE_CLIENT_OAUTH_TOKEN_URL

Default: https://login.eveonline.com/v2/oauth/token

## STARFOUNDRY_EVE_USER_AGENT

Default: StarFoundry
