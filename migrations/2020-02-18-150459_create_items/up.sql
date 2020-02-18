create table items
(
    id         uuid primary key not null default uuid_generate_v4(),
    title      text             not null,
    item_type  text             not null,
    created_at timestamp        not null default current_timestamp,
    created_by uuid             not null references users (id),
    updated_at timestamp        null,
    owned_by   uuid             null references items (id)
);

create table todos
(
    id      uuid primary key not null references items (id),
    content text             not null default '',
    done    bool             not null default false
);