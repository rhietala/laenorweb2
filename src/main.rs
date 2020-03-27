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
use chrono::NaiveDateTime;

pub mod schema;
pub mod models;

use handlebars::{Helper, Handlebars, Context, RenderContext, Output, HelperResult, JsonRender};

#[database("db")]
struct Db(diesel::PgConnection);

#[derive(Serialize)]
struct IndexTemplateContext {
    title: &'static str,
    taggroups: Vec<(models::TagGroup, Vec<models::Tag>)>,
    // This key tells handlebars which template is the parent.
    highlights: Vec<models::Tag>,
    parent: &'static str,
}

#[get("/")]
fn index(conn: Db) -> Template {
    use schema::taggroups::dsl::*;
    use schema::tags::dsl::*;

    let _userid = "9e2474d1-5b4e-5a13-ad6d-5022a44f51d9";

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

    let highlights: Vec<models::Tag> = tags
        .limit(6)
        .load(&*conn)
        .unwrap();

    Template::render("index", &IndexTemplateContext {
        title: "Hello",
        taggroups: tgs_tags,
        parent: "layout",
        highlights: highlights
    })
}

#[derive(Serialize)]
struct TagTemplateContext {
    tag: models::Tag,
    notescount: usize,
    notes: Vec<TagTemplateNote>,
    parent: &'static str,
}

#[derive(Serialize)]
struct TagTemplateNote {
    note: models::Note,
    owner: models::User,
    tags: Vec<models::Tag>,
    texts: Vec<(models::NoteText, models::User)>,
    shared_with: Vec<models::User>,
    current_text: models::NoteText,
    updated_at: NaiveDateTime,
    title: String,
}

#[get("/tags/<tag_id_param>")]
fn tag(conn: Db, tag_id_param: String) -> Template {

    let tag_id_uuid = Uuid::parse_str(&tag_id_param).unwrap();
    let user_id_uuid = Uuid::parse_str("9e2474d1-5b4e-5a13-ad6d-5022a44f51d9").unwrap();

    let tag = {
        use schema::tags::dsl::*;
        tags.find(tag_id_uuid).first(&*conn).unwrap()
    };

    let tag_notes: Vec<TagTemplateNote> = {
        use schema::notes::dsl::*;
        use schema::notestags::dsl::*;
        use schema::notesusers::dsl::*;
        use schema::users::dsl::*;

        let raw_notes: Vec<(
            models::Note,
            models::NoteTag,
            models::User, // owner of note, for information
            models::NoteUser // note shared to someone
        )> = notes
            .inner_join(notestags)
            .inner_join(users)
            .inner_join(notesusers)
            .filter(tag_id.eq(tag_id_uuid))
            .filter(user_id.eq(user_id_uuid))
            .load(&*conn)
            .unwrap();

        let notes_with_texts = raw_notes
            .into_iter()
            .map(|r| {
                let tags: Vec<models::Tag> = {
                    use schema::notestags::dsl::*;
                    use schema::tags::dsl::*;
                    notestags
                        .inner_join(tags)
                        .filter(note_id.eq(r.1.note_id))
                        .load(&*conn)
                        .unwrap()
                        .into_iter()
                        .map(|nt: (models::NoteTag, models::Tag)| nt.1)
                        .collect::<Vec<models::Tag>>()
                };

                let notetexts: Vec<(models::NoteText, models::User)> = {
                    use schema::notetexts::dsl::*;
                    use schema::users::dsl::*;
                    notetexts
                        .inner_join(users)
                        .filter(note_id.eq(r.0.id))
                        .load(&*conn)
                        .unwrap()
                };

                let all_tags = tags
                    .iter()
                    .map(|t| t.name.clone())
                    .collect::<Vec<String>>()
                    .join(", ");

                let title = format!("{} ({})", all_tags, r.2.name);

                let current_text = notetexts.last().unwrap().0.clone();
                let updated_at = notetexts.last().unwrap().0.created_at.clone();

                let shared_with = {
                    use schema::notesusers::dsl::*;
                    use schema::users::dsl::*;

                    users
                        .inner_join(notesusers)
                        .filter(note_id.eq(r.0.id))
                        .load(&*conn)
                        .unwrap()
                        .into_iter()
                        .map(|x: (models::User, models::NoteUser)| x.0)
                        .collect()
                };

                TagTemplateNote {
                    note: r.0,
                    owner: r.2,
                    tags: tags,
                    texts: notetexts,
                    current_text: current_text,
                    shared_with: shared_with,
                    updated_at: updated_at,
                    title: title,
                }
            })
            .collect();

        notes_with_texts
    };

    Template::render("tag", &TagTemplateContext {
        tag: tag,
        notescount: tag_notes.len(),
        notes: tag_notes,
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

fn date_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output
) -> HelperResult {
    if let Some(param) = h.param(0) {
        out.write(
            &param
                .value()
                .render()
                .get(0..10)
                .unwrap()
        ).unwrap();
    }

    Ok(())
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
        .mount("/", routes![index, tag, taggroups, taggroup_by_id])
        .mount("/static", StaticFiles::from("static"))
        .register(catchers![not_found])
        .attach(Db::fairing())
        .attach(Template::custom(|engines| {
            engines.handlebars.register_helper("date", Box::new(date_helper));
            engines.handlebars.register_helper("color_map", Box::new(color_map_helper));
        }))
}

fn main() {
    rocket().launch();
}
