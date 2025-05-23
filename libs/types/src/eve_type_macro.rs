/// Generator for EVE ID's.
///
/// Automatically derives a bunch of useful traits.
///
/// As an addition it implements [std::ops::Deref], [std::convert::From],
/// [std::convert::Into].
///
/// The generated new type struct is marked as an serde::transparent struct.
///
/// # Parameters
///
/// * `name` - Name of the ID
/// * `typ`  - Datatype of the ID (e.g. i32)
///
#[macro_export]
macro_rules! eve_id {
    ($name:ident, $typ:ty, $typ2:ty) => {
        /// Represents an ID-Type from EVE
        #[derive(
            Clone, Copy, Debug, Hash,
            PartialEq, Eq, PartialOrd, Ord,
        )]
        #[derive(sqlx::Type, serde::Deserialize, serde::Serialize)]
        #[sqlx(transparent)]
        #[serde(transparent)]
        #[derive(utoipa::ToSchema)]
        pub struct $name(pub $typ);

        impl From<$typ> for $name {
            fn from(x: $typ) -> Self {
                Self(x)
            }
        }

        impl From<&$typ> for $name {
            fn from(x: &$typ) -> Self {
                Self(*x)
            }
        }

        impl From<$typ2> for $name {
            fn from(x: $typ2) -> Self {
                Self(x as $typ)
            }
        }

        impl From<&$typ2> for $name {
            fn from(x: &$typ2) -> Self {
                Self(*x as $typ)
            }
        }

        impl std::ops::Deref for $name {
            type Target = $typ;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl std::str::FromStr for $name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                s.parse::<$typ>()
                    .map_err(|e| format!("Error parsing {e} into number {s}"))
                    .map(|p| Self(p))
            }
        }
    };
}
