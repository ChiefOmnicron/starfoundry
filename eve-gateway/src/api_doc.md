# EVE-Gateway

This service acts as a gateway towards the [EVE-API](https://developers.eveonline.com).

The service is used within the StarFoundry Eco-System to provide login functionality
and wrap the EVE-API.

## Current functionality

- EVE-SSO Login for main characters, alt characters and corporations
- Authentication for all applications within the Eco-System

## JWT expiration

- The `refresh_token` is valid for 24h from the time it was created
- The `access_token` is valid for 15 minutes and must be refreshed using the `refresh_token`

## Processes

### Main character login

1. A GET request is made to `/auth/login`
2. Based on the HOST Header it is determined if the domain is allowed in the Eco-System
3. A redirect to the EVE-SSO Server is made, including required scopes
4. After a successful login, the user is redirect to `/auth/callback`
5. The route will store the `refresh_token` and use it in future operations
6. It will generate `refresh_token` and redirect back towards the given callback route in the config file
7. The requester service must set the `refresh_token` as a cookie and redirect towards the 
application
1. An access_token can be retrieved by calling `/auth/token`, the `refresh_token` must be set as a cookie in the request
2.  The access_token should be securely stored, for example storing it in memory and requesting a new one when needed
3.  Request towards the API can be made

### Validating the StarFoundry JWT-Token

- Make sure it's not expired `exp`
- The `iss` should be `https://api.eve-gateway.starfoundry.space` (if you run it locally, the domain will be different, you will need to set it as ENV)
- The `aud` will contain the host of the requesting service
- The signature can be validated using `ES256` with `P-256` curve
  - The current JWK is located under `/.well-known/jwks`
  - In the JWT and JKW the field `kid` should be `starfoundry-eve-gateway`

# API

The API allows for logging in new users, add additional characters or corporations, and generate API keys for external applications.

**Base URL**

`https://api.eve-gateway.starfoundry.space`

## API Versioning

The API is versioned by an path param.

Example:
- `https://api.eve-gateway.starfoundry.space/v{version}`
- `https://api.eve-gateway.starfoundry.space/latest`
- `https://api.eve-gateway.starfoundry.space`

All three shown above paths result in the same request.

If you omit adding a version or latest, the default version shown in the table below will be used.

| Version | Status | Default |
|---------|--------|---------|
| v1      | Active | Yes     |

## User-Agent

Please provide a [user agent](https://www.rfc-editor.org/rfc/rfc9110.html#section-10.1.5)

User Agent Example

`User-Agent: SomeBot ($url, $versionNumber)`

## Content Type

The API always answers with `Content-Type: application/json`, other content types will be rejected.

# Contact

- Discord: https://discord.gg/qShbyn4r9N
- GitHub: https://github.com/ChiefOmnicron/starfoundry
- Ingame: mail to 'Eistonen Kodan Sasen'
