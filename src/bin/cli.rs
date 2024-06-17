use std::path::PathBuf;
use diesel::prelude::*;
use money_tracker::models::*;
use money_tracker::csv::read_csv_file;
use money_tracker::schema::transactions;
use money_tracker::schema::accounts::dsl::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'a', long)]
    bank_account: String,

    #[arg(short = 'b', long)]
    bank_name: String,

    #[arg(short = 'f', long)]
    transaction_file: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let connection = &mut money_tracker::establish_connection();
    let bank_account = accounts
        .filter(name.eq(args.bank_name))
        .filter(account.eq(args.bank_account))
        .select(id)
        .first(connection);

    if bank_account.is_err() {
        return Err(("Bank account not found").into())
    }

    match read_csv_file(PathBuf::from(args.transaction_file), bank_account.unwrap()) {
        Ok(records) => {
            diesel::insert_into(transactions::table)
                .values(&records)
                .returning(Transaction::as_returning())
                .get_result(connection)?;
        }
        Err(e) => return Err(format!("CSV Error: {}", e).into()),
    }

    Ok(())
}