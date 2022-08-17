The sample code in this directory demonstrates how to connect to CockroachDB with the [Rust Postgres driver](https://crates.io/crates/postgres).

## Prerequisites

You must have rust and Cargo installed on your local machine, and a running CockroachDB cluster.

## Step 1. Set the `DATABASE_URL` environment variable

Set the `DATABASE_URL` environment variable to the connection URL of your CockroachDB cluster.

~~~ shell
export DATABASE_URL="postgresql://<username>:<password>@<hostname>:<port>/bank?sslmode=require&options=--cluster%3D<routing ID>"
~~~

Where:

* `<username>` is the SQL user on the CockroachDB cluster.
* `<password>` is the password for the SQL user.
* `<hostname>` is the hostname of the CockroachDB cluster.
* `<port>` is the port number on which CockroachDB is running on the host.
* `<routing ID>` is the routing ID cluster if the CockroachDB cluster is a Serverless cluster. Omit the options query parameter if the cluster is a Self-Hosted or Dedicated cluster.

**Note**: You must set `sslmode=require` in the connection URL, as the `postgres` driver does not recognize `sslmode=verify-full`. This example uses `postgres-openssl`, which will perform host verification when the `sslmode=require` option is set, so `require` is functionally equivalent to `verify-full`.

## Step 2. Run the Rust code

Build and run the example:

~~~ shell
cargo run
~~~

After the dependencies are downloaded, the example is compiled and run. You should see output similar to this:

~~~
Finished dev [unoptimized + debuginfo] target(s) in 0.13s
 Running `target/debug/bank`
Balances at 2022-08-15T17:27:21.298523Z:
account id: db735b2b-1072-48b6-bd80-e9b7f521d2b0  balance: 250
account id: f858534f-0c20-4c10-a0d3-402deae77fba  balance: 1000
Final balances at 2022-08-15T17:27:21.734536Z:
account id: db735b2b-1072-48b6-bd80-e9b7f521d2b0  balance: 350
account id: f858534f-0c20-4c10-a0d3-402deae77fba  balance: 900
Deleted accounts.
~~~