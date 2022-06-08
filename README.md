## aurach
### integration with mysql via diesel and r2d2 [https://diesel.rs/]
### web framework use rocket[https://rocket.rs/]
### how to start
1. add database_url to src/db/mysql_db_pool.rs
2. create table `aurach_user` to database
3. start via main.rs
> rust should use lightly, like this `rustup default nightly`

// todo
1. add parquet storage obj
2. storage and connection abstract
3. add server and client