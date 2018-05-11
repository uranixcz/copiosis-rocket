/*
* Copyright 2018 Michal Mauser
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU Affero General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU Affero General Public License for more details.
*
* You should have received a copy of the GNU Affero General Public License
* along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate rusqlite;

//#[cfg(test)] mod tests;

use std::sync::Mutex;
use rocket::Rocket;
use rusqlite::Connection;
use rocket::request::Form;
#[macro_use] extern crate serde_derive;
use rocket_contrib::Template;
use rocket::State;
use rocket::response::Redirect;
use rocket::request::FlashMessage;
use rocket::response::Flash;
use rocket::fairing::AdHoc;

type DbConn = Mutex<Connection>;

#[derive(FromForm, Serialize)]
struct User {
    id: i64,
    name: String,
    nbr: i64,
    //password: String,
    time_created: String,
}

#[derive(FromForm, Serialize)]
struct Product {
    id: i64,
    name: String,
    gateway: i64,
    benefit: i64,
    time_created: String,
}

fn init_database(conn: &Connection) {
    conn.execute("CREATE TABLE IF NOT EXISTS users (
                    id              INTEGER PRIMARY KEY AUTOINCREMENT,
                    name            TEXT NOT NULL,
                    NBR             INTEGER NOT NULL,
                    password        TEXT NOT NULL,
                    time_created    TEXT NOT NULL
                    )", &[])
        .expect("create users table");

    /*conn.execute("INSERT INTO users (name, password, NBR) VALUES ($1, $2, $3)",
                 &[&"Rocketeer", &"bla", &0])
        .expect("insert single entry into entries table");

    conn.execute("INSERT INTO users (name, password, NBR) VALUES ($1, $2, $3)",
                 &[&"Rocketeer1", &"bla1", &0])
        .expect("insert single entry into entries table");*/

    conn.execute("CREATE TABLE IF NOT EXISTS products (
                    id          INTEGER PRIMARY KEY AUTOINCREMENT,
                    name        TEXT NOT NULL,
                    gateway     INTEGER NOT NULL,
                    benefit     INTEGER NOT NULL,
                    time_created    TEXT NOT NULL
                )", &[])
        .expect("create products table");

    /*conn.execute("INSERT INTO products (name, gateway, benefit) VALUES ($1, $2, $3)",
                 &[&"jablka 100g", &0, &5])
        .expect("insert single entry into entries table");*/

    conn.execute("CREATE TABLE IF NOT EXISTS transfers (
                    id          INTEGER PRIMARY KEY AUTOINCREMENT,
                    ConsumerID     INTEGER NOT NULL,
                    ProducerID INTEGER NOT NULL,
                    ProductID  INTEGER NOT NULL,
                    amount     INTEGER NOT NULL,
                    NBR        INTEGER NOT NULL,
                    time_created    TEXT NOT NULL
                )", &[])
        .expect("create withdrawals table");

    /*conn.execute("CREATE TABLE offers (
                    id          INTEGER PRIMARY KEY,
                    user_id     INTEGER NOT NULL,
                    product_id  INTEGER NOT NULL,
                    quantity    INTEGER NOT NULL,
                    is_complete INTEGER NOT NULL
                )", &[])
        .expect("create offers table");*/
}

#[get("/")]
fn index(flash: Option<FlashMessage>) -> Template {
    match flash {
        Some(x) => Template::render("index", x.msg()),
        _ => Template::render("index", "")
    }
    /*let mut context = HashMap::new();
    if let Some(ref msg) = flash {
        context.insert("flash", msg.msg());
    }

    Template::render("login", &context)*/
}

#[get("/adduser")]
fn adduser_page() -> Template {
    Template::render("adduser", "")
}

