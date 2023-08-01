-- Your SQL goes here
create table vehicles (
    id serial primary key,
    name text not null,
    battery_capacity int not null,
    charge_limit float not null
);

create table charge_sessions (
    id serial primary key,
    date timestamp not null,
    vehicle_id int not null,
    end_soc int not null,
    energy float not null,
    odometer int not null
);