use axum::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use password_auth::verify_password;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    id: Uuid,
    pub username: String,
    pub email: String,
    password: String,
}

impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("username", &self.username)
            .field("email", &self.email)
            .field("password", &"[redacted]")
            .finish()
    }
}

impl AuthUser for User {
    type Id = Uuid;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_bytes() // We use the password hash as the auth
                                 // hash--what this means
                                 // is when the user changes their password the
                                 // auth session becomes invalid.
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub next: Option<String>,
}

#[derive(Clone)]
pub struct Auth {
    pub db: Pool<Postgres>,
}

#[async_trait]
impl AuthnBackend for Auth {
    type User = User;
    type Credentials = Credentials;
    type Error = sqlx::Error;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user: Option<Self::User> = sqlx::query_as("select * from users where username = $1 ")
            .bind(creds.username)
            .fetch_optional(&self.db)
            .await?;

        Ok(user.filter(|user| {
            verify_password(creds.password, &user.password)
                .ok()
                .is_some() // We're using password-based authentication--this
                           // works by comparing our form input with an argon2
                           // password hash.
        }))
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let user = sqlx::query_as("select * from users where id = $1")
            .bind(user_id)
            .fetch_optional(&self.db)
            .await?;

        Ok(user)
    }
}

// We use a type alias for convenience.
//
// Note that we've supplied our concrete backend here.
pub type AuthSession = axum_login::AuthSession<Auth>;
