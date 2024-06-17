use std::error::Error;
use std::path::PathBuf;
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use csv::{ReaderBuilder, Trim::All};
use crate::models::NewTransaction;
use crate::types::TransactionType;

pub fn read_csv_file(path: PathBuf, account: i32) -> Result<Vec<NewTransaction>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .trim(All)
        .from_path(path)?;

    let mut transactions: Vec<NewTransaction> = Vec::new();

    for result in rdr.deserialize() {
        let record: CSVTransaction = result?;
        transactions.push(NewTransaction {
            account,
            record_date: record.record_date,
            value_date: record.value_date,
            description: record.description,
            t_type: record.t_type,
            amount: record.amount,
            balance: record.balance,
        });
    }

    Ok(transactions)
}

#[derive(Debug, PartialEq, serde::Deserialize)]
struct CSVTransaction {
    #[serde(with = "my_date_format")]
    pub record_date: NaiveDate,
    #[serde(with = "my_date_format")]
    pub value_date: NaiveDate,
    pub description: String,
    pub amount: BigDecimal,
    pub t_type: TransactionType,
    pub balance: BigDecimal,
}

mod my_date_format {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer};

    const FORMAT: &'static str = "%d-%m-%Y";

    // pub fn serialize<S>(
    //     date: &NaiveDate,
    //     serializer: S,
    // ) -> Result<S::Ok, S::Error>
    // where
    //     S: Serializer,
    // {
    //     let s = format!("{}", date.format(FORMAT));
    //     serializer.serialize_str(&s)
    // }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(dt)
    }
}
