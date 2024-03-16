mod mongodbc;
mod tools;
use core::panic;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;

use axum::extract::Path;
use axum::http::{HeaderValue, Response};
use axum::response::IntoResponse;
use axum::{
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use bson::oid::ObjectId;
use bson::Document;
use futures::StreamExt;
use mongodb::bson::doc;
use mongodb::options::FindOneOptions;
use mongodb::Collection;
use mongodbc::game::Game;
use mongodbc::get_handle_with_collection;
use mongodbc::menu::Menu;
use mongodbc::user::User;
use tools::{load_image_to_base64, update_image_by_base64};
use tower_http::cors::CorsLayer;

use crate::mongodbc::game::GameData;
use crate::tools::{delete_image, save_image_by_base64};

// #[tokio::main]
// async fn main() {
//     let _id = ObjectId::from_str(&"65e1bc801f9360e4de8f8ce8".to_string());
//     let s = get_game_image_path(_id.unwrap()).await;
//     println!("{}", s);
// }

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/menu", get(get_menu_data))
        .route("/menu", put(update_menu_data))
        .route("/games", get(get_game_data))
        .route("/games", post(add_game_info))
        .route("/games", put(update_game_data))
        .route("/games/:_id", delete(delete_game_data))
        .route("/user", post(add_user_info))
        .route("/user", get(get_user_data))
        .route("/init", get(init_menu_data))
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_methods(tower_http::cors::Any),
        );
        
    // run our app with hyper, listening globally on port 3000
    let listener =
        tokio::net::TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 3000))
            .await
            .unwrap();
    axum::serve(listener, app).await.unwrap();
}
//
async fn init_menu_data() -> String {
    let collection: Collection<Menu> =
        match get_handle_with_collection("game_menu", "menu_data").await {
            Ok(v) => v,
            Err(e) => panic!("When get a collection handle, an error occurs: {:?}", e),
        };
    let menu_data = Menu::new(1, "公告".to_string(), "链接".to_string());
    let result = collection.insert_one(menu_data, None).await;
    format!("_id:{}", result.unwrap().inserted_id)
}
//获取游戏菜单页面信息
async fn get_menu_data() -> impl IntoResponse {
    let collection: Collection<Menu> =
        match get_handle_with_collection("game_menu", "menu_data").await {
            Ok(v) => v,
            Err(e) => panic!("When get a collection handle, an error occurs: {:?}", e),
        };
    // Query the menu data in the collection with a filter and an option.
    let filter = doc! {};
    match collection.find_one(filter, None).await {
        Ok(v) => match v {
            Some(value) => {
                return Response::builder()
                    .status(200)
                    .body(serde_json::to_string(&value).unwrap())
                    .unwrap();
            }
            None => {
                let not_found: String = "暂未初始化公告信息！".to_string();
                return Response::builder()
                    .status(200)
                    .body(serde_json::to_string(&not_found).unwrap())
                    .unwrap();
            }
        },
        Err(e) => panic!("查询时出现了错误：{:?}", e),
    };
}
//更改游戏菜单页面信息
async fn update_menu_data(Json(menu_data): Json<Menu>) -> String {
    let collection: Collection<Menu> =
        match get_handle_with_collection("game_menu", "menu_data").await {
            Ok(v) => v,
            Err(e) => panic!("When get a collection handle, an error occurs: {:?}", e),
        };
    // let id = ObjectId::from_str("1").expect("Could not convert to ObjectId");
    let filter_doc = doc! { "_id": 1 };
    let update_doc = doc! {
            "$set": doc! {
              "content":menu_data.content.clone(),
            "link": menu_data.link.clone()
          }
    };
    let res = collection.update_one(filter_doc, update_doc, None).await;
    format!(
        " content: {}, link: {}, count: {}",
        menu_data.content,
        menu_data.link,
        res.unwrap().modified_count
    )
}

//查询游戏图片路径
async fn get_game_image_path(_id: ObjectId) -> String {
    let collection: Collection<Document> =
        match get_handle_with_collection("game_menu", "game_data").await {
            Ok(v) => v,
            Err(e) => panic!("When get a collection handle, an error occurs: {:?}", e),
        };
    // Query the menu data in the collection with a filter and an option.
    let filter = doc! {"_id" : _id};
    let option = FindOneOptions::builder()
        .projection(doc! {
          "image_path":1,
          "_id":0
        })
        .build();
    let image_path = match collection.find_one(filter, option).await {
        Ok(v) => v.unwrap(),
        Err(e) => panic!("查询时出现了错误：{:?}", e),
    };
    image_path.get_str("image_path").unwrap().to_string()
}

