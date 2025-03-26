use rand::distr::Alphanumeric;
use rand::Rng;

pub fn generate_code() -> String {
    let mut rng = rand::rng();
    (&mut rng).sample_iter(Alphanumeric)
        .take(10)
        .map(char::from)
        .collect::<String>()
}
