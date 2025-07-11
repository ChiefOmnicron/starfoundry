use super::AuthError;

#[derive(Debug, PartialEq, Eq)]
pub enum Intention {
    Login,
    LoginAlt,
    LoginCorporation,
}

impl Intention {
    pub fn from_str(
        intention: String,
    ) -> Result<Self, AuthError> {
        match intention.as_ref() {
            "LOGIN"             => Ok(Self::Login),
            "LOGIN_ALT"         => Ok(Self::LoginAlt),
            "LOGIN_CORPORATION" => Ok(Self::LoginCorporation),
            _                   => Err(AuthError::InvalidIntention),
        }
    }

    pub fn to_string(
        self
    ) -> String {
        match self {
            Self::Login            => "LOGIN",
            Self::LoginAlt         => "LOGIN_ALT",
            Self::LoginCorporation => "LOGIN_CORPORATION",
        }.into()
    }
}
