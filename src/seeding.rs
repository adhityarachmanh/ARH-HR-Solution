use sqlx::PgPool;
use tracing::info;

const ADMIN_USERNAME: &str = "superadmin";
const ADMIN_PASSWORD: &str = "password123";
const ADMIN_ROLE_SLUG: &str = "super_admin";

pub async fn seed_admin_user(pool: &PgPool) -> Result<(), sqlx::Error> {
    let exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)",
        ADMIN_USERNAME
    )
    .fetch_one(pool)
    .await?
    .unwrap_or(false);

    if !exists {
        info!(
            "Admin user '{}' not found, creating new one...",
            ADMIN_USERNAME
        );

        let hashed_password =
            bcrypt::hash(ADMIN_PASSWORD, bcrypt::DEFAULT_COST).expect("Failed to hash password");

        let mut tx = pool.begin().await?;

        let new_user_id: i64 = sqlx::query_scalar(
            "INSERT INTO users (username, password_hash) VALUES ($1, $2) RETURNING id",
        )
        .bind(ADMIN_USERNAME)
        .bind(&hashed_password)
        .fetch_one(&mut *tx)
        .await?;

        let superadmin_role_id: Option<i32> =
            sqlx::query_scalar("SELECT id FROM roles WHERE name = $1")
                .bind(ADMIN_ROLE_SLUG)
                .fetch_optional(&mut *tx)
                .await?;

        if let Some(role_id) = superadmin_role_id {
            sqlx::query("INSERT INTO user_roles (user_id, role_id) VALUES ($1, $2)")
                .bind(new_user_id)
                .bind(role_id)
                .execute(&mut *tx)
                .await?;
            info!(
                "Successfully linked user '{}' to '{}' role.",
                ADMIN_USERNAME, ADMIN_ROLE_SLUG
            );
        } else {
            tracing::error!("'{}' role not found in database!", ADMIN_ROLE_SLUG);
        }

        tx.commit().await?;
        info!("Admin user '{}' created successfully.", ADMIN_USERNAME);
    } else {
        info!(
            "Admin user '{}' already exists. Skipping seed.",
            ADMIN_USERNAME
        );
    }
    Ok(())
}
