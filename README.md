Periodically read events from outbox table and send to kafka.

----

#### CLI Args

```
    --db-url <db-url>
    --table-name <table-name>
    --brokers <brokers>
    --topic <topic>
    --concurrency <concurrency>      
    --max-db-connection <max-db-connection> 
    --outbox-check-interval <outbox-check-interval> 
    --cleaner-run-interval <cleaner-run-interval>
    --processed-data-retention <processed-data-retention>
```

<details>
 <summary>Show details of each argument</summary>

Options can be given with flag or env variable

**--db-url**

DB which contains the outbox table (`postgres://user:passwd@host:5432/mydb`). 

**--table-name**

Outbox table name

**--brokers**

Comma separated kafka broker list. Default: `localhost:9092`

**--concurrency**

Number of workers to read outbox table and send to kafka. Default: `1`

**--outbox-check-interval**

Interval of fetching new records from outbox table, time units: `ms,s,m,h,d,w,mon`. Default: `10ms`

**--cleaner-run-interval**

Interval of deleting old processed records from outbox table. `0` means never delete. 
Supported time units: `m,h,d,w,mon`. Default: `10m`

**--processed-data-retention**

Retention period of processed records in outbox table. `0` means never. 
Supported time units: `ms,s,m,h,d,w,mon`. Default: `1h`

**--max-db-connection**

Max db connection to use. You may give `concurrency` + 1, Default: `2`

</details>

Each flag can be given with an env variable.
For example for giving `--db-url` flag with env var you have to use `OUTBOX2KAFKA_DB_URL`.

Also, you can overwrite default logging with `RUST_LOG` env variable. Default: `info,sqlx=error`.
You can read [env_logger documentation](https://docs.rs/env_logger/latest/env_logger/) for more information.

-----

### Outbox table schema

```sql
CREATE TABLE my_outbox_table (
    id uuid NOT NULL,
    aggregate_id character varying(255) NOT NULL, -- using message key and adding to event headers
    event_type character varying(255) NOT NULL, -- adding to event headers
    payload text NOT NULL,
    metadata text,
    occurred_on timestamp with time zone NOT NULL,
    processed_date timestamp with time zone
);
```

---

License: MIT
