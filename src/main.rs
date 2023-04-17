use std::env;
use uuid::Uuid;
use openssl::error::ErrorStack;
use openssl::ssl::{SslConnector, SslMethod};
use postgres::{error::SqlState, Client, Error, Transaction};
use postgres_openssl::MakeTlsConnector;

// BEGIN ssl_config
fn ssl_config() -> Result<MakeTlsConnector, ErrorStack> {
    let mut builder = SslConnector::builder(SslMethod::tls())?;
    // set this path to the downloaded CA certificate for your Dedicated cluster 
    builder.set_ca_file("/PATH/TO/DEDICATED/CLUSTER/ca.crt")?;
    Ok(MakeTlsConnector::new(builder.build()))
}
// END ssl_config

// BEGIN execute_txn
/// Runs op inside a transaction and retries it as needed.
/// On non-retryable failures, the transaction is aborted and
/// rolled back; on success, the transaction is committed.
fn execute_txn<T, F>(client: &mut Client, op: F) -> Result<T, Error>
where
    F: Fn(&mut Transaction) -> Result<T, Error>,
{
    let mut txn = client.transaction()?;
    loop {
        // Set a retry savepoint 
        // See https://www.cockroachlabs.com/docs/stable/advanced-client-side-transaction-retries
        let mut sp = txn.savepoint("cockroach_restart")?;
        match op(&mut sp).and_then(|t| sp.commit().map(|_| t)) {
            Err(ref err)
                if err
                    .code()
                    .map(|e| *e == SqlState::T_R_SERIALIZATION_FAILURE)
                    .unwrap_or(false) => {}
            r => break r,
        }
    }
    .and_then(|t| txn.commit().map(|_| t))
}
// END execute_txn

// BEGIN transfer_funds
fn transfer_funds(txn: &mut Transaction, from: Uuid, to: Uuid, amount: i64) -> Result<(), Error> {
    // Read the balance.
    let from_balance: i64 = txn
        .query_one("SELECT balance FROM accounts WHERE id = $1", &[&from])?
        .get(0);

    assert!(from_balance >= amount);

    // Perform the transfer.
    txn.execute(
        "UPDATE accounts SET balance = balance - $1 WHERE id = $2",
        &[&amount, &from],
    )?;
    txn.execute(
        "UPDATE accounts SET balance = balance + $1 WHERE id = $2",
        &[&amount, &to],
    )?;
    Ok(())
}
// END transfer_funds

// BEGIN delete_accounts
fn delete_accounts(client: &mut Client) -> Result<(), Error> {
    client.execute(
        "DELETE FROM accounts", &[],
    )?;
    Ok(())
}
// END delete_accounts

fn main() -> Result<(), Error> {
    let connector = ssl_config().unwrap();
    let connection_uri = env::var("DATABASE_URL").expect("$DATABASE_URL is not set");
    let mut client =
        Client::connect(&connection_uri, connector).unwrap();

    println!("Creating accounts table if it doesn't already exist.");
    // Create the "accounts" table.
    client.execute(
        "CREATE TABLE IF NOT EXISTS accounts (id UUID PRIMARY KEY, balance INT)",
        &[],
    )?;

    // Delete the accounts
    delete_accounts(&mut client)?;
    println!("Deleted existing accounts.");

    // create the accounts and get the IDs
    let ids: Vec<Uuid> =  client.query(
        "INSERT INTO accounts (id, balance) VALUES (gen_random_uuid(), 1000), (gen_random_uuid(), 250) RETURNING id", &[])
        .unwrap()
        .iter()
        .map(|row| row.get(0))
        .collect();

    // Print out the balances.
    println!("Balances before transfer:");
    for row in client.query("SELECT id, balance FROM accounts", &[])? {
        let id: Uuid = row.get(0);
        let balance: i64 = row.get(1);
        println!("account id: {}  balance: ${}", id, balance);
    }
    
    // Run a transfer in a transaction.
    execute_txn(&mut client, |txn| transfer_funds(txn, ids[0], ids[1], 100))?;

    // Check account balances after the transaction.
    println!("Final balances:");
    for row in client.query("SELECT id, balance FROM accounts", &[])? {
        let id: Uuid = row.get(0);
        let balance: i64 = row.get(1);
        println!("account id: {}  balance: ${}", id, balance);
    }
    Ok(())
    
}
