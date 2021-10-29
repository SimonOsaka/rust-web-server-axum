use crate::{DomainError, GetUserError, RegistryUsers, Users};

#[async_trait]
pub trait UsersManager {
    async fn add_user(&self, reg_user: RegistryUsers) -> Result<Users, DomainError>;

    async fn get_user_by_username(&self, username: String) -> Result<Users, GetUserError>;

    async fn get_user(&self, username: String, password: String) -> Result<Users, GetUserError>;

    async fn verify_user(
        &self,
        username: String,
        login_password: String,
    ) -> Result<(bool, Users), GetUserError>;

    async fn change_password(
        &self,
        username: String,
        password: String,
    ) -> Result<bool, DomainError>;

    async fn change_username(
        &self,
        old_username: String,
        new_username: String,
    ) -> Result<bool, DomainError>;
}
