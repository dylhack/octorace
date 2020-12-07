-- Your SQL goes here
create schema if not exists octorace;

create table if not exists octorace.users
(
    discord_id bigint not null,
    contributions int default 0 not null,
    expires timestamp without time zone not null
);

create unique index if not exists users_discord_id_uindex
    on octorace.users (discord_id);

alter table octorace.users drop constraint if exists users_pk;

alter table octorace.users
    add constraint users_pk
        primary key (discord_id);

