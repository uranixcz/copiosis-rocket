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
    resabundance: i64,
    resabundancetun: i64,
    prodpop: i64,
    consdem: i64,
    proddembalance: i64,
    conssubsat: i64,
    conssubsattun: i64,
    consobjben: i64,
    consobjbentun: i64,
    consbenefit: i64,
    socbenefit: i64,
    socbenefittun: i64,
    enveffect: i64,
    enveffecttun: i64,
    humaneffect: i64,
    humaneffecttun: i64,
    envbenefit: i64,
}

fn init_database(conn: &Connection) {

    fn upgrade_message(version: usize) {
        println!("Upgrading DB version {}, stand by...", version);
    }

    let mut db_version:i64 = conn.query_row("PRAGMA user_version",&[], |row| {row.get(0)})
                     .expect("lookup db table version");
    if db_version == 0 {
        upgrade_message(0);
        let altered = conn.execute("ALTER TABLE transfers ADD COLUMN GNBR INTEGER NOT NULL DEFAULT 0", &[]).is_ok();
        if altered {
            conn.execute("UPDATE transfers SET GNBR = amount * (SELECT products.gateway
                    FROM products
                    WHERE products.id = transfers.ProductID )
                    WHERE EXISTS (
                    SELECT *
                    FROM products
                    WHERE products.id = transfers.ProductID
                )", &[])
                .expect("update producer entry in transfers table");
            conn.execute("PRAGMA user_version = 1", &[])
                .expect("alter db version");
            db_version = 1;
        } else {
            conn.execute("CREATE TABLE IF NOT EXISTS users (
                    id              INTEGER PRIMARY KEY AUTOINCREMENT,
                    name            TEXT NOT NULL,
                    NBR             INTEGER NOT NULL,
                    password        TEXT NOT NULL,
                    time_created    TEXT NOT NULL
                    )", &[])
                .expect("create users table");

            conn.execute("CREATE TABLE IF NOT EXISTS products (
                    id          INTEGER PRIMARY KEY AUTOINCREMENT,
                    name        TEXT NOT NULL,
                    gateway     INTEGER NOT NULL,
                    benefit     INTEGER NOT NULL,
                    time_created    TEXT NOT NULL,
                    resabundance    INTEGER,
                    resabundancetun INTEGER,
                    prodpop     INTEGER,
                    consdem     INTEGER,
                    proddembalance  INTEGER,
                    conssubsat  INTEGER,
                    conssubsattun   INTEGER,
                    consobjben  INTEGER,
                    consobjbentun   INTEGER,
                    consbenefit  INTEGER,
                    socbenefit  INTEGER,
                    socbenefittun   INTEGER,
                    enveffect  INTEGER,
                    enveffecttun   INTEGER,
                    humaneffect INTEGER,
                    humaneffecttun  INTEGER,
                    envbenefit INTEGER
                )", &[])
                .expect("create products table");

            conn.execute("CREATE TABLE IF NOT EXISTS transfers (
                    id          INTEGER PRIMARY KEY AUTOINCREMENT,
                    ConsumerID     INTEGER NOT NULL,
                    ProducerID INTEGER NOT NULL,
                    ProductID  INTEGER NOT NULL,
                    amount     INTEGER NOT NULL,
                    NBR        INTEGER NOT NULL,
                    time_created    TEXT NOT NULL,
                    GNBR       INTEGER NOT NULL
                )", &[])
                .expect("create withdrawals table");
            conn.execute("PRAGMA user_version = 2", &[])
                .expect("alter db version");
            db_version = 2;
        }

    }
    if db_version == 1 {
        upgrade_message(1);
        conn.execute("CREATE TABLE IF NOT EXISTS transfers2 (
                    id          INTEGER PRIMARY KEY AUTOINCREMENT,
                    ConsumerID     INTEGER NOT NULL,
                    ProducerID INTEGER NOT NULL,
                    ProductID  INTEGER NOT NULL,
                    amount     INTEGER NOT NULL,
                    NBR        INTEGER NOT NULL,
                    time_created    TEXT NOT NULL,
                    GNBR       INTEGER NOT NULL
                )", &[]).expect("alter db modify column");
        conn.execute("INSERT INTO transfers2 SELECT * FROM transfers", &[]).expect("alter db modify column");
        conn.execute("DROP TABLE transfers", &[]).expect("alter db modify column");
        conn.execute("ALTER TABLE transfers2 RENAME TO transfers", &[]).expect("alter db modify column");

        conn.execute("ALTER TABLE products ADD COLUMN resabundance INTEGER", &[]).expect("alter db add column");
        conn.execute("ALTER TABLE products ADD COLUMN resabundancetun INTEGER", &[]).expect("alter db add column");
        conn.execute("ALTER TABLE products ADD COLUMN prodpop INTEGER", &[]).expect("alter db add column");
        conn.execute("ALTER TABLE products ADD COLUMN consdem INTEGER", &[]).expect("alter db add column");
        conn.execute("ALTER TABLE products ADD COLUMN proddembalance INTEGER", &[]).expect("alter db add column");
        conn.execute("ALTER TABLE products ADD COLUMN conssubsat INTEGER", &[]).expect("alter db add column");
        conn.execute("ALTER TABLE products ADD COLUMN conssubsattun INTEGER", &[]).expect("alter db add column");
        conn.execute("ALTER TABLE products ADD COLUMN consobjben INTEGER", &[]).expect("alter db add column");
        conn.execute("ALTER TABLE products ADD COLUMN consobjbentun INTEGER", &[]).expect("alter db add column");
        conn.execute("ALTER TABLE products ADD COLUMN consbenefit INTEGER", &[]).expect("alter db add column");
        conn.execute("ALTER TABLE products ADD COLUMN socbenefit INTEGER", &[]).expect("alter db add column");
        conn.execute("ALTER TABLE products ADD COLUMN socbenefittun INTEGER", &[]).expect("alter db add column");
        conn.execute("ALTER TABLE products ADD COLUMN enveffect INTEGER", &[]).expect("alter db add column");
        conn.execute("ALTER TABLE products ADD COLUMN enveffecttun INTEGER", &[]).expect("alter db add column");
        conn.execute("ALTER TABLE products ADD COLUMN humaneffect INTEGER", &[]).expect("alter db add column");
        conn.execute("ALTER TABLE products ADD COLUMN humaneffecttun INTEGER", &[]).expect("alter db add column");
        conn.execute("ALTER TABLE products ADD COLUMN envbenefit INTEGER", &[]).expect("alter db add column");

        conn.execute("PRAGMA user_version = 2", &[])
            .expect("alter db version");
        db_version = 2;
    }
    if db_version == 2 {
        upgrade_message(2);
        conn.execute("INSERT OR IGNORE INTO users (id, name, NBR, password, time_created)\
        VALUES (0, '-', 0, 0, datetime('now','localtime'))", &[])
            .expect("insert single entry into users table");

        conn.execute("ALTER TABLE transfers ADD COLUMN comment TEXT", &[]).expect("alter db add column");

        conn.execute("PRAGMA user_version = 3", &[])
            .expect("alter db version");
        db_version = 3;
    }
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

    tmpconn.execute("INSERT INTO users (name, NBR, password, time_created)\
    VALUES ($1, $2, $3, datetime('now', 'localtime'))",
                    &[&user.name, &0, &"0"])
        .expect("insert single entry into products table");

    Flash::success(Redirect::to("/"),
                   if templatedir.0 { "Uživatel přidán." }
                            else { "User added." })
}

