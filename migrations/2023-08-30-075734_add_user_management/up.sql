-- Your SQL goes here
create table users(
    id serial primary key,
    username text not null,
    password text not null
);

alter table vehicles add column user_id int not null default -1;
create index vehicles_user_id_idx on vehicles(user_id);

create table oauth_clients(
    id serial primary key,
    client_id text not null,
    client_secret text not null,
    redirect_uris text not null
);