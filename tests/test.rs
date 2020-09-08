extern crate diesel;
extern crate mobc;
extern crate mobc_diesel;
extern crate tokio;

use std::sync::Arc;
use tokio::sync::mpsc;

use diesel::{pg::PgConnection, sqlite::SqliteConnection};
use mobc_diesel::ConnectionManager;
use tokio::runtime::Runtime;

#[test]
fn pg_basic_connection() {
    let mut rt = Runtime::new().unwrap();

    // Spawn the root task
    rt.block_on(async {
        let manager = ConnectionManager::<PgConnection>::new("postgres://localhost/");
        let pool = Arc::new(mobc::Pool::builder().build(manager));

        let (mut s1, mut r1) = mpsc::channel(10);
        let (mut s2, mut r2) = mpsc::channel(10);

        let pool1 = pool.clone();
        let t1 = tokio::spawn(async move {
            let conn = pool1.get().await.unwrap();
            s1.send(()).await.unwrap();
            r2.recv().await.unwrap();
            drop(conn);
        });

        let pool2 = pool.clone();
        let t2 = tokio::spawn(async move {
            let conn = pool2.get().await.unwrap();
            s2.send(()).await.unwrap();
            r1.recv().await.unwrap();
            drop(conn);
        });

        t1.await.unwrap();
        t2.await.unwrap();

        pool.get().await.unwrap();
    });
}

#[test]
fn pg_is_valid() {
    let mut rt = Runtime::new().unwrap();

    // Spawn the root task
    rt.block_on(async {
        let manager = ConnectionManager::<PgConnection>::new("postgres://localhost/");
        let pool = mobc::Pool::builder().test_on_check_out(true).build(manager);

        pool.get().await.unwrap();
    });
}

#[test]
fn sqlite_basic_connection() {
    let mut rt = Runtime::new().unwrap();

    // Spawn the root task
    rt.block_on(async {
        let manager = ConnectionManager::<SqliteConnection>::new("test.db");
        let pool = Arc::new(mobc::Pool::builder().build(manager));

        let (mut s1, mut r1) = mpsc::channel(10);
        let (mut s2, mut r2) = mpsc::channel(10);

        let pool1 = pool.clone();
        let t1 = tokio::spawn(async move {
            let conn = pool1.get().await.unwrap();
            s1.send(()).await.unwrap();
            r2.recv().await.unwrap();
            drop(conn);
        });

        let pool2 = pool.clone();
        let t2 = tokio::spawn(async move {
            let conn = pool2.get().await.unwrap();
            s2.send(()).await.unwrap();
            r1.recv().await.unwrap();
            drop(conn);
        });

        t1.await.unwrap();
        t2.await.unwrap();

        pool.get().await.unwrap();
    });
}

#[test]
fn sqlite_is_valid() {
    let mut rt = Runtime::new().unwrap();

    // Spawn the root task
    rt.block_on(async {
        let manager = ConnectionManager::<SqliteConnection>::new("test.db");
        let pool = mobc::Pool::builder().test_on_check_out(true).build(manager);

        pool.get().await.unwrap();
    });
}
