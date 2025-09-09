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
- Create RSA key per cli

# Generate RSA Key

- `openssl ecparam -genkey -noout -name prime256v1 | openssl pkcs8 -topk8 -nocrypt -out private.pem`
- `openssl ec -pubout -in private.pem -out public.pem`

# Future ideas

- Refresh token rotation
