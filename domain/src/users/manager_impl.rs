use std::num::NonZeroU32;

use repository::{
    find_user, find_user_by_username, update_user_password, update_username,
    users::models::InsertMyUsers,
};
use ring::{digest, pbkdf2};

use crate::{database_to_domain_error, DomainError, GetUserError, RegistryUsers, Users};

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
const HASH_ROUNDS: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(1_000) };
pub type Credential = [u8; CREDENTIAL_LEN];

#[derive(Clone, Debug)]
pub struct UsersManagerImpl;

#[async_trait]
impl super::UsersManager for UsersManagerImpl {
    async fn add_user(&self, reg_user: RegistryUsers) -> Result<Users, DomainError> {
        let id = InsertMyUsers {
            username: reg_user.username.clone(),
            password: hash_password(reg_user.password),
            roles: reg_user.roles.clone(),
        }
        .add_user()
        .await
        .map_err(database_to_domain_error)?;

        Ok(Users {
            id,
            username: reg_user.username,
            password: "".to_string(),
            roles: reg_user.roles,
        })
    }

    async fn get_user_by_username(&self, username: String) -> Result<Users, GetUserError> {
        let user = find_user_by_username(username.clone())
            .await
            .map_err(database_to_domain_error)?;
        match user {
            Some(u) => Ok(Users::from(u)),
            None => Err(GetUserError::NotFound {
                username: username.clone(),
            }),
        }
    }

    async fn get_user(&self, username: String, password: String) -> Result<Users, GetUserError> {
        let user = find_user(username.clone(), hash_password(password.clone()))
            .await
            .map_err(database_to_domain_error)?;
        match user {
            Some(u) => Ok(Users::from(u)),
            None => Err(GetUserError::NotFound {
                username: username.clone(),
            }),
        }
    }

    async fn verify_user(
        &self,
        username: String,
        login_password: String,
    ) -> Result<(bool, Users), GetUserError> {
        let my_user = find_user_by_username(username.clone())
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
        let pass = verify(password.to_string(), login_password);
        Ok((pass, user.clone()))
    }

    async fn change_password(
        &self,
        username: String,
        password: String,
    ) -> Result<bool, DomainError> {
        let success = update_user_password(username, hash_password(password))
            .await
            .map_err(database_to_domain_error)?;

        Ok(success)
    }

    async fn change_username(
        &self,
        old_username: String,
        new_username: String,
    ) -> Result<bool, DomainError> {
        let success = update_username(old_username, new_username)
            .await
            .map_err(database_to_domain_error)?;

        Ok(success)
    }
}

fn verify(password: String, attemp_password: String) -> bool {
    let my_password = base64::decode(&password).unwrap();

    pbkdf2::verify(
        PBKDF2_ALG,
        HASH_ROUNDS,
        &"asdf".as_bytes(),
        attemp_password.as_bytes(),
        my_password.as_slice(),
    )
    .is_ok()
}

fn hash_password(password: String) -> String {
    let mut to_store: Credential = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        PBKDF2_ALG,
        HASH_ROUNDS,
        &"asdf".as_bytes(),
        password.as_bytes(),
        &mut to_store,
    );
    base64::encode(&to_store)
}
