//use std::sync::Mutex;
//use rusqlite::Connection;
use rocket::request::Form;
use rocket_contrib::Template;
use rocket::State;
use rocket::response::Redirect;
//use rocket::request::FlashMessage;
use rocket::response::Flash;

use super::{DbConn,TemplateDir};

#[derive(FromForm, Serialize)]
pub struct Product {
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

    if p.gateway < 0 {
        return Flash::success(Redirect::to("/"),
                              if templatedir.0 { "Error: Brána nesmí být nikdy záporná!" } else { "Error: Gateway must never be negative!" })
    }

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
        .prepare("SELECT id, name, gateway, benefit, time_created FROM products ORDER BY name")
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