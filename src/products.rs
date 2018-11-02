//use std::sync::Mutex;
//use rusqlite::Connection;
use rocket::request::Form;
use rocket_contrib::templates::Template;
use rocket::State;
use rocket::response::Redirect;
//use rocket::request::FlashMessage;
use rocket::response::Flash;

use super::{DbConn,TemplateDir};

#[derive(FromForm, Serialize)]
pub struct Product {
    id: i64,
    name: String,
    gateway: f64,
    benefit: f64,
    time_created: String,
    resabundance: f64,
    consprodratio: f64,
    socimpact: f64,
    ccs: f64,
    conssubben: f64,
    cco: f64,
    consobjben: f64,
    ceb: f64,
    envben: f64,
    chb: f64,
    humanben: f64,
}

#[get("/product")]
pub fn addproduct_page() -> Template {
    let product = Product {
        id: 0,
        name: String::new(),
        gateway: 0.0,
        benefit: 0.0,
        time_created: String::new(),
        resabundance: 1.0,
        consprodratio: 1.0,
        socimpact: 1.0,
        ccs: 1.0,
        conssubben: 0.0,
        cco: 1.0,
        consobjben: 0.0,
        ceb: 1.0,
        envben: 0.0,
        chb: 1.0,
        humanben: 0.0,
    };
    Template::render("addproduct", product)
}

#[get("/product/<product_id>")]
pub fn product_page(product_id: i64, db_conn: State<DbConn>) -> Template {
    let tmpconn = db_conn.lock()
        .expect("db connection lock");
    let product: Product = tmpconn.query_row("SELECT id, name, gateway,
    resabundance, consprodratio, socimpact, ccs, conssubben, cco, consobjben,
    ceb, envben, chb, humanben
    FROM products WHERE id = $1", &[&product_id],
                                             |row| {
                                                 Product {
                                                     id: row.get(0),
                                                     name: row.get(1),
                                                     gateway: row.get(2),
                                                     benefit: 0.0,
                                                     time_created: String::new(),
                                                     resabundance: row.get_checked(3).unwrap_or(1.0),
                                                     consprodratio: row.get_checked(4).unwrap_or(1.0),
                                                     socimpact: row.get_checked(5).unwrap_or(1.0),
                                                     ccs: row.get_checked(6).unwrap_or(1.0),
                                                     conssubben: row.get_checked(7).unwrap_or(0.0),
                                                     cco: row.get_checked(8).unwrap_or(1.0),
                                                     consobjben: row.get_checked(9).unwrap_or(0.0),
                                                     ceb: row.get_checked(10).unwrap_or(1.0),
                                                     envben: row.get_checked(11).unwrap_or(0.0),
                                                     chb: row.get_checked(12).unwrap_or(1.0),
                                                     humanben: row.get_checked(13).unwrap_or(0.0),
                                                 }
                                             }).expect("get product from db");

    Template::render("addproduct", product)

}

#[post("/product", data = "<product>")]
pub fn addproduct(product: Form<Product>, db_conn: State<DbConn>, templatedir: State<TemplateDir>) -> Flash<Redirect> {
    let tmpconn = db_conn.lock()
        .expect("db connection lock");

    let p = product.into_inner();

    if p.gateway < 0.0 {
        return Flash::success(Redirect::to("/"),
                              if templatedir.0 { "Error: Brána nesmí být nikdy záporná!" } else { "Error: Gateway must never be negative!" })
    }

    let benefit = p.resabundance * p.consprodratio * (1.0 + p.socimpact).ln() *
        (
            p.ccs * p.conssubben + p.cco * p.consobjben +
            p.ceb * p.envben + p.chb * p.humanben
        );

    if p.id == 0 {
        tmpconn.execute("INSERT INTO products (name, gateway, benefit, time_created,
    resabundance, consprodratio, socimpact, ccs, conssubben, cco, consobjben,
    ceb, envben, chb, humanben)
    VALUES ($1, $2, $3, datetime('now', 'localtime'), $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)",
                        &[&p.name, &p.gateway, &benefit,
                            &p.resabundance, &p.consprodratio, &p.socimpact, &p.ccs, &p.conssubben, &p.cco, &p.consobjben,
                            &p.ceb, &p.envben, &p.chb, &p.humanben])
            .expect("insert single entry into products table");

        Flash::success(Redirect::to("/"),
                       if templatedir.0 { "Produkt přidán." } else { "Product added." })
    } else {
        tmpconn.execute("UPDATE products SET name = $1, gateway = $2, benefit = $3,
    resabundance = $4, consprodratio = $5, socimpact = $6, ccs = $7, conssubben = $8, cco = $9, consobjben = $10,
    ceb = $11, envben = $12, chb = $13, humanben = $14
    WHERE id = $15",
                        &[&p.name, &p.gateway, &benefit,
                            &p.resabundance, &p.consprodratio, &p.socimpact, &p.ccs, &p.conssubben, &p.cco, &p.consobjben,
                            &p.ceb, &p.envben, &p.chb, &p.humanben, &p.id])
            .expect("update entry in products table");

        Flash::success(Redirect::to("/"),
                       if templatedir.0 { "Produkt upraven." } else { "Product modified." })
    }
}

#[get("/products")]
pub fn products(db_conn: State<DbConn>) -> Template {
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
            resabundance: 0.0,
            consprodratio: 0.0,
            socimpact: 0.0,
            ccs: 0.0,
            conssubben: 0.0,
            cco: 0.0,
            consobjben: 0.0,
            ceb: 0.0,
            envben: 0.0,
            chb: 0.0,
            humanben: 0.0,
        }
    }).unwrap();

    let mut vct = Vec::new();
    for product in product_iter {
        vct.push(product.unwrap());
    }

    Template::render("products", vct)
}