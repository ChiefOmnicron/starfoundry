# Auth

This service provides a general authentication layer for all services within the StarFoundry environment.

Users are logged in via OAuth using EVE.

## Processes

After a user logs in successfully the EVE access and refresh token are stored in the database.
The application generates a refresh token that is returned using a cookie `refresh_token`.
The requester then has to make a request to get a new access token using the refresh token.

The `refresh_token` is valid for 24h.
With requesting a new `access_token` a new `refresh_token` is generated and set.
The `access_token` is only valid for 15 minutes, and needs to be refreshed using the `refresh_token`.

# API

The API allows for logging in new users, add additional characters or corporations, and generate API keys for external applications.

**Base URL**

`https://api.auth.starfoundry.space`

## API Versioning

The API is versioned by an path param.

Example:
- `https://api.auth.starfoundry.space/v{version}`
- `https://api.auth.starfoundry.space/latest`
- `https://api.auth.starfoundry.space`

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
