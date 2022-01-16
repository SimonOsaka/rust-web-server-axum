use std::sync::{Arc, RwLock};

use casbin::{CoreApi, DefaultModel, EnforceArgs, Enforcer, Error, MemoryAdapter, MgmtApi};
use once_cell::sync::OnceCell;
use tracing::{debug, error};

const RBAC_MODEL: &str = include_str!("../../rbac_model.conf");
const RBAC_POLICY: &str = include_str!("../../rbac_policy.csv");

static EF: OnceCell<Arc<RwLock<Enforcer>>> = OnceCell::new();

/// Create enforcer, load all policies to `memory adapter`.
async fn get_enforcer() -> Result<Enforcer, Error> {
    let m = DefaultModel::from_str(RBAC_MODEL).await?;
    let a = MemoryAdapter::default();
    let mut e = Enforcer::new(m, a).await?;
    e.enable_log(true);
    e.add_policies(read_policy(RBAC_POLICY, PtypeEnum::P))
        .await?;
    e.add_grouping_policies(read_policy(RBAC_POLICY, PtypeEnum::G))
        .await?;
    Ok(e)
}

/// init casbin enforcer, thread safe.
pub(crate) async fn init_authorization() {
    let e = get_enforcer().await.unwrap();
    match EF.set(Arc::new(RwLock::new(e))) {
        Ok(_o) => debug!("init casbin success!!!"),
        Err(_e) => error!("init casbin failed!!!"),
    };
}

/// The authorization verification is passed or not.
/// # Result
/// OK
/// - `true`: success
/// - `false`: failed
///
/// Err
/// - AuthorizationError
///
/// # Examples
/// ```
/// let s = enforce(("alice", "data1", "read"));
/// println!("{}", s);
/// ```
pub async fn enforce<ARGS: EnforceArgs + Copy>(rvals: ARGS) -> Result<bool, AuthorizationError> {
    let ef = EF.get().unwrap().read().unwrap();
    ef.enforce(rvals)
        .map_err(|e| AuthorizationError::VerifyFailed {
            args: format!("{:?}", &rvals.try_into_vec().unwrap()),
            source: e,
        })
}

/// Read RBAC policy strings to `Vec<Vec<String>>`
fn read_policy(multi_line_str: &str, ptype_enum: PtypeEnum) -> Vec<Vec<String>> {
    let ptype = match ptype_enum {
        PtypeEnum::P => 'p',
        PtypeEnum::G => 'g',
    };

    multi_line_str
        .lines()
        .filter(|&x| !x.is_empty() && x.starts_with(ptype))
        .enumerate()
        .map(|(_, s)| {
            s.split(',')
                .collect::<Vec<&str>>()
                .into_iter()
                .map(|f| f.trim().to_string())
                .collect()
        })
        .collect::<Vec<Vec<String>>>()
        .into_iter()
        .map(|mut f| {
            f.remove(0);
            f
        })
        .collect::<Vec<Vec<String>>>()
}

enum PtypeEnum {
    P,
    G,
}

#[derive(thiserror::Error, Debug)]
pub enum AuthorizationError {
    #[error("verify failed({args})")]
    VerifyFailed { args: String, source: Error },
}

#[cfg(test)]
mod tests {
    use crate::{
        authorization::{read_policy, PtypeEnum},
        enforce, init,
    };

    #[tokio::test]
    async fn test_enforce() {
        init().await;
        let pass = enforce(("alice", "data1", "read")).await.unwrap();
        assert_eq!(true, pass);
    }

    #[tokio::test]
    async fn test_enforce_role() {
        init().await;
        let pass = enforce(("alice", "data1", "read")).await.unwrap();
        assert_eq!(true, pass);

        let pass = enforce(("bob", "data2", "read")).await.unwrap();
        assert_eq!(true, pass);
    }

    #[test]
    fn test_string_multi_line() {
        let multi_line_str = r#"p, alice, data1, read
p, bob, data2, write

g, alice, admin
g, admin, data1_admin
g, admin, data2_admin"#;
        let p = read_policy(multi_line_str, PtypeEnum::P);

        println!("p: {:?}", p);

        let g = read_policy(multi_line_str, PtypeEnum::G);

        println!("g: {:?}", g);
    }
}
