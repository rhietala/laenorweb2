use uuid::Uuid;

#[derive(Deserialize, Queryable, Serialize)]
pub struct TagGroup {
    pub id: Uuid,
    pub name: String,
    pub ordering: i16,
}