#[post("/adduser", data = "<user>")]
fn adduser(user: Form<User>, db_conn: State<DbConn>, templatedir: State<TemplateDir>) -> Flash<Redirect> {
    let user = user.into_inner();
    let tmpconn = db_conn.lock()
        .expect("db connection lock");

    tmpconn.execute("INSERT INTO users (name, NBR, password, time_created) VALUES ($1, $2, $3, datetime('now', 'localtime'))",
                    &[&user.name, &0, &"0"])
        .expect("insert single entry into products table");

    Flash::success(Redirect::to("/"),
                   if templatedir.0.eq("templates_cz") { "Uživatel přidán." }
                            else { "User added." })
}

#[get("/users")]
fn users(db_conn: State<DbConn>) -> Template {
    let tmpconn = db_conn.lock()
        .expect("db connection lock");
    let mut stmt = tmpconn
        .prepare("SELECT id, name, NBR, time_created FROM users")
        .unwrap();

    let user_iter = stmt.query_map(&[], |row| {
        User {
            id: row.get(0),
            name: row.get(1),
            nbr: row.get(2),
            time_created: row.get(3),
        }
    }).unwrap();

    let mut vct = Vec::new();
    for user in user_iter {
        vct.push(user.unwrap());
    }

    Template::render("users", vct)
}

#[get("/addproduct")]
fn addproduct_page() -> Template {
    Template::render("addproduct", "")
}

#[post("/addproduct", data = "<product>")]
fn addproduct(product: Form<Product>, db_conn: State<DbConn>, templatedir: State<TemplateDir>) -> Flash<Redirect> {
    let tmpconn = db_conn.lock()
        .expect("db connection lock");

    let product = product.into_inner();
    tmpconn.execute("INSERT INTO products (name, gateway, benefit, time_created)VALUES ($1, $2, $3, datetime('now', 'localtime'))",
                 &[&product.name, &product.gateway, &product.benefit])
        .expect("insert single entry into products table");

    Flash::success(Redirect::to("/"),
                   if templatedir.0.eq("templates_cz") { "Produkt přidán." }
                            else { "Product added." })
}

#[get("/products")]
fn products(db_conn: State<DbConn>) -> Template {
    let tmpconn = db_conn.lock()
        .expect("db connection lock");
    let mut stmt = tmpconn
        .prepare("SELECT id, name, gateway, benefit, time_created FROM products")
        .unwrap();

    let product_iter = stmt.query_map(&[], |row| {
        Product {
            id: row.get(0),
            name: row.get(1),
            gateway: row.get(2),
            benefit: row.get(3),
            time_created: row.get(4),
        }
    }).unwrap();

    let mut vct = Vec::new();
    for product in product_iter {
        vct.push(product.unwrap());
    }

    Template::render("products", vct)
}

#[derive(Serialize)]
struct ContextTransfer {
    users: Vec<User>,
    products: Vec<User>,
}

#[get("/transfer")]
fn transfer_page(conn: State<DbConn> ) -> Template {
    let tmpconn = conn.lock()
        .expect("db connection lock");

    let mut users = Vec::new();
    let mut stmt = tmpconn
        .prepare("SELECT id, name FROM users")
        .unwrap();
    {
        let user_iter = stmt.query_map(&[], |row| {
            User {
                id: row.get(0),
                name: row.get(1),
                nbr: 0,
                time_created: String::from(""),
            }
        }).unwrap();
        for user in user_iter {
            users.push(user.unwrap());
        }
    }
    stmt = tmpconn
        .prepare("SELECT id, name FROM products")
        .unwrap();
    let item_iter = stmt.query_map(&[], |row| {
        User { //yes, this is on purpose to save data
            id: row.get(0),
            name: row.get(1),
            nbr: 0,
            time_created: String::from(""),
        }
    }).unwrap();
    let mut products = Vec::new();
    for item in item_iter {
        products.push(item.unwrap());
    }

    let context: ContextTransfer = ContextTransfer {
        users,
        products,
        //nbr: 10
    };

    Template::render("transfer", &context)
}

#[derive(FromForm, Serialize)]
struct Transfer {
    producer: i64,
    consumer: i64,
    product: i64,
    amount: i64,
    nbr: i64,
}

