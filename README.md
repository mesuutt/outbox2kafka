Periodically read events from outbox table and send to kafka.

----

#### Usage

```
outbox2kafka [OPTIONS]

OPTIONS:
    --db-url <db-url>             DB which contains the outbox table
                                  (e.g. postgres://user:passwd@host:5432/mydb).
    --table-name <table-name>     Outbox table name.
    --brokers <brokers>           Comma separated kafka broker list (default:
                                  localhost:9092).
    --concurrency <concurrency>   Number of worker threads to read outbox table
                                  and send to kafka (default: 1).
    --outbox-check-interval       Interval of fetching new records from outbox
                                  table (default: 10ms).
    --cleaner-run-interval        Interval of deleting old processed records
                                  from outbox table (default: 10m).
    --processed-data-retention    Retention period of processed records in
                                  outbox table (default: 1d).
    --max-db-connection           Max db connection to use (default: 2).

Supported time units: `ms,s,m,h,d,w,mon`
```

Each flag can be given with an env variable.
For example for giving `--db-url` and `--table-name` flags with env var 
you have to use `OUTBOX2KAFKA_DB_URL` and `OUTBOX2KAFKA_TABLE_NAME` respectively.

Also, you can overwrite default logging with `RUST_LOG` env variable. Default: `info,sqlx=error`.
You can read [env_logger documentation](https://docs.rs/env_logger/latest/env_logger/) for more information.

-----

### Outbox table schema

```sql
CREATE TABLE my_outbox_table (
    id uuid NOT NULL,
    aggregate_id character varying(255) NOT NULL, -- using message key and adding to message headers: (e.g. my-user-id) 
    event_type character varying(255) NOT NULL, -- adding to message headers: (e.g. OrderCreated)
    topic character varying(255) NOT NULL, -- topic to send message
    payload text NOT NULL, -- message payload.
    metadata text, -- json serializable object for adding to message headers. (e.g. {"correlation_id": "uuid"})
    occurred_on timestamp with time zone NOT NULL, -- time of the event, using for message ordering 
    processed_date timestamp with time zone -- filled by the crate after message sent to kafka
);
```


### Building

The crate needs to `cmake` for building rdkafka and link it statically to executable.
You can look at the [rdkafka readme file](https://github.com/fede1024/rust-rdkafka#installation) for more information. 
After installing the required dependencies you can build the executable with `cargo build --release`

---

License: MIT
