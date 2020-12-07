-- Your SQL goes here
create table if not exists octorace.connections
(
    discord_id bigint not null
        constraint connections_users_discord_id_fk
            references octorace.users,
    github text not null,
    expires timestamp without time zone not null
);

create unique index if not exists connections_discord_id_uindex
    on octorace.connections (discord_id);

create unique index if not exists connections_github_uindex
    on octorace.connections (github);