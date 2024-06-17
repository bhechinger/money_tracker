use diesel::prelude::*;
use chrono::NaiveDate;
use bigdecimal::BigDecimal;
use crate::types::TransactionType;

#[derive(Insertable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewAccount {
    pub name: String,
    pub account: String,
}

#[derive(Selectable, Queryable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub account: String,
}

// build_model!(Account; NewAccount; accounts => {
//     name: String,
//     account: String
// });

#[derive(Insertable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::transactions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewTransaction {
    pub account: i32,
    pub record_date: NaiveDate,
    pub value_date: NaiveDate,
    pub description: String,
    pub t_type: TransactionType,
    pub amount: BigDecimal,
    pub balance: BigDecimal,
}

#[derive(Selectable, Queryable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::transactions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Transaction {
    pub id: i32,
    pub account: i32,
    pub record_date: NaiveDate,
    pub value_date: NaiveDate,
    pub description: String,
    pub t_type: TransactionType,
    pub amount: BigDecimal,
    pub balance: BigDecimal,
}
