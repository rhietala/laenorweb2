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
struct IndexTemplateContext {
    title: &'static str,
    taggroups: Vec<(models::TagGroup, Vec<models::Tag>)>,
    // This key tells handlebars which template is the parent.
    parent: &'static str,
}

#[database("db")]
struct Db(diesel::PgConnection);

#[get("/")]
fn index(conn: Db) -> Template {
    use schema::taggroups::dsl::*;
    use schema::tags::dsl::*;
    let taggroups_and_tags: Vec<(models::TagGroup, models::Tag)> =
        taggroups.inner_join(tags).load(&*conn).unwrap();

    let mut tgs: Vec<models::TagGroup> =
        taggroups_and_tags.iter().map(|x| x.clone().0).collect();

    tgs.dedup();

    let tgs_tags: Vec<(models::TagGroup, Vec<models::Tag>)> = tgs
        .iter()
        .map(|tg| (
            tg.clone(),
            taggroups_and_tags.iter()
                .filter_map(|x| if &x.0 == tg { Some(x.clone().1) } else { None })
                .collect()
        ))
        .collect();

    Template::render("index", &IndexTemplateContext {
        title: "Hello",
        taggroups: tgs_tags,
        parent: "layout",
    })
}

#[get("/taggroups")]
fn taggroups(conn: Db) -> Json<Vec<models::TagGroup>> {
    use schema::taggroups::dsl::*;
    taggroups
        .load::<models::TagGroup>(&*conn)
        .map(|taggroup| Json(taggroup))
        .unwrap()
}

#[get("/taggroups/<myid>")]
fn taggroup_by_id(conn: Db, myid: String) -> Json<models::TagGroup> {
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

fn color_map_char(c: char) -> String {
    let color = match c {
        '#' | '+' | '-' |
        '=' | 'C' | 'c' |
        '|' | '/' | '\\'      => Some("light_black"),
        '!' | '?' | '%' | 't' => Some("white"),
        'y' | 'z' | 'b' | 'd' => Some("yellow"),
        ','                   => Some("light_yellow"),
        '.' | 'F' | 'v' | 'j' => Some("green"),
        'f'                   => Some("light_green"),
        'V'                   => Some("red"),
        '@' | 'L' | 'x' | 's' => Some("light_red"),
        'H' | 'h'             => Some("magenta"),
        '^'                   => Some("light_magenta"),
        '~'                   => Some("blue"),
        'R' | 'r' | 'i' | 'l' => Some("light_blue"),
        'S' | 'w'             => Some("cyan"),
        _                     => None,
    };

    match color {
        Some(col) => format!("<span class=\"{}\">{}</span>", col, c),
        None => format!("{}", c),
    }
}

fn color_map_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output
) -> HelperResult {
    if let Some(param) = h.param(0) {
        &param
            .value()
            .render()
            .chars()
            .for_each(|c| out.write(&color_map_char(c)).unwrap());
    }

    Ok(())
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, taggroups, taggroup_by_id])
        .mount("/static", StaticFiles::from("static"))
        .register(catchers![not_found])
        .attach(Db::fairing())
        .attach(Template::custom(|engines| {
            engines.handlebars.register_helper("wow", Box::new(wow_helper));
            engines.handlebars.register_helper("color_map", Box::new(color_map_helper));
        }))
}

fn main() {
    rocket().launch();
}
