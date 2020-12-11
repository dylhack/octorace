use crate::api::user::get_contributions;
use crate::config::Config;
use crate::db;
use crate::db::pool::Pool;
use chrono::{Duration, Utc};
use std::error::Error;
use std::sync::Arc;

async fn update_contributions(pool: Arc<Pool>, config: Arc<Config>) -> Result<(), Box<dyn Error>> {
    let connections = sqlx::query!(
        "
        SELECT github, users.expires, users.discord_id FROM octorace.users
            INNER JOIN octorace.connections c
                on users.discord_id = c.discord_id"
    )
    .fetch_all(&*pool)
    .await?;

    let now = Utc::now().naive_utc();
    for connection in connections {
        if connection.expires < now {
            let contribs = get_contributions(connection.github, &*config).await;
            let expires = (Utc::now() + Duration::minutes(5)).naive_utc();
            sqlx::query!(
                "UPDATE octorace.users SET contributions = $1, expires = $2 WHERE discord_id = $3",
                contribs,
                expires,
                connection.discord_id
            )
            .execute(&*pool)
            .await?;
        }
    }
    Ok(())
}

pub async fn start_tasks() {
    let pool = Arc::new(db::pool::pool().await);
    let config = Arc::new(Config::new());

    tokio::spawn(async move {
        loop {
            let pool1 = Arc::clone(&pool);
            let config1 = Arc::clone(&config);
            tokio::spawn(async move {
                if let Err(e) = update_contributions(Arc::clone(&pool1), Arc::clone(&config1)).await
                {
                    println!(
                        "An error occurred while running update_contributions() >>> {}",
                        e
                    );
                }
            });

            tokio::time::delay_for(std::time::Duration::from_secs(5)).await;
        }
    });
}
