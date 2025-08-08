# Auth

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

## Environment

### APP_ADDRESS
**Parameter**: `STARFOUNDRY_AUTH_APP_ADDRESS`

**Default**: `0.0.0.0:8080`

**Description**: Sets the address the app server should listen to

### SERVICE_ADDRESS
**Parameter**: `STARFOUNDRY_AUTH_SERVICE_ADDRESS`

**Default**: `0.0.0.0:8081`

**Description**: Sets the address the healthcheck and metrics listen to
