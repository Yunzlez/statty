-- Your SQL goes here
create table vehicle (
    id serial primary key,
    name text not null
)
create table charge_session (
    id serial primary key,
    date timestamp not null,
    vehicle_id int not null,
    energy int not null,
    odometer int not null
)