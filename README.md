The sample code in this directory demonstrates how to connect to CockroachDB with the [Rust Postgres driver](https://crates.io/crates/postgres).

## Step 1. Specify the Rust Postgres driver as a dependency

Update your `Cargo.toml` file to specify a dependency on the Rust Postgres driver, as described in the [Rust Postgres driver](https://crates.io/crates/postgres).

Additionally, include the [OpenSSL bindings](https://crates.io/crates/postgres-openssl) and [Rust Postgres OpenSSL](https://crates.io/crates/postgres-openssl) crates as dependencies.

## Step 2. Create the `maxroach` users and `bank` database

Start the built-in SQL shell:

~~~ shell
$ cockroach sql --certs-dir=certs
~~~

In the SQL shell, issue the following statements to create the `maxroach` user and `bank` database:

~~~ sql
> CREATE USER IF NOT EXISTS maxroach;
~~~

~~~ sql
> CREATE DATABASE bank;
~~~

Give the `maxroach` user the necessary permissions:

~~~ sql
> GRANT ALL ON DATABASE bank TO maxroach;
~~~

Exit the SQL shell:

~~~ sql
> \q
~~~

## Step 3. Generate a certificate for the `maxroach` user

Create a certificate and key for the `maxroach` user by running the following command.  The code samples will run as this user.

~~~ shell
$ cockroach cert create-client maxroach --certs-dir=certs --ca-key=my-safe-directory/ca.key
~~~

## Step 4. Run the Rust code

### Basic statements

Run `basic-sample.rs` to connect as the `maxroach` user and execute some basic SQL statements, inserting rows and reading and printing the rows.

You might need to open `basic-sample.rs`, and edit the connection configuration parameters to connect to your cluster.

### Transaction (with retry logic)

Run `txn-sample.rs` to again connect as the `maxroach` user but this time execute a batch of statements as an atomic transaction to transfer funds from one account to another, where all included statements are either committed or aborted.

You might need to open `txn-sample.rs`, and edit the connection configuration parameters to connect to your cluster.
