-- This file should undo anything in `up.sql`
drop table if exists users;
drop index if exists vehicles_user_id_idx;
alter table vehicles drop column user_id;

drop table if exists oauth_clients;