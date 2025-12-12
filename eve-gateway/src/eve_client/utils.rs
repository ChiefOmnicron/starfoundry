use serde::de::{self, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;

// Wrapper struct to hold a Vec<T>
#[derive(Debug, PartialEq)]
struct SingleOrVec(Vec<String>);
 
// Helper method to unwrap the inner Vec<T>
impl SingleOrVec {
    fn into_inner(self) -> Vec<String> {
        self.0
    }
}

impl<'de> de::Deserialize<'de> for SingleOrVec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de> {

        // Define a visitor to handle single values or sequences
        struct SingleOrVecVisitor<T>(std::marker::PhantomData<T>);
 
        impl<'de, T> Visitor<'de> for SingleOrVecVisitor<T>
            where
                T: de::Deserialize<'de> {

            type Value = SingleOrVec;
 
            // Describe what the visitor expects (for error messages)
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a single value or a sequence of values")
            }
 
            // Handle sequences (arrays) by collecting elements into a Vec<T>
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut vec = Vec::new();
                while let Some(element) = seq.next_element()? {
                    vec.push(element);
                }
                Ok(SingleOrVec(vec))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
                where
                    E: de::Error {

                Ok(SingleOrVec(vec![v]))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: de::Error, {

                Ok(SingleOrVec(vec![v.into()]))
            }
        }

        deserializer.deserialize_any(SingleOrVecVisitor(std::marker::PhantomData::<String>))
    }
}

// Helper function to deserialize single value or array into Vec<T>
pub fn single_or_vec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where
        D: Deserializer<'de> {

    SingleOrVec::deserialize(deserializer).map(|sov| sov.into_inner())
}
