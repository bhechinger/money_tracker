use diesel::deserialize::FromSql;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::*;
use std::io::Write;
use serde::Deserialize;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq, Deserialize)]
#[diesel(sql_type = Text)]
pub enum TransactionType {
    Credit,
    Debit
}

impl ToSql<Text, Pg> for TransactionType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            TransactionType::Credit => out.write_all(b"Credit")?,
            TransactionType::Debit => out.write_all(b"Debit")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Pg> for TransactionType {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Credit" => Ok(TransactionType::Credit),
            b"Debit" => Ok(TransactionType::Debit),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

// impl<'de> Deserialize<'de> for TransactionType {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         #[derive(Debug, Deserialize)]
//         struct Mapping {
//             field: i32,
//             #[serde(rename = "A")]
//             a: Option<i32>,
//             #[serde(rename = "B")]
//             b: Option<i32>,
//         }
//
//         let Mapping { field, a, b } = Mapping::deserialize(deserializer)?;
//
//         match (a, b) {
//             (Some(_), Some(_)) =>
//                 Err(serde::de::Error::custom("multiple variants specified")),
//             (Some(a), None) =>
//                 Ok(TransactionType::Credit),
//             (None, Some(b)) =>
//                 Ok(TransactionType::Debit),
//             (None, None) =>
//                 Err(D::Error::custom("no variants specified")),
//         }
//     }
// }