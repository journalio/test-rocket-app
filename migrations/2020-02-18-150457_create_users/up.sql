create table users (
    id uuid not null primary key default uuid_generate_v4(),
    full_name text not null,
    email text unique not null,
    password_hash text not null
);