use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, sqlx::Type, Serialize, Deserialize)]
#[sqlx(rename = "varchar", rename_all = "camelCase")]
#[serde(rename_all(deserialize = "camelCase"))]
pub enum Occupation {
    Engineer,
    Manager,
}
