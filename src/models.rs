use chrono::NaiveDateTime;

#[derive(Deserialize, Queryable, Serialize, Clone, PartialEq)]
pub struct TagGroup {
    pub id: i32,
    pub name: String,
    pub ordering: i16,
}

#[derive(Deserialize, Queryable, Serialize, Clone, PartialEq)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub map: Option<String>,
    pub taggroup_id: Option<i32>,
}

#[derive(Deserialize, Queryable, Serialize, Clone, PartialEq)]
pub struct Note {
    pub id: i32,
    pub owner_id: i32,
}

#[derive(Deserialize, Queryable, Serialize, Clone, PartialEq)]
pub struct NoteTag {
    pub note_id: i32,
    pub tag_id: i32,
}

#[derive(Deserialize, Queryable, Serialize, Clone, PartialEq)]
pub struct NoteText {
    pub id: i32,
    pub author_id: i32,
    pub note_id: i32,
    pub text: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize, Queryable, Serialize, Clone, PartialEq)]
pub struct NoteUser {
    pub note_id: i32,
    pub user_id: i32,
}

#[derive(Deserialize, Queryable, Serialize, Clone, PartialEq)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Deserialize, Queryable, Serialize, Clone, PartialEq)]
pub struct UserGroup {
    pub id: i32,
    pub name: String,
}
