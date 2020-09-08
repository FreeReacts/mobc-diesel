extern crate diesel;
extern crate mobc;
extern crate mobc_diesel;
extern crate tokio;
#[macro_use]
extern crate futures_await_test;

use std::sync::Arc;
use std::sync::mpsc;

use diesel::{pg::PgConnection, sqlite::SqliteConnection};
use mobc_diesel::ConnectionManager;

#[async_test]
async fn pg_basic_connection() {
    let manager = ConnectionManager::<PgConnection>::new("postgres://localhost/");
    let pool = Arc::new(mobc::Pool::builder().build(manager));

    let (s1, r1) = mpsc::channel();
    let (s2, r2) = mpsc::channel();

    let pool1 = pool.clone();
    let t1 = tokio::spawn(async move {
        let conn = pool1.get().await.unwrap();
        s1.send(()).unwrap();
        r2.recv().unwrap();
        drop(conn);
    });

    let pool2 = pool.clone();
    let t2 = tokio::spawn( async move {
        let conn = pool2.get().await.unwrap();
        s2.send(()).unwrap();
        r1.recv().unwrap();
        drop(conn);
    });

    t1.await.unwrap();
    t2.await.unwrap();

    pool.get().await.unwrap();
}

#[async_test]
async fn pg_is_valid() {
    let manager = ConnectionManager::<PgConnection>::new("postgres://localhost/");
    let pool = mobc::Pool::builder().test_on_check_out(true).build(manager);

    pool.get().await.unwrap();
}

#[async_test]
async fn sqlite_basic_connection() {
    let manager = ConnectionManager::<SqliteConnection>::new("test.db");
    let pool = Arc::new(mobc::Pool::builder().build(manager));

    let (s1, r1) = mpsc::channel();
    let (s2, r2) = mpsc::channel();

    let pool1 = pool.clone();
    let t1 = tokio::spawn( async move {
        let conn = pool1.get().await.unwrap();
        s1.send(()).unwrap();
        r2.recv().unwrap();
        drop(conn);
    });

    let pool2 = pool.clone();
    let t2 = tokio::spawn( async move {
        let conn = pool2.get().await.unwrap();
        s2.send(()).unwrap();
        r1.recv().unwrap();
        drop(conn);
    });

    t1.await.unwrap();
    t2.await.unwrap();

    pool.get().await.unwrap();
}

#[async_test]
async fn sqlite_is_valid() {
    let manager = ConnectionManager::<SqliteConnection>::new("test.db");
    let pool = mobc::Pool::builder().test_on_check_out(true).build(manager);

    pool.get().await.unwrap();
}
