//use std::sync::Mutex;
use rocket_sync_db_pools::rusqlite::params;
use rocket::form::Form;
use rocket_dyn_templates::Template;
use rocket::State;
use rocket::response::Redirect;
//use rocket::request::FlashMessage;
use rocket::response::Flash;
use rocket::serde::Serialize;

use super::{DbConn,TemplateDir};
use crate::products::Product;

#[derive(FromForm, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i64,
    pub name: String,
    pub nbr: f64,
    pub fame: f64,
    //password: String,
    pub time_created: String,
}

#[derive(FromForm)]
pub struct UserProduct {
    user_id: i64,
    product_id: i64,
    //name: String,
}

#[get("/adduser")]
pub fn adduser_page() -> Template {
    Template::render("adduser", User {
        id: 0,
        name: String::new(),
        nbr: 1.0,
        fame: 0.0,
        time_created: String::new()
    })
}

#[post("/adduser", data = "<user>")]
pub async fn adduser(user: Form<User>, db_conn: &State<DbConn>, templatedir: &State<TemplateDir>) -> Flash<Redirect> {
    let user = user.into_inner();
    let tmpconn = db_conn.lock().await;

    tmpconn.execute("INSERT INTO users (name, NBR, password, time_created)\
    VALUES ($1, $2, $3, datetime('now', 'localtime'))",
                    params![&user.name, &0, &"0"])
        .expect("insert single entry into products table");

    Flash::success(Redirect::to("/"),
                   if templatedir.0 { "Uživatel přidán." }
                       else { "User added." })
}

#[get("/users")]
pub async fn users(db_conn: &State<DbConn>) -> Template {

    #[derive(Serialize)]
    #[serde(crate = "rocket::serde")]
    struct UsersPage {
        users: Vec<User>,
        products: Vec<User>
    }

    let tmpconn = db_conn.lock().await;
    let mut stmt = tmpconn
        .prepare("SELECT id, name, NBR, time_created, fame FROM users WHERE id != 0 ORDER BY name")
        .unwrap();

    let user_iter = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            nbr: row.get(2)?,
            fame: row.get(4)?,
            time_created: row.get(3)?,
        })
    }).unwrap();

    let mut users = Vec::new();
    for user in user_iter {
        users.push(user.unwrap());
    }

    let mut stmt = tmpconn
        .prepare("SELECT id, name FROM products WHERE id != 0 ORDER BY name")
        .unwrap();
    let product_iter = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            nbr: 0.0,
            fame: 0.0,
            time_created: String::new(),
        })
    }).unwrap();

    let mut products = Vec::new();
    for product in product_iter {
        products.push(product.unwrap());
    }

    Template::render("users", UsersPage { users, products})
}

#[post("/user/product", data = "<userproduct>")]
pub async fn product_page(userproduct: Form<UserProduct>, db_conn: &State<DbConn>) -> Template {
    let p = userproduct.into_inner();
    let tmpconn = db_conn.lock().await;
    let product: Product = tmpconn.query_row("SELECT gateway,
    resabundance, beneficiaries, producers, ccs, conssubben, cco, consobjben,
    ceb, envben, chb, humanben
    FROM user_products WHERE ProductID = $1 AND UserID = $2", [&p.product_id, &p.user_id],
                                             |row| {
                                                 Ok(Product {
                                                     id: p.product_id,
                                                     name: String::new(),
                                                     gateway: row.get(0)?,
                                                     benefit: 0.0,
                                                     time_created: String::new(),
                                                     resabundance: row.get(1)?,
                                                     beneficiaries: row.get(2)?,
                                                     producers: row.get(3)?,
                                                     ccs: row.get(4)?,
                                                     conssubben: row.get(5)?,
                                                     cco: row.get(6)?,
                                                     consobjben: row.get(7)?,
                                                     ceb: row.get(8)?,
                                                     envben: row.get(9)?,
                                                     chb: row.get(10)?,
                                                     humanben: row.get(11)?,
                                                     user_id: p.user_id,
                                                 })
                                             }).unwrap_or(Product {
        id: p.product_id,
        name: String::new(),
        gateway: 0.0,
        benefit: 0.0,
        time_created: String::new(),
        resabundance: 1.0,
        beneficiaries: 1.0,
        producers: 1.0,
        ccs: 1.0,
        conssubben: 0.0,
        cco: 1.0,
        consobjben: 0.0,
        ceb: 1.0,
        envben: 0.0,
        chb: 1.0,
        humanben: 0.0,
        user_id: p.user_id,
    });

    Template::render("adduserproduct", product)
}

#[post("/user/addproduct", data = "<product>")]
pub async fn addproduct(product: Form<Product>, db_conn: &State<DbConn>, templatedir: &State<TemplateDir>) -> Flash<Redirect> {
    let tmpconn = db_conn.lock().await;

    let p = product.into_inner();

    if p.gateway < 0.0 {
        return Flash::success(Redirect::to("/"),
                              if templatedir.0 { "Error: Brána nesmí být nikdy záporná!" } else { "Error: Gateway must never be negative!" })
    }

    let benefit = p.resabundance * (1.0 + p.beneficiaries / p.producers).ln() *
        (
            p.ccs * p.conssubben + p.cco * p.consobjben +
                p.ceb * p.envben + p.chb * p.humanben
        );

    let update_result = tmpconn.execute("UPDATE user_products SET UserID = $1, gateway = $2, benefit = $3,
    resabundance = $4, beneficiaries = $5, producers = $6, ccs = $7, conssubben = $8, cco = $9, consobjben = $10,
    ceb = $11, envben = $12, chb = $13, humanben = $14, ProductID = $15
    WHERE ProductID = $15 AND UserID = $1",
                        params![&p.user_id, &p.gateway, &benefit,
                            &p.resabundance, &p.beneficiaries, &p.producers, &p.ccs, &p.conssubben, &p.cco, &p.consobjben,
                            &p.ceb, &p.envben, &p.chb, &p.humanben, &p.id]);
    if update_result.is_ok() && update_result.unwrap() == 1 {
        return Flash::success(Redirect::to("/"),
                       if templatedir.0 { "Produkt upraven." } else { "Product modified." })
    }

    tmpconn.execute("INSERT INTO user_products (UserID, gateway, benefit, time_created,
    resabundance, beneficiaries, producers, ccs, conssubben, cco, consobjben,
    ceb, envben, chb, humanben, ProductID)
    VALUES ($1, $2, $3, datetime('now', 'localtime'), $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)",
                        params![&p.user_id, &p.gateway, &benefit,
                            &p.resabundance, &p.beneficiaries, &p.producers, &p.ccs, &p.conssubben, &p.cco, &p.consobjben,
                            &p.ceb, &p.envben, &p.chb, &p.humanben, &p.id])
            .expect("insert single entry into products table");

        Flash::success(Redirect::to("/"),
                       if templatedir.0 { "Produkt přidán." } else { "Product added." })

}

#[get("/fame")]
pub async fn fame(db_conn: &State<DbConn>) -> Template {
    let tmpconn = db_conn.lock().await;
    let mut stmt = tmpconn
        .prepare("SELECT id, name, NBR, time_created, fame FROM users WHERE id != 0 ORDER BY fame DESC")
        .unwrap();

    let user_iter = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            nbr: row.get(2)?,
            fame: row.get(4)?,
            time_created: row.get(3)?,
        })
    }).unwrap();

    let users: Vec<User> = user_iter.map(|x| x.unwrap()).collect();

    Template::render("fame", users)
}
