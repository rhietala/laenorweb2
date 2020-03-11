table! {
    notes (id) {
        id -> Uuid,
        owner_id -> Uuid,
    }
}

table! {
    notestags (note_id, tag_id) {
        note_id -> Uuid,
        tag_id -> Uuid,
    }
}

table! {
    notesusergroups (note_id, usergroup_id) {
        note_id -> Uuid,
        usergroup_id -> Uuid,
    }
}

table! {
    notesusers (note_id, user_id) {
        note_id -> Uuid,
        user_id -> Uuid,
    }
}

table! {
    notetexts (id) {
        id -> Uuid,
        author_id -> Uuid,
        note_id -> Uuid,
        text -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

table! {
    taggroups (id) {
        id -> Uuid,
        name -> Text,
        ordering -> Int2,
    }
}

table! {
    tags (id) {
        id -> Uuid,
        name -> Text,
        map -> Nullable<Text>,
        taggroup_id -> Nullable<Uuid>,
    }
}

table! {
    usergroups (id) {
        id -> Uuid,
        name -> Text,
    }
}

table! {
    users (id) {
        id -> Uuid,
        name -> Text,
    }
}

table! {
    usersusergroups (user_id, usergroup_id) {
        user_id -> Uuid,
        usergroup_id -> Uuid,
    }
}

joinable!(notes -> users (owner_id));
joinable!(notestags -> notes (note_id));
joinable!(notestags -> tags (tag_id));
joinable!(notesusergroups -> notes (note_id));
joinable!(notesusergroups -> usergroups (usergroup_id));
joinable!(notesusers -> notes (note_id));
joinable!(notesusers -> users (user_id));
joinable!(notetexts -> notes (note_id));
joinable!(notetexts -> users (author_id));
joinable!(tags -> taggroups (taggroup_id));
joinable!(usersusergroups -> usergroups (usergroup_id));
joinable!(usersusergroups -> users (user_id));

allow_tables_to_appear_in_same_query!(
    notes,
    notestags,
    notesusergroups,
    notesusers,
    notetexts,
    taggroups,
    tags,
    usergroups,
    users,
    usersusergroups,
);