#[get("/users")]
fn users(db_conn: State<DbConn>) -> Template {
    let tmpconn = db_conn.lock()
        .expect("db connection lock");
    let mut stmt = tmpconn
        .prepare("SELECT id, name, NBR, time_created FROM users WHERE id != 0")
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

#[get("/product")]
fn addproduct_page() -> Template {
    let product = Product {
        id: 0,
        name: String::new(),
        gateway: 0,
        benefit: 0,
        time_created: String::new(),
        resabundance: 1,
        resabundancetun: 1,
        prodpop: 1,
        consdem: 1,
        proddembalance: 1,
        conssubsat: 0,
        conssubsattun: 1,
        consobjben: 0,
        consobjbentun: 1,
        consbenefit: 1,
        socbenefit: 1,
        socbenefittun: 1,
        enveffect: 0,
        enveffecttun: 1,
        humaneffect: 0,
        humaneffecttun: 1,
        envbenefit: 1,
    };
    Template::render("addproduct", product)
}

#[get("/product/<product_id>")]
fn product_page(product_id: i64, db_conn: State<DbConn>) -> Template {
    let tmpconn = db_conn.lock()
        .expect("db connection lock");
    let product: Product = tmpconn.query_row("SELECT id, name, gateway,
    resabundance, resabundancetun, prodpop, consdem, proddembalance, conssubsat, conssubsattun,
    consobjben, consobjbentun, consbenefit, socbenefit, socbenefittun, enveffect, enveffecttun,
    humaneffect, humaneffecttun, envbenefit
    FROM products WHERE id = $1", &[&product_id],
    |row| {
        Product {
            id: row.get(0),
            name: row.get(1),
            gateway: row.get(2),
            benefit: 0,
            time_created: String::new(),
            resabundance: row.get_checked(3).unwrap_or(1),
            resabundancetun: row.get_checked(4).unwrap_or(1),
            prodpop: row.get_checked(5).unwrap_or(1),
            consdem: row.get_checked(6).unwrap_or(1),
            proddembalance: row.get_checked(7).unwrap_or(1),
            conssubsat: row.get_checked(8).unwrap_or(0),
            conssubsattun: row.get_checked(9).unwrap_or(1),
            consobjben: row.get_checked(10).unwrap_or(0),
            consobjbentun: row.get_checked(11).unwrap_or(1),
            consbenefit: row.get_checked(12).unwrap_or(1),
            socbenefit: row.get_checked(13).unwrap_or(1),
            socbenefittun: row.get_checked(14).unwrap_or(1),
            enveffect: row.get_checked(15).unwrap_or(0),
            enveffecttun: row.get_checked(16).unwrap_or(1),
            humaneffect: row.get_checked(17).unwrap_or(0),
            humaneffecttun: row.get_checked(18).unwrap_or(1),
            envbenefit: row.get_checked(19).unwrap_or(1),
        }
    }).expect("get product from db");

    Template::render("addproduct", product)

}

#[post("/product", data = "<product>")]
fn addproduct(product: Form<Product>, db_conn: State<DbConn>, templatedir: State<TemplateDir>) -> Flash<Redirect> {
    let tmpconn = db_conn.lock()
        .expect("db connection lock");

    let p = product.into_inner();

    let benefit = p.proddembalance * (p.resabundance / p.resabundancetun + p.consdem / p.prodpop)
        + p.consbenefit * (p.conssubsat / p.conssubsattun * p.consobjben / p.consobjbentun)
        + p.envbenefit * (p.socbenefit / p.socbenefittun + p.enveffect / p.enveffecttun + p.humaneffect / p.humaneffecttun);

    if p.id == 0 {
        tmpconn.execute("INSERT INTO products (name, gateway, benefit, time_created,
    resabundance, resabundancetun, prodpop, consdem, proddembalance, conssubsat, conssubsattun,
    consobjben, consobjbentun, consbenefit, socbenefit, socbenefittun, enveffect, enveffecttun,
    humaneffect, humaneffecttun, envbenefit)
    VALUES ($1, $2, $3, datetime('now', 'localtime'), $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20)",
                        &[&p.name, &p.gateway, &benefit,
                            &p.resabundance, &p.resabundancetun, &p.prodpop, &p.consdem, &p.proddembalance, &p.conssubsat, &p.conssubsattun,
                            &p.consobjben, &p.consobjbentun, &p.consbenefit, &p.socbenefit, &p.socbenefittun, &p.enveffect, &p.enveffecttun,
                            &p.humaneffect, &p.humaneffecttun, &p.envbenefit])
            .expect("insert single entry into products table");

        Flash::success(Redirect::to("/"),
                       if templatedir.0 { "Produkt přidán." } else { "Product added." })
    } else {
        tmpconn.execute("UPDATE products SET name = $1, gateway = $2, benefit = $3,
    resabundance = $4, resabundancetun = $5, prodpop = $6, consdem = $7, proddembalance = $8, conssubsat = $9, conssubsattun = $10,
    consobjben = $11, consobjbentun = $12, consbenefit = $13, socbenefit = $14, socbenefittun = $15, enveffect = $16, enveffecttun = $17,
    humaneffect = $18, humaneffecttun = $19, envbenefit = $20
    WHERE id = $21",
                        &[&p.name, &p.gateway, &benefit,
                            &p.resabundance, &p.resabundancetun, &p.prodpop, &p.consdem, &p.proddembalance, &p.conssubsat, &p.conssubsattun,
                            &p.consobjben, &p.consobjbentun, &p.consbenefit, &p.socbenefit, &p.socbenefittun, &p.enveffect, &p.enveffecttun,
                            &p.humaneffect, &p.humaneffecttun, &p.envbenefit, &p.id])
            .expect("update entry in products table");

        Flash::success(Redirect::to("/"),
                       if templatedir.0 { "Produkt upraven." } else { "Product modified." })
    }
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
            resabundance: 0,
            resabundancetun: 0,
            prodpop: 0,
            consdem: 0,
            proddembalance: 0,
            conssubsat: 0,
            conssubsattun: 0,
            consobjben: 0,
            consobjbentun: 0,
            consbenefit: 0,
            socbenefit: 0,
            socbenefittun: 0,
            enveffect: 0,
            enveffecttun: 0,
            humaneffect: 0,
            humaneffecttun: 0,
            envbenefit: 0,
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
        .prepare("SELECT id, name, NBR FROM users WHERE id != 0 ORDER BY name")
        .unwrap();
    {
        let user_iter = stmt.query_map(&[], |row| {
            User {
                id: row.get(0),
                name: row.get(1),
                nbr: row.get(2),
                time_created: String::new(),
            }
        }).unwrap();
        for user in user_iter {
            users.push(user.unwrap());
        }
    }
    stmt = tmpconn
        .prepare("SELECT id, name, gateway FROM products ORDER BY name")
        .unwrap();
    let item_iter = stmt.query_map(&[], |row| {
        User { //yes, this is on purpose to save data
            id: row.get(0),
            name: row.get(1),
            nbr: row.get(2),
            time_created: String::new(),
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
    comment: String,
}

#[post("/transfer", data = "<post>")]
fn transfer(conn: State<DbConn>, post: Form<Transfer>, templatedir: State<TemplateDir>) -> Flash<Redirect> {
    let transfer = post.into_inner();

    let tmpconn = conn.lock()
        .expect("db connection lock");

    let product_params:(i64, i64) = tmpconn.query_row("SELECT gateway, benefit FROM products WHERE id = $1",
                   &[&transfer.product], |row| { (row.get(0), row.get(1)) })
        .expect("product does not exist");

    let nbr: i64 = tmpconn.query_row("SELECT NBR FROM users WHERE id = $1",
                      &[&transfer.consumer], |row| { row.get(0) })
        .expect("get nbr for user");

    if nbr - product_params.0 * transfer.amount < 0 && transfer.consumer != 0 {
        return Flash::success(Redirect::to("/"),
                              if templatedir.0 { "Konzument je v mínusu! Transfer zamítnut." }
                                  else { "Consumer is in deficit! Transfer denied." })
    }

    if transfer.producer == transfer.consumer {
        return Flash::success(Redirect::to("/"),
                              if templatedir.0 { "Konzument a producent nesmí být stejná osoba! Transfer zamítnut." }
                                  else { "Consumer and producer must not be the same! Transfer denied." })
    }

    if transfer.consumer != 0 {
        tmpconn.execute("INSERT INTO transfers (ProducerID, ConsumerID, ProductID, amount, NBR, GNBR, comment, time_created)\
    VALUES ($1, $2, $3, $4, $5, $6, $7, datetime('now', 'localtime'))",
                        &[&transfer.producer, &transfer.consumer, &transfer.product, &transfer.amount,
                            &(product_params.1 * transfer.amount), &(product_params.0 * transfer.amount), if transfer.comment.is_empty() { &rusqlite::types::Null} else { &transfer.comment }])
            .expect("insert single entry into transfers table");
    } else {
        tmpconn.execute("INSERT INTO transfers (ProducerID, ConsumerID, ProductID, amount, NBR, GNBR, comment, time_created)\
    VALUES ($1, $2, $3, $4, $5, $6, $7, datetime('now', 'localtime'))",
                        &[&transfer.producer, &transfer.consumer, &transfer.product, &transfer.amount,
                            &(product_params.1 * transfer.amount), &0, if transfer.comment.is_empty() { &rusqlite::types::Null} else { &transfer.comment }])
            .expect("insert single entry into transfers table");
    }
    tmpconn.execute("UPDATE users SET NBR = NBR + $1 WHERE id = $2",
                    &[&(product_params.1 * transfer.amount), &transfer.producer])
        .expect("update producer entry in transfers table");
    if transfer.consumer != 0 {
        tmpconn.execute("UPDATE users SET NBR = NBR - $1 WHERE id = $2",
                        &[&(product_params.0 * transfer.amount), &transfer.consumer])
            .expect("update consumer entry in transfers table");
    }

    Flash::success(Redirect::to("/"),
                   if templatedir.0 { "Transfer proveden." } else { "Transfer complete." })

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
    gnbr: i64,
    comment: String,
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
        , t2.GNBR
        , t2.comment
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
                gnbr: row.get(9),
                comment: row.get_checked(10).unwrap_or(String::new()),
            }
        }).unwrap();
        for transfer in transfer_iter {
            transfers.push(transfer.unwrap());
        }
    }

    Template::render("transfers", &transfers)
}

