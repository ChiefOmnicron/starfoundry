use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize)]
pub struct Bonus {
    pub before:     f32,
    pub after:      f32,
    pub percent:    f32,
    pub typ:        String,
    pub additional: String,
    pub reason:     String,
}