//查询游戏列表
async fn get_game_data() -> (StatusCode, Json<Vec<Game>>) {
    let collection: Collection<GameData> =
        match get_handle_with_collection("game_menu", "game_data").await {
            Ok(v) => v,
            Err(e) => panic!("When get a collection handle, an error occurs: {:?}", e),
        };
    // Query the menu data in the collection with a filter and an option.
    let filter = doc! {};

    let mut cursor = match collection.find(filter, None).await {
        Ok(v) => v,
        Err(e) => panic!("查询时出现了错误：{:?}", e),
    };
    let mut games: Vec<Game> = Vec::new();
    while let Some(doc) = cursor.next().await {
        match doc {
            Ok(doc) => {
                let _id = doc._id.to_string();
                let image = match load_image_to_base64(doc.image_path).await {
                    Ok(v) => v,
                    Err(e) => panic!("发生了一些错误！{:?}", e),
                };
                let name = doc.name;
                let desciption = doc.description;
                let download_link = doc.download_link;
                let game_path = doc.game_path;
                let game = Game::new(_id, image, name, desciption, download_link, game_path);
                games.push(game);
            }
            Err(e) => panic!("获取文档时出现了错误: {}", e),
        };
    }
    (StatusCode::OK, Json(games))
}
//添加游戏信息
async fn add_game_info(Json(game): Json<Game>) -> String {
    let collection: Collection<Document> =
        match get_handle_with_collection("game_menu", "game_data").await {
            Ok(v) => v,
            Err(e) => panic!("When get a collection handle, an error occurs: {:?}", e),
        };
    let image_path = match save_image_by_base64(game.image).await {
        Ok(v) => v,
        Err(e) => panic!("发生错误！{:?}", e),
    };
    let base_path = "./games/".to_string() + &game.name + "/";

    let doc = doc! {
      "image_path":image_path,
      "name":game.name,
      "description":game.description,
      "download_link":game.download_link,
      "game_path":base_path + &game.game_path
    };
    let result = collection.insert_one(doc, None).await;
    format!("_id:{}", result.unwrap().inserted_id)
}
//修改游戏信息
async fn update_game_data(Json(game_data): Json<Game>) -> String {
    let collection: Collection<Game> =
        match get_handle_with_collection("game_menu", "game_data").await {
            Ok(v) => v,
            Err(e) => panic!("When get a collection handle, an error occurs: {:?}", e),
        };
    let object_id = ObjectId::from_str(&game_data._id).unwrap();
    let filter_doc = doc! { "_id": object_id };
    let image_path = get_game_image_path(object_id).await;
    println!("..............{}", image_path);
    match update_image_by_base64(image_path.clone(), game_data.image).await {
        Ok(v) => v,
        Err(e) => panic!("{:?}", e),
    };
    let base_path = "./games/".to_string() + &game_data.name + "/";
    let update_doc = doc! {
            "$set": doc! {
            "image_path":image_path,
            "name":game_data.name.clone(),
            "description": game_data.description.clone(),
            "download_link":game_data.download_link.clone(),
            "game_path":base_path + &game_data.game_path
          }
    };
    let res = collection.update_one(filter_doc, update_doc, None).await;

    format!("_id:{}", res.unwrap().modified_count)
}

//删除游戏信息
async fn delete_game_data(Path(_id): Path<String>) -> String {
    let collection: Collection<Game> =
        match get_handle_with_collection("game_menu", "game_data").await {
            Ok(v) => v,
            Err(e) => panic!("When get a collection handle, an error occurs: {:?}", e),
        };
    let object_id = ObjectId::from_str(&_id).unwrap();
    let image_path = get_game_image_path(object_id).await;
    let _ = delete_image(image_path).await;
    let filter_doc = doc! { "_id": object_id};
    let res = collection.delete_many(filter_doc, None).await;
    format!("共删除：{}", res.unwrap().deleted_count)
}
//查询用户信息
async fn get_user_data() -> impl IntoResponse {
    let collection: Collection<User> =
        match get_handle_with_collection("game_menu", "user_data").await {
            Ok(v) => v,
            Err(e) => panic!("When get a collection handle, an error occurs: {:?}", e),
        };
    // Query the menu data in the collection with a filter and an option.
    let filter = doc! {
      "_id":1
    };

    match collection.find_one(filter, None).await {
        Ok(v) => match v {
            Some(value) => Response::builder()
                .status(200)
                .body(serde_json::to_string(&value).unwrap())
                .unwrap(),
            None => Response::builder()
                .status(404)
                .body(serde_json::to_string("User data not found").unwrap())
                .unwrap(),
        },
        Err(e) => panic!("查询时出现了错误：{:?}", e),
    };
}
//添加用户信息
async fn add_user_info() -> String {
    let collection: Collection<User> =
        match get_handle_with_collection("game_menu", "user_data").await {
            Ok(v) => v,
            Err(e) => panic!("When get a collection handle, an error occurs: {:?}", e),
        };
    let user_data = User::new(2, "admin".to_string(), "123456".to_string());
    let result = collection.insert_one(user_data, None).await;
    format!("_id:{}", result.unwrap().inserted_id)
}
