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
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate rusqlite;

//#[cfg(test)] mod tests;

use std::sync::Mutex;
use rocket::Rocket;
use rusqlite::Connection;
#[macro_use] extern crate serde_derive;
use rocket_contrib::templates::Template;
use rocket::request::FlashMessage;
use rocket::fairing::AdHoc;

mod users;
//use users::*;
mod products;
//use products::*;
mod transfers;
//use transfers::*;
mod db;

type DbConn = Mutex<Connection>;
pub struct TemplateDir(bool);

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

fn rocket() -> Rocket {
    let conn = Connection::open("copiosis.sqlite").expect("db file");

    // Initialize the `entries` table in the database.
    db::init_database(&conn);

    let rct = rocket::ignite()
        .attach(Template::fairing())
        .attach(AdHoc::on_attach("template_dir",|rocket| {
            println!("Adding token managed state from config...");
            let token_val = rocket.config().get_str("template_dir").unwrap_or("").to_string();
            Ok(rocket.manage(TemplateDir(token_val.ne(""))))
        }))
        .manage(Mutex::new(conn))
        .mount("/", routes![index, users::adduser_page, products::addproduct_page, products::addproduct, products::product_page, users::adduser,
        transfers::transfer_page, transfers::transfer, transfers::transfers, users::users, products::products, transfers::delete_transfer]);

    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    println!("Please open http://localhost:8000 in web browser.\n");

    rct
}

fn main() {
    rocket().launch();
}
