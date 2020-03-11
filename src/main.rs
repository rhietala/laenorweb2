#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket::Request;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::{Template, handlebars};
use rocket_contrib::json::Json;

use diesel::prelude::*;
use uuid::Uuid;

pub mod schema;
pub mod models;

use handlebars::{Helper, Handlebars, Context, RenderContext, Output, HelperResult, JsonRender};

#[derive(Serialize)]
struct TemplateContext {
    title: &'static str,
    items: Vec<&'static str>,
    // This key tells handlebars which template is the parent.
    parent: &'static str,
}

#[database("db")]
struct VoroveiDb(diesel::PgConnection);

#[get("/")]
fn index() -> Template {
    Template::render("index", &TemplateContext {
        title: "Hello",
        items: vec!["One", "Two", "Three"],
        parent: "layout",
    })
}

#[get("/taggroups")]
fn taggroups(conn: VoroveiDb) -> Json<Vec<models::TagGroup>> {
    use schema::taggroups::dsl::*;
    taggroups
        .load::<models::TagGroup>(&*conn)
        .map(|taggroup| Json(taggroup))
        .unwrap()
}

#[get("/taggroups/<myid>")]
fn taggroup_by_id(conn: VoroveiDb, myid: String) -> Json<models::TagGroup> {
    use schema::taggroups::dsl::*;
    taggroups
        .find(Uuid::parse_str(&myid).unwrap())
        .get_result::<models::TagGroup>(&*conn)
        .map(|taggroup| Json(taggroup))
        .unwrap()
}

#[catch(404)]
fn not_found(req: &Request) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}

fn wow_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output
) -> HelperResult {
    if let Some(param) = h.param(0) {
        out.write("<b><i>")?;
        out.write(&param.value().render())?;
        out.write("</b></i>")?;
    }

    Ok(())
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, taggroups, taggroup_by_id])
        .mount("/static", StaticFiles::from("static"))
        .register(catchers![not_found])
        .attach(VoroveiDb::fairing())
        .attach(Template::custom(|engines| {
            engines.handlebars.register_helper("wow", Box::new(wow_helper));
        }))
}

fn main() {
    rocket().launch();
}
