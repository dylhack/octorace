-- Your SQL goes here
create table if not exists octorace.guilds
(
    discord_id bigint not null
        constraint guilds_users_discord_id_fk
            references octorace.users,
    guild_id bigint not null
);