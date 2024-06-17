// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Int4,
        name -> Varchar,
        account -> Varchar,
    }
}

diesel::table! {
    transactions (id) {
        id -> Int4,
        account -> Int4,
        record_date -> Date,
        value_date -> Date,
        description -> Varchar,
        t_type -> Varchar,
        amount -> Numeric,
        balance -> Numeric,
    }
}

diesel::joinable!(transactions -> accounts (account));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    transactions,
);
