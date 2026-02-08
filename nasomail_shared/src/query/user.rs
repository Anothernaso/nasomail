use serde::Deserialize;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum UserQuery {
    ById { id: i64 },        // Use an `i64` since that's what SQLite uses internally
    ByName { name: String }, // `name` is implicitly the name of the user because of the name of enum
}
