extern crate r2d2;
extern crate r2d2_postgres;
extern crate postgres;
extern crate rpassword;

#[macro_use] extern crate prettytable;

use std::io;
use std::io::Write;
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::{TlsMode, PostgresConnectionManager};

use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;

pub type PostgresPool = Pool<PostgresConnectionManager>;
pub type PostgresPooledConnection = PooledConnection<PostgresConnectionManager>;

fn welcome_menu() -> Vec<String> {
  println!("Welcome, please start with connecting to the MariaDB server");
  print!("Host:");
  io::stdout().flush();

  // Prompt for connection parameters and read user input
  let mut host = String::new();
  io::stdin().read_line(&mut host);
  host.pop();

  print!("Database:");
  io::stdout().flush();
  let mut database = String::new();
  io::stdin().read_line(&mut database);
  database.pop();

  print!("Port:");
  io::stdout().flush();
  let mut port = String::new();
  io::stdin().read_line(&mut port);
  port.pop();

  print!("Username:");
  io::stdout().flush();
  let mut username = String::new();
  io::stdin().read_line(&mut username);
  username.pop();

  // Mask password
  let mut passwd = String::new();
  passwd = rpassword::prompt_password_stdout("Password: ").unwrap();

  // Return conn info as a vector
  let mut db_params: Vec<String> = Vec::new();
  db_params.push(host);
  db_params.push(database);
  db_params.push(port);
  db_params.push(username);
  db_params.push(passwd);
  db_params
}

fn list_databases(conn :&PostgresPooledConnection, operation: &i32) {
  println!("\n{}. List of databases", operation);
  let mut table = Table::new();
  table.add_row(row!["DB Name", "Connection Limit", "Tablespace", "Owner"]);
  
  for dbrow in &conn.query("SELECT datname, datconnlimit, spcname, rolname FROM pg_database d, pg_tablespace t, pg_roles r WHERE datistemplate = false AND d.dattablespace=t.oid AND d.datdba=r.oid;", &[]).unwrap() {
    let datname: String = dbrow.get(0);
    let connlimit: i32 = dbrow.get(1);
    let connlimit_str: String = connlimit.to_string();
    let spcname: String = dbrow.get(2);
    let owner: String = dbrow.get(3);
    table.add_row(Row::new(vec![
      Cell::new(&datname),
      Cell::new(&connlimit_str),
      Cell::new(&spcname),
      Cell::new(&owner)]));
  }
  table.printstd();
}

fn list_users(conn :&PostgresPooledConnection, operation: &i32) {
  println!("\n{}. List db users", operation);
  let mut table = Table::new();
  table.add_row(row!["Username", "CreateDB", "SuperUser", "Replication", "Bypass Rowlevel Sec"]);

  for dbrow in &conn.query("select usename, usecreatedb, usesuper, userepl, usebypassrls, valuntil from pg_user;", &[]).unwrap() {
    let username: String = dbrow.get(0);
    let createdb: bool = dbrow.get(1);
    let createdb_str = createdb.to_string();
    let superuser: bool = dbrow.get(2);
    let superuser_str = superuser.to_string();
    let repl: bool = dbrow.get(3);
    let repl_str = repl.to_string();
    let bypass: bool = dbrow.get(4);
    let bypass_str = bypass.to_string();
    table.add_row(Row::new(vec![
      Cell::new(&username),
      Cell::new(&createdb_str),
      Cell::new(&superuser_str),
      Cell::new(&repl_str),
      Cell::new(&bypass_str)]));
  }
  table.printstd();
  
}

