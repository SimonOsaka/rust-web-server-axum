## repository
using sqlx and postgres

## key point
- read from db
    - `query_list_tx_optional`: query many results using transaction or not
    - `query_one_tx_optional`: query one result using transaction or not
- write to db
    - `insert_one_tx_optional`: insert a record using transaction or not
    - `update_tx_optional`: update record(s) using transaction or not
    - `delete_tx_optional`: delete record(s) using transaction or not
    - `delete_tx_optional`: delete record(s) using transaction or not
- two or more tables join and return results
    1. table
    ```sql
        CREATE TABLE A {
            id SERIAL NOT NULL,
            name VARCHAR(20) NOT NULL DEFAULT '',
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            CONSTRAINT "a_pk" PRIMARY KEY ("id")
        }

        CREATE TABLE B {
            id SERIAL NOT NULL,
            name VARCHAR(20) NOT NULL DEFAULT '',
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            a_id int4 NOT NULL DEFAULT 0,
            CONSTRAINT "b_pk" PRIMARY KEY ("id")
        }
    ```
    2. struct
    ```rust
        #[derive(sqlx::FromRow, sqlx::Type, Debug, Clone)]
        #[sqlx(type_name = "RECORD")] // required
        pub struct A {
            pub id: i64,
            pub name: String
            pub created_at: NaiveDateTime
        }

        #[derive(sqlx::FromRow, sqlx::Type, Debug, Clone)]
        #[sqlx(type_name = "RECORD")] // required
        pub struct B {
            pub id: i64,
            pub name: String
            pub created_at: NaiveDateTime
            pub a_id: i64
        }

        #[derive(sqlx::FromRow, Debug, Clone)]
        pub struct Result {
            pub ra: A,
            pub rb: B
        }
    ```
    3. query
    ```sql
        SELECT 
            (a.id, a.name, a.created_at) AS "ra",
            (b.id, b.name, b.created_at, b.a_id) AS "rb"
        FROM 
            A AS a
        LEFT JOIN B AS b 
            ON a.id = b.a_id
    ```
    4. mapper
    ```rust
        let query_result: Vec<Result> = sqlx::query_as(sql).fetch_all(pool).await?;
        let result = query_result.into_iter().map(|r| (r.ra, r.rb)).collect(); // don't focus this line, result is Vec<(A, B)>
    ```
- connection
    ```rust
        let connection_pool = PoolOptions::new()
            .max_connections(10)
            .min_connections(1)
            .connect_timeout(std::time::Duration::from_secs(30))
            .after_connect(|conn| {
                Box::pin(async move {
                    conn.execute("SET TIME ZONE 'Asia/Shanghai';").await?; // timezone for postgres

                    Ok(())
                })
            })
            .connect(&database_url)
            .await
            .expect("init database error");
    ```