table! {
    items (id) {
        id -> Uuid,
        title -> Text,
        item_type -> Text,
        created_at -> Timestamp,
        created_by -> Uuid,
        updated_at -> Nullable<Timestamp>,
        owned_by -> Nullable<Uuid>,
    }
}

table! {
    items_tags (id) {
        id -> Uuid,
        tag_id -> Uuid,
        item_id -> Uuid,
    }
}

table! {
    tags (id) {
        id -> Uuid,
        title -> Text,
        description -> Text,
        color -> Nullable<Bpchar>,
        created_at -> Timestamp,
        created_by -> Uuid,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    todos (id) {
        id -> Uuid,
        content -> Text,
        done -> Bool,
    }
}

table! {
    users (id) {
        id -> Uuid,
        full_name -> Text,
        email -> Text,
        password_hash -> Text,
    }
}

joinable!(items -> users (created_by));
joinable!(items_tags -> items (item_id));
joinable!(items_tags -> tags (tag_id));
joinable!(tags -> users (created_by));
joinable!(todos -> items (id));

allow_tables_to_appear_in_same_query!(
    items,
    items_tags,
    tags,
    todos,
    users,
);
