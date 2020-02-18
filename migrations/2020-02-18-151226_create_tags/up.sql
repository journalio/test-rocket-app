create table tags
(
    id          uuid primary key not null default uuid_generate_v4(),
    title       text             not null,
    description text             not null,
    color       char(8)                   default 'FFFFFFFF',
    created_at  timestamp        not null default current_timestamp,
    created_by  uuid             not null references users (id),
    updated_at  timestamp        null
);

create table items_tags
(
    id      uuid primary key not null default uuid_generate_v4(),
    tag_id  uuid             not null references tags (id),
    item_id uuid             not null references items (id)
)