use uuid::Uuid;

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