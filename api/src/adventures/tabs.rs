use std::convert::Infallible;

use crate::{
    adventures::response::{Tabs, TabsResponse},
    consts::my_item_type_format::to_item_type_name,
};

pub async fn tabs_adventures(token: Option<String>) -> Result<impl warp::Reply, Infallible> {
    debug!("token: {:?}", token);

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
    Ok(warp::reply::json(&response))
}
