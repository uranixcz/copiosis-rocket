/*
* Copyright 2018-2019 Michal Mauser
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

#[macro_use] extern crate rocket;

//#[cfg(test)] mod tests;

use rocket::tokio::sync::Mutex;
use rocket_sync_db_pools::rusqlite::Connection;
use rocket::serde::Deserialize;
use rocket_dyn_templates::Template;
use rocket::request::FlashMessage;
use rocket::fairing::AdHoc;
#[cfg(feature = "gui")]
use webbrowser;
use rocket::figment;

mod users;
//use users::*;
mod products;
//use products::*;
mod transfers;
//use transfers::*;
mod db;

pub type DbConn = Mutex<Connection>;
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TemplateDir(bool);

#[get("/")]
fn index(flash: Option<FlashMessage>) -> Template {
    match flash {
        Some(x) => Template::render("index", x.message()),
        _ => Template::render("index", "")
    }
    /*let mut context = HashMap::new();
    if let Some(ref msg) = flash {
        context.insert("flash", msg.msg());
    }

    Template::render("login", &context)*/
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let conn = Connection::open("copiosis.sqlite").expect("db file");

    // Initialize the `entries` table in the database.
    db::init_database(&conn);

    //let rct = rocket::ignite()
    let mut rct = rocket::build()
        .attach(Template::fairing())
        .manage(Mutex::new(conn))
        .mount("/", routes![index, users::adduser_page, products::addproduct_page, products::addproduct, products::product_page, users::adduser,
        transfers::transfer_page, transfers::transfer, transfers::transfers, users::users, products::products, transfers::delete_transfer,
        users::addproduct, users::product_page, products::product_producers, users::fame]);

    #[cfg(feature = "gui")] {
        rct = rct.attach(AdHoc::on_liftoff("Liftoff Printer", |_| Box::pin(async move {
            println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
            println!("Please open http://localhost:8000 in web browser.\n");
            webbrowser::open("http://localhost:8000").ok();
        })))
    }

    let conf: Result<Vec<String>, figment::Error> = rct.figment().extract_inner("template_dir");
    rct.manage(TemplateDir(if let Ok(dir) = conf {!dir.is_empty()} else {false}))
        .launch().await?;
    Ok(())
}
