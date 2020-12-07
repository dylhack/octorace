-- Your SQL goes here
create table if not exists octorace.users
(
    discord_id bigint not null,
    contributions int default 0 not null,
    expires timestamp without time zone not null
);

create unique index if not exists users_discord_id_uindex
    on octorace.users (discord_id);

alter table octorace.users
    add constraint table_name_pk
        primary key (discord_id);

