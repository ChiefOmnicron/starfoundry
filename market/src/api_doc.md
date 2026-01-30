# API

The API provides an interface for external tools. It is free to use, I only ask that you don't abuse it, otherwise I will have to do counter messures.

**Base URL**

`https://api.market.starfoundry.space`

## API Versioning

The API is versioned by an path param.

Example:
- `https://api.market.starfoundry.space/v{version}`
- `https://api.market.starfoundry.space/latest`
- `https://api.market.starfoundry.space`

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

## Contact

- Discord: https://discord.gg/qShbyn4r9N
- GitHub: https://github.com/ChiefOmnicron/starfoundry
- Ingame: mail to 'Eistonen Kodan Sasen'
