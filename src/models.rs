use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Deserialize, Queryable, Serialize, Clone, PartialEq)]
pub struct TagGroup {
    pub id: Uuid,
    pub name: String,
    pub ordering: i16,
}

#[derive(Deserialize, Queryable, Serialize, Clone, PartialEq)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
    pub map: Option<String>,
    pub taggroup_id: Option<Uuid>,
}

#[derive(Deserialize, Queryable, Serialize, Clone, PartialEq)]
pub struct Note {
    pub id: Uuid,
    pub owner_id: Uuid,
}

#[derive(Deserialize, Queryable, Serialize, Clone, PartialEq)]
pub struct NoteTag {
    pub note_id: Uuid,
    pub tag_id: Uuid,
}

#[derive(Deserialize, Queryable, Serialize, Clone, PartialEq)]
pub struct NoteText {
    pub id: Uuid,
    pub author_id: Uuid,
    pub note_id: Uuid,
    pub text: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize, Queryable, Serialize, Clone, PartialEq)]
pub struct NoteUser {
    pub note_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Deserialize, Queryable, Serialize, Clone, PartialEq)]
pub struct User {
    pub id: Uuid,
    pub name: String,
}

#[derive(Deserialize, Queryable, Serialize, Clone, PartialEq)]
pub struct UserGroup {
    pub id: Uuid,
    pub name: String,
}
