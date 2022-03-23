use axum::Json;
use tracing::debug;
use vars::my_item_type_format::to_item_type_name;

use crate::{
    adventures::response::{Tabs, TabsResponse},
    app_response::AppError,
};

#[tracing::instrument]
pub async fn tabs_adventures() -> Result<Json<TabsResponse>, AppError> {
    let tabs: Vec<Tabs> = vec![
        Tabs {
            name: to_item_type_name(0),
            item_id: 0,
        },
        // Tabs {
        //     name: "日常".to_owned(),
        //     item_id: 1,
        // },
        Tabs {
            name: to_item_type_name(2),
            item_id: 2,
        },
        // Tabs {
        //     name: "游戏".to_owned(),
        //     item_id: 3,
        // },
        Tabs {
            name: to_item_type_name(4),
            item_id: 4,
        },
        Tabs {
            name: to_item_type_name(5),
            item_id: 5,
        },
        Tabs {
            name: to_item_type_name(6),
            item_id: 6,
        },
        Tabs {
            name: to_item_type_name(7),
            item_id: 7,
        },
    ];
    let response = TabsResponse { tab_list: tabs };
    debug!("response: {:?}", &response);
    Ok(response.into())
}
