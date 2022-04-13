use repository::{
    find_user_by_username, update_user_password, update_username, users::models::NewMyUsers,
};

use crate::{
    database_to_domain_error,
    utils::{hash, verify},
    DomainError, GetUserError, RegistryUsers, Users,
};

#[derive(Clone, Debug)]
pub struct UsersManagerImpl;

#[async_trait]
impl super::UsersManager for UsersManagerImpl {
    #[tracing::instrument(skip(self,reg_user),fields(reg_user.username=%reg_user.username))]
    async fn add_user(&self, reg_user: RegistryUsers) -> Result<Users, DomainError> {
        let inserted_my_user_id = NewMyUsers {
            username: reg_user.username.clone(),
            password: hash_password(reg_user.password),
            roles: reg_user.roles.clone(),
        }
        .insert(None)
        .await
        .map_err(database_to_domain_error)?;

        Ok(Users {
            id: inserted_my_user_id,
            username: reg_user.username,
            password: "".to_string(),
            roles: reg_user.roles,
        })
    }

    #[tracing::instrument(skip(self))]
    async fn get_user_by_username(&self, username: String) -> Result<Users, GetUserError> {
        let user = find_user_by_username(username.clone(), None)
            .await
            .map_err(database_to_domain_error)?;
        match user {
            Some(u) => Ok(Users::from(u)),
            None => Err(GetUserError::NotFound {
                username: username.clone(),
            }),
        }
    }

    #[tracing::instrument(skip(self))]
    async fn get_user(&self, username: String, password: String) -> Result<Users, GetUserError> {
        let user = find_user_by_username(username.clone(), None)
            .await
            .map_err(database_to_domain_error)?;
        match user {
            Some(u) => {
                if u.password == hash_password(password) {
                    Ok(Users::from(u))
                } else {
                    Err(GetUserError::PasswordNotCorrect {
                        username: username.clone(),
                    })
                }
            }
            None => Err(GetUserError::NotFound {
                username: username.clone(),
            }),
        }
    }

    #[tracing::instrument(skip(self))]
    async fn verify_user(
        &self,
        username: String,
        login_password: String,
    ) -> Result<(bool, Users), GetUserError> {
        let my_user = find_user_by_username(username.clone(), None)
            .await
            .map_err(database_to_domain_error);
        let result = match my_user {
            Ok(mu) => match mu {
                Some(u) => Ok(Users::from(u)),
                None => Err(GetUserError::NotFound {
                    username: username.clone(),
                }),
            },
            Err(_) => Err(GetUserError::NotFound {
                username: username.clone(),
            }),
        };

        let user = result?;
        let password = &user.password;
        let pass = verify_password(password.to_string(), login_password);
        Ok((pass, user.clone()))
    }

    #[tracing::instrument(skip(self))]
    async fn change_password(
        &self,
        username: String,
        password: String,
    ) -> Result<bool, DomainError> {
        let success = update_user_password(username, hash_password(password), None)
            .await
            .map_err(database_to_domain_error)?;

        Ok(success)
    }

    #[tracing::instrument(skip(self))]
    async fn change_username(
        &self,
        old_username: String,
        new_username: String,
    ) -> Result<bool, DomainError> {
        let success = update_username(old_username, new_username, None)
            .await
            .map_err(database_to_domain_error)?;

        Ok(success)
    }
}

fn verify_password(password: String, attemp_password: String) -> bool {
    verify(password, attemp_password)
}

fn hash_password(password: String) -> String {
    hash(password)
}
