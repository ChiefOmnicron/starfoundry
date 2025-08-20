# Eve-Gateway-Lib

Library for communication with the Eve-Gateway service.

## Auth

- Routes and Extractors for [axum](https://github.com/tokio-rs/axum)
- JWT-Token validation

# Usage

## Auth

### AppState

The JWT decoding key needs to be in the state.

1. Fetch the current keys
```
TODO: add example
```

2. Add a new field to the AppState
``` rust
#[derive(Clone)]
pub struct AppState {
    ... some other state

    pub decoding_key: Arc<DecodingKey>,
}
```

3. Implement FromRef for the library state
``` rust
impl FromRef<AppState> for EveGatewayState {
    fn from_ref(input: &AppState) -> Self {
        EveGatewayState {
            decoding_key: input.decoding_key.clone(),
        }
    }
}
```

### Environment Variables

**STARFOUNDRY_EVE_GATEWAY_URL**

Description: URL to the Eve-Gateway service

Example: `https://api.eve-gateway.starfoundry.space`

**STARFOUNDRY_EVE_GATEWAY_JWK_URL**

Description: URL to the JWK endpoint

Example: `https://api.eve-gateway.starfoundry.space/.well-known/jwks`

## Test helpers

### JWT Tokens

For generating valid JWT-Tokens during testing, a helper module `test` is provided.
The struct `crate::test::JwtToken` can generate tokens on demand that are signed.
Because the EC public and private key are constants, this is not secure in any way, and should never be used outside of testing.
