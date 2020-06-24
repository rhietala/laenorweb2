table! {
    notes (id) {
        id -> Int4,
        owner_id -> Int4,
    }
}

table! {
    notestags (note_id, tag_id) {
        note_id -> Int4,
        tag_id -> Int4,
    }
}

table! {
    notesusergroups (note_id, usergroup_id) {
        note_id -> Int4,
        usergroup_id -> Int4,
    }
}

table! {
    notesusers (note_id, user_id) {
        note_id -> Int4,
        user_id -> Int4,
    }
}

table! {
    notetexts (id) {
        id -> Int4,
        author_id -> Int4,
        note_id -> Int4,
        text -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

table! {
    taggroups (id) {
        id -> Int4,
        name -> Text,
        ordering -> Int2,
    }
}

table! {
    tags (id) {
        id -> Int4,
        name -> Text,
        map -> Nullable<Text>,
        taggroup_id -> Nullable<Int4>,
    }
}

table! {
    usergroups (id) {
        id -> Int4,
        name -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Text,
    }
}

table! {
    usersusergroups (user_id, usergroup_id) {
        user_id -> Int4,
        usergroup_id -> Int4,
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
