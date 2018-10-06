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
pub struct User {
    pub id: i64,
    pub name: String,
    pub nbr: f64,
    //password: String,
    pub time_created: String,
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
        .prepare("SELECT id, name, NBR, time_created FROM users WHERE id != 0 ORDER BY name")
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