fn list_activities(conn :&PostgresPooledConnection, operation: &i32) {
  println!("\n{}. List db activities", operation);
  let mut table = Table::new();
  table.add_row(row!["DB Name", "Pid", "Username", "Application Name"]);

  for dbrow in &conn.query("select datname, pid, usename, application_name, client_addr from pg_stat_activity;", &[]).unwrap() {
    let dbname: String = dbrow.get(0);
    let pid: i32 = dbrow.get(1);
    let pid_str = pid.to_string();
    let username: String = dbrow.get(2);
    let appname: String = dbrow.get(3);
    //let client: String = dbrow.get(4);
    table.add_row(Row::new(vec![
      Cell::new(&dbname),
      Cell::new(&pid_str),
      Cell::new(&username),
      Cell::new(&appname)]));
      //Cell::new(&client)]));
  }
  table.printstd();
}

fn create_database(conn :&PostgresPooledConnection, operation: &i32) {
  println!("\n{}. Create database", operation);
  print!("DB name: "); 
  io::stdout().flush().unwrap();
  let mut dbname = String::new();
  io::stdin().read_line(&mut dbname)
    .expect("Error reading password");

  print!("DB owner: "); 
  io::stdout().flush().unwrap();
  let mut owner = String::new();
  io::stdin().read_line(&mut owner)
    .expect("Error reading password");

  // Not working as rust-postgres driver does not support parameters for 'create database'
  // statements
  let sql = &conn.execute("create database $1 owner $2", &[&dbname,&owner]);
}

fn create_user(conn :&PostgresPooledConnection, operation: &i32) {
  println!("\n{}. Create db user", operation);
  print!("Username: ");
  io::stdout().flush().unwrap();
  let mut username = String::new();
  io::stdin().read_line(&mut username)
    .expect("Error reading password");

  print!("Password: ");
  io::stdout().flush().unwrap();
  let mut passwd = String::new();
  io::stdin().read_line(&mut passwd)
    .expect("Error reading password");

  // Not working as rust-postgres driver does not support parameters for 'create role'
  // statements
  &conn.execute("CREATE ROLE $1 PASSWORD $2", &[&username,&passwd]);
}

// Main menu of the application
fn db_menu(conn :&PostgresPooledConnection) {
  println!("Select the operation you want to perform");
  println!("1. Create database");
  println!("2. Create user");
  println!("3. List databases");
  println!("4. List users");
  println!("5. List activities");

  // Keep looping to keep the menu on
  loop {
    print!("Operation (eg: 1):");
    io::stdout().flush().unwrap();

    let mut operation = String::new();
    io::stdin().read_line(&mut operation)
      .expect("dsds");

    // Exit immediately if user decides to quit
    match &*(operation).trim() {
      "q" => {
        println!("\nThank you for using pgclir!");
        break;
      },
      "Q" => {
        println!("\nThank you for using pgclir!");
        break;
      },
      _ => (),
    }

    let operation: i32 = match operation.trim().parse() {
      Ok(num) if num > 0 && num < 6 => num,
      Ok(_) => continue,
      Err(_) => continue,
    };

    // Redirect to the correct function according to user preference
    match operation {
      1 => create_database(&conn, &operation),
      2 => create_user(&conn, &operation),
      3 => list_databases(&conn, &operation),
      4 => list_users(&conn, &operation),
      5 => list_activities(&conn, &operation),
      _ => println!("dsds"),
    }
  }
}

// Seting up the db connection pool
fn setup_connection_pool(cn_str: &str, pool_size: u32) -> PostgresPool {
    let manager = ::r2d2_postgres::PostgresConnectionManager::new(cn_str, TlsMode::None).unwrap();
    let config = ::r2d2::Config::builder().pool_size(pool_size).build();
    ::r2d2::Pool::new(config, manager).unwrap()
}

fn main() {
  let mut params: Vec<String> = Vec::new();

  // Welcome message and prompt for connection info
  params = welcome_menu();

  // Construction of DB connection string
  let conn_string = format!("postgres://{}:{}@{}:{}/{}", &params[3], &params[4], &params[0], &params[2], &params[1]);

  // DB connection pooling
  let pool = setup_connection_pool(&conn_string, 1);
  let conn = pool.get().unwrap();

  // Display DB operations menu
  println!("\nYou are now connected to: {}\n", &params[0]);
  db_menu(&conn);
}
