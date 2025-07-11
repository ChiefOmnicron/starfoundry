/// Generator for StarFoundry-Uuids
///
/// Automatically derives a bunch of useful traits.
///
/// As an addition it implements [std::ops::Deref], [std::convert::From],
/// [std::convert::Into].
///
/// The generated new type struct is marked as an serde::transparent struct.
///
#[macro_export]
macro_rules! starfoundry_uuid {
    ($name:ident, $name_str:tt) => {
        /// Uuid wrapper for tool specific ids. Mostly for type safety and
        /// code clarity
        #[derive(
            Clone, Copy, Debug, Hash,
            PartialEq, Eq, PartialOrd, Ord,
        )]
        #[derive(sqlx::Type, serde::Deserialize, serde::Serialize)]
        #[sqlx(transparent)]
        #[serde(transparent)]
        #[derive(utoipa::ToSchema, utoipa::IntoParams)]
        #[into_params(names($name_str))]
        #[schema(
            example = json!(uuid::Uuid::new_v4()),
            value_type = Uuid,
        )]
        pub struct $name(uuid::Uuid);

        impl $name {
            pub fn new(uuid: uuid::Uuid) -> Self {
                Self(uuid)
            }
        }

        impl From<uuid::Uuid> for $name {
            fn from(value: uuid::Uuid) -> Self {
                Self(value)
            }
        }

        impl std::str::FromStr for $name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                s.parse::<uuid::Uuid>()
                    .map_err(|e| format!("Error parsing {e} into number {s}"))
                    .map(|p| Self(p))
            }
        }

        impl std::ops::Deref for $name {
            type Target = uuid::Uuid;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}
