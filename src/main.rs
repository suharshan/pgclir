extern crate r2d2;
extern crate r2d2_postgres;
extern crate postgres;

use std::io;
use std::io::Write;
use std::thread;
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::{TlsMode, PostgresConnectionManager};
//use postgres::{Connection, TlsMode};

pub type PostgresPool = Pool<PostgresConnectionManager>;
pub type PostgresPooledConnection = PooledConnection<PostgresConnectionManager>;

fn welcome_menu() {
  println!("Welcome, please start with connecting to the MariaDB server");
  print!("Host:");
  io::stdout().flush().unwrap();

  let mut host = String::new();
  io::stdin().read_line(&mut host)
    .expect("Error reading host");

  print!("Username:");
  io::stdout().flush().unwrap();
  let mut username = String::new();
  io::stdin().read_line(&mut username)
    .expect("Error reading username");

  print!("Password:");
  io::stdout().flush().unwrap();
  let mut passwd = String::new();
  io::stdin().read_line(&mut passwd)
    .expect("Error reading password");
}

fn list_databases(conn :&PostgresPooledConnection) {
  //let stmt = conn.prepare("SELECT datname FROM pg_database WHERE datistemplate = false;").unwrap();
  //for row in stmt.query(&[]).unwrap() {
  for row in &conn.query("SELECT datname FROM pg_database WHERE datistemplate = false;", &[]).unwrap() {
    let datname: String = row.get(0);
    println!("DB name: {}", datname);
  }
}

fn db_menu(conn :&PostgresPooledConnection) {
  println!("Select the operation you want to perform");
  println!("1. List databases");
  println!("2. List users");

  loop {
    print!("Operation (eg: 1):");
    io::stdout().flush().unwrap();

    let mut operation = String::new();
    io::stdin().read_line(&mut operation)
      .expect("dsds");

    let operation: u32 = match operation.trim().parse() {
      Ok(num) if num > 0 && num < 6 => num,
      Ok(_) => continue,
      Err(_) => continue,
    };

    match operation {
      1 => list_databases(&conn),
      2 => println!("2"),
      3 => println!("3"),
      4 => println!("4"),
      _ => println!("dsds"),
    }
  }
}

fn setup_connection_pool(cn_str: &str, pool_size: u32) -> PostgresPool {
    let manager = ::r2d2_postgres::PostgresConnectionManager::new(cn_str, TlsMode::None).unwrap();
    let config = ::r2d2::Config::builder().pool_size(pool_size).build();
    ::r2d2::Pool::new(config, manager).unwrap()
}

fn insert_dummy_data(conn :&PostgresPooledConnection) {
    conn.execute("DROP TABLE IF EXISTS messages;", &[]).unwrap();    
    conn.execute("CREATE TABLE IF NOT EXISTS messages (id INT PRIMARY KEY);", &[]).unwrap();
    conn.execute("INSERT INTO messages VALUES (1);", &[]).unwrap();
    conn.execute("INSERT INTO messages VALUES (2);", &[]).unwrap();
    conn.execute("INSERT INTO messages VALUES (3);", &[]).unwrap();    
}

fn main() {
  welcome_menu();
  let conn_string = String::from("postgres://postgres:Admin123.!@rust-postgres.cnkyd5c6frx2.us-east-2.rds.amazonaws.com:5432/pub");

  let pool = setup_connection_pool(&conn_string, 1);
  let conn = pool.get().unwrap();

  loop {
    db_menu(&conn);
  //println!("inserting dummy data.");
  //insert_dummy_data(&conn);
  }
}
