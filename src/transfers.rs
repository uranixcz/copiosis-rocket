//use std::sync::Mutex;
//use rusqlite::Connection;
use rocket::request::Form;
use rocket_contrib::templates::Template;
use rocket::State;
use rocket::response::Redirect;
//use rocket::request::FlashMessage;
use rocket::response::Flash;

use crate::users::User;
use super::{DbConn,TemplateDir};

#[derive(Serialize)]
struct ContextTransfer {
    users: Vec<User>,
    products: Vec<User>,
}

#[derive(FromForm, Serialize)]
pub struct Transfer {
    producer: i64,
    consumer: i64,
    product: i64,
    amount: f64,
    comment: String,
}

#[derive(Serialize)]
struct NamedTransfer {
    id: i64,
    producer: String,
    consumer: String,
    product: String,
    amount: f64,
    nbr: f64,
    time_created: String,
    gnbr: f64,
    comment: String,
}

#[get("/transfer")]
pub fn transfer_page(conn: State<DbConn> ) -> Template {
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

#[post("/transfer", data = "<post>")]
pub fn transfer(conn: State<DbConn>, post: Form<Transfer>, templatedir: State<TemplateDir>) -> Flash<Redirect> {
    let transfer = post.into_inner();

    let tmpconn = conn.lock()
        .expect("db connection lock");

    let product_query = tmpconn.query_row("SELECT gateway, benefit FROM user_products WHERE ProductID = $1 AND UserID = $2",
                                          &[&transfer.product, &transfer.producer], |row| { (row.get(0), row.get(1)) });
    if product_query.is_err() {
        return Flash::success(Redirect::to("/"),
                       if templatedir.0 { "Produkt musí být nejprve uživateli přiřazen." } else { "Product must be assigned to the user first." })
    }
    let product_params:(f64, f64) = product_query.unwrap();

    let nbr: f64 = tmpconn.query_row("SELECT NBR FROM users WHERE id = $1",
                                     &[&transfer.consumer], |row| { row.get(0) })
        .expect("get nbr for user");

    if nbr - product_params.0 * transfer.amount < 0.0 && transfer.consumer != 0 {
        return Flash::success(Redirect::to("/"),
                              if templatedir.0 { "Konzument nemá dostatek NBR." }
                                  else { "Consumer has insufficient NBR." })
    }

    if transfer.producer == transfer.consumer {
        return Flash::success(Redirect::to("/"),
                              if templatedir.0 { "Konzument a producent nesmí být stejná osoba." }
                                  else { "Consumer and producer must not be the same." })
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

#[get("/transfers")]
pub fn transfers(conn: State<DbConn>) -> Template {
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
        ORDER BY t2.time_created DESC LIMIT 30;")
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
pub fn delete_transfer(conn: State<DbConn>, transfer_id: i64, templatedir: State<TemplateDir>) -> Flash<Redirect> {

    let tmpconn = conn.lock()
        .expect("db connection lock");

    let transfer_params: (i64, i64, f64, f64) = tmpconn.query_row(
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