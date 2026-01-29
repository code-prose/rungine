//src/model.rs

pub mod db {
    use diesel::prelude::*;
    use dotenv::dotenv;
    use std::env;

    pub fn establish_connection() -> SqliteConnection {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL muset be set");
        SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connection to the database: {}", database_url))
    }
}
