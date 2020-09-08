extern crate diesel;
extern crate mobc;
extern crate mobc_diesel;
extern crate tokio;

use diesel::sqlite::SqliteConnection;
use mobc_diesel::ConnectionManager;

#[tokio::main]
async fn main() {
    let manager = ConnectionManager::<SqliteConnection>::new("db.sqlite");
    let pool = mobc::Pool::builder().build(manager);

    for _ in 0..10i32 {
        let pool = pool.clone();
        tokio::spawn(async move {
            let connection = pool.get().await;

            assert!(connection.is_ok());
        });
    }
}