#[get("/deletetransfer/<transfer_id>")]
fn delete_transfer(conn: State<DbConn>, transfer_id: i64, templatedir: State<TemplateDir>) -> Flash<Redirect> {

    let tmpconn = conn.lock()
        .expect("db connection lock");

    let transfer_params: (i64, i64, i64, i64) = tmpconn.query_row(
        "SELECT ProducerID, ConsumerID, NBR, GNBR FROM transfers WHERE id = $1",
        &[&transfer_id], |row| { (row.get(0), row.get(1), row.get(2), row.get(3)) })
        .expect("product does not exist");


    tmpconn.execute("DELETE FROM transfers WHERE id = $1",
                    &[&transfer_id])
        .expect("delete single entry from transfers table");
    tmpconn.execute("UPDATE users SET NBR = NBR - $1 WHERE id = $2",
                    &[&transfer_params.2, &transfer_params.0])
        .expect("update producer entry in transfers table");
    tmpconn.execute("UPDATE users SET NBR = NBR + $1 WHERE id = $2",
                    &[&transfer_params.3, &transfer_params.1])
        .expect("update producer entry in transfers table");

    Flash::success(Redirect::to("/"),
                   if templatedir.0 { "Transfer smazán." }
                       else { "Transfer deleted." })
}

struct TemplateDir(bool);

fn rocket() -> Rocket {
    // Open a new in-memory SQLite database.
    //let conn = Connection::open_in_memory().expect("in memory db");
    let conn = Connection::open("copiosis.sqlite").expect("db file");

    // Initialize the `entries` table in the database.
    init_database(&conn);

    let rct = rocket::ignite()
        .attach(Template::fairing())
        .attach(AdHoc::on_attach(|rocket| {
            println!("Adding token managed state from config...");
            let token_val = rocket.config().get_str("template_dir").unwrap_or("").to_string();
            Ok(rocket.manage(TemplateDir(token_val.ne(""))))
        }))
        .manage(Mutex::new(conn))
        .mount("/", routes![index, adduser_page, addproduct_page, addproduct, product_page, adduser,
        transfer_page, transfer, transfers, users, products, delete_transfer]);

    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    println!("Please open http://localhost:8000 in web browser.\n");

    rct
}

fn main() {
    rocket().launch();
}
