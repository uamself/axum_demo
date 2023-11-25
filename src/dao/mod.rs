use crate::utils::constant::GLOBAL_CONFIG;

mod mysql_conn_pool;
mod entity;
mod mapper;

// const DB_URL: &str = "mysql://awesome_db:m3HsjCPhTM5aEYxJ@175.24.205.216:3306/awesome_db";

pub fn init() {
    let config = &GLOBAL_CONFIG;
    let mysql = &config.mysql;
    let prefix = "mysql://";
    let url = prefix.to_owned() + mysql.username.as_str() + ":" + mysql.password.as_str() + "@" + mysql.host.as_str() + ":" + mysql.port.to_string().as_str() + "/" + mysql.db_name.as_str();
    // 初始化链接池
    mysql_conn_pool::init_mysql_pool(url.as_str());
}