#[post("/transfer", data = "<post>")]
fn transfer(conn: State<DbConn>, post: Form<Transfer>, templatedir: State<TemplateDir>) -> Flash<Redirect> {
    let transfer = post.into_inner();

    let product_params:(i64, i64) = conn.lock()
        .expect("db connection lock")
        .query_row("SELECT gateway, benefit FROM products WHERE id = $1",
                   &[&transfer.product], |row| { (row.get(0), row.get(1)) })
        .expect("product does not exist");

    let tmpconn = conn.lock()
        .expect("db connection lock");
    tmpconn.execute("INSERT INTO transfers (ProducerID, ConsumerID, ProductID, amount, NBR, time_created)\
    VALUES ($1, $2, $3, $4, $5, datetime('now', 'localtime'))",
                 &[&transfer.producer, &transfer.consumer, &transfer.product, &transfer.amount, &(product_params.1 * transfer.amount)])
        .expect("insert single entry into transfers table");
    tmpconn.execute("UPDATE users SET NBR = NBR + $1 WHERE id = $2",
                    &[&(product_params.1 * transfer.amount), &transfer.producer])
        .expect("update producer entry in transfers table");
    tmpconn.execute("UPDATE users SET NBR = NBR - $1 WHERE id = $2",
                    &[&(product_params.0 * transfer.amount), &transfer.consumer])
        .expect("update consumer entry in transfers table");


    Flash::success(Redirect::to("/"),
                   if templatedir.0.eq("templates_cz") { "Transfer proveden." }
                            else { "Transfer COMPLETE." })
}

#[derive(Serialize)]
struct NamedTransfer {
    id: i64,
    producer: String,
    consumer: String,
    product: String,
    amount: i64,
    nbr: i64,
    time_created: String,
}

#[get("/transfers")]
fn transfers(conn: State<DbConn>) -> Template {
    let tmpconn = conn.lock()
        .expect("db connection lock");

    let mut transfers = Vec::new();
    let mut stmt = tmpconn
        .prepare("SELECT t2.id          AS table2_id
        , t2.ConsumerID
        , t2.ProducerID
        , t31.name      AS x
        , t32.name      AS y
        , p.name		 AS z
	    , t2.amount
	    , t2.NBR
        , t2.time_created
        FROM   transfers t2
        LEFT   JOIN users t31 ON t31.id = t2.ConsumerID
        LEFT   JOIN users t32 ON t32.id = t2.ProducerID
        LEFT   JOIN products p ON p.id = t2.ProductID
        ORDER BY t2.id DESC LIMIT 100;")
        .unwrap();
    {
        let transfer_iter = stmt.query_map(&[], |row| {
            NamedTransfer {
                id: row.get(0),
                producer: row.get(4),
                consumer: row.get(3),
                product: row.get(5),
                amount: row.get(6),
                nbr: row.get(7),
                time_created: row.get(8),
            }
        }).unwrap();
        for transfer in transfer_iter {
            transfers.push(transfer.unwrap());
        }
    }

    Template::render("transfers", &transfers)
}

struct TemplateDir(String);

fn rocket() -> Rocket {
    // Open a new in-memory SQLite database.
    //let conn = Connection::open_in_memory().expect("in memory db");
    let conn = Connection::open("copiosis.sqlite").expect("db file");

    // Initialize the `entries` table in the database.
    init_database(&conn);

    // Have Rocket manage the database pool.
    rocket::ignite()
        .attach(Template::fairing())
        .attach(AdHoc::on_attach(|rocket| {
            println!("Adding token managed state from config...");
            let token_val = rocket.config().get_str("template_dir").unwrap_or("").to_string();
            Ok(rocket.manage(TemplateDir(token_val)))
        }))
        .manage(Mutex::new(conn))
        .mount("/", routes![index, adduser_page, addproduct_page, addproduct, adduser,
        transfer_page, transfer, transfers, users, products])
}

fn main() {
    rocket().launch();
}
