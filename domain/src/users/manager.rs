use std::future::Future;

use crate::{DomainError, GetUserError, RegistryUsers, Users};

pub trait UsersManager {
    fn add_user(
        &self,
        reg_user: RegistryUsers,
    ) -> impl Future<Output = Result<Users, DomainError>> + Send;

    fn get_user_by_username(
        &self,
        username: String,
    ) -> impl Future<Output = Result<Users, GetUserError>> + Send;

    fn get_user(
        &self,
        username: String,
        password: String,
    ) -> impl Future<Output = Result<Users, GetUserError>> + Send;

    fn verify_user(
        &self,
        username: String,
        login_password: String,
    ) -> impl Future<Output = Result<(bool, Users), GetUserError>> + Send;

    fn change_password(
        &self,
        username: String,
        password: String,
    ) -> impl Future<Output = Result<bool, DomainError>> + Send;

    fn change_username(
        &self,
        old_username: String,
        new_username: String,
    ) -> impl Future<Output = Result<bool, DomainError>> + Send;
}
