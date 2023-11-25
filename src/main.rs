use axum::Router;
use tracing::info;

use crate::utils::constant::GLOBAL_CONFIG;

mod dao;
mod error;
mod handler;
mod load_config;
mod logs;
mod utils;

#[tokio::main]
async fn main() {
    // 初始化工具箱 todo
    utils::init();
    // 初始化日志
    logs::init();

    let config = &GLOBAL_CONFIG;
    let server = &config.application.server;

    info!("main function start");
    // 初始化数据库连接池
    dao::init();
    // 初始化service
    //service::init();
    // 初始化请求路由
    info!("收集所有的请求路由配置---------------开始---------------");
    let app = Router::new()
        .merge(handler::test_controller::get_test_routes())
        .merge(handler::user_controller::get_user_routes());
    info!("收集所有的请求路由配置---------------结束---------------");

    // run it with hyper on localhost:3000
    axum::Server::bind(
        &("0.0.0.0:".to_owned() + server.port.to_string().as_str())
            .parse()
            .unwrap(),
    )
    .serve(app.into_make_service())
    .await
    .unwrap();
}
