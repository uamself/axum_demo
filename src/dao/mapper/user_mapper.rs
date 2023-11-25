use mysql::params;
use mysql::prelude::{BinQuery, Queryable, WithParams};

use crate::dao::entity::entity::User;
use crate::dao::mysql_conn_pool::get_connect;
use crate::error::error::GlobalError;
use crate::utils;

#[allow(dead_code)]
pub struct UserMapper;

#[allow(dead_code)]
impl UserMapper {
    // 创建用户测试
    pub fn insert(name: &str, password: &str) -> Result<u64, GlobalError> {
        // 获取数据库链接
        let mut conn = get_connect();
        // 生成主键id
        let id = utils::generate_id()?;
        // 执行插入语句
        let x: Result<u64, GlobalError> = match "insert into user (id,name,password,enabled,create_time,modify_time) values (?,?,?,1,now(),now())"
            .with((id, name, password))
            .run(&mut conn) {
            // 返回受影响的数据行数
            Ok(res) => {
                Ok(res.affected_rows())
            }
            Err(e) => {
                // error!(e);
                Err(GlobalError::new(200, "创建用户失败", e.to_string().as_str()))
            }
        };
        x
    }

    pub fn get_by_id(id: &str) -> Option<User> {
        // 获取数据库链接
        let mut conn = get_connect();

        // 根据id查询账号信息
        let query_result = conn
            .exec_first(
                "select id,name,password,enabled,create_time,modify_time from user where id=:id",
                params! {"id"=>id},
            )
            .map(|row| {
                row.map(
                    |(id, name, password, enabled, create_time, modify_time)| User {
                        id,
                        name,
                        password,
                        enabled,
                        create_time,
                        modify_time,
                    },
                )
            });
        // 判断是否查询到数据
        match query_result {
            Ok(result) => result,
            Err(_) => None,
        }
    }

    pub fn get_all_users() -> Option<Vec<User>> {
        let mut conn = get_connect();
        let vec = conn
            .query_map(
                "select id, name, password, enabled, create_time,modify_time from user",
                |(id, name, password, enabled, create_time, modify_time)| User {
                    id,
                    name,
                    password,
                    enabled,
                    create_time,
                    modify_time,
                },
            )
            .expect("query failed");
        Some(vec)
    }

    pub fn delete_by_id(id: &str) -> Result<u64, GlobalError> {
        let mut conn = get_connect();
        let x = match "delete from user where id=?".with((id,)).run(&mut conn) {
            Ok(res) => Ok(res.affected_rows()),
            Err(e) => Err(GlobalError::new(
                200,
                "删除用户失败",
                e.to_string().as_str(),
            )),
        };
        x
    }

    pub fn update(user: User) -> Result<u64, GlobalError> {
        let mut conn = get_connect();
        let x = match "update user set name=?, password=?,enabled=?, modify_time=now() where id=?"
            .with((&user.name, &user.password, &user.enabled, &user.id))
            .run(&mut conn)
        {
            Ok(res) => Ok(res.affected_rows()),
            Err(e) => Err(GlobalError::new(
                200,
                "用户信息更新失败",
                e.to_string().as_str(),
            )),
        };
        x
    }
}

#[cfg(test)]
mod test {
    use chrono::NaiveDateTime;

    use crate::dao;
    use crate::dao::entity::entity::User;
    use crate::dao::mapper::user_mapper::UserMapper;

    #[test]
    pub fn insert_test() {
        dao::init();
        let res = UserMapper::insert("mxy", "098765");
        assert_eq!(res, Ok(1));
    }

    #[test]
    pub fn get_by_id_test() {
        dao::init();
        let res = UserMapper::get_by_id("174278592971558912");
        match res {
            None => {
                dbg!("None");
            }
            Some(_) => {
                dbg!(res);
            }
        }
    }
    #[test]
    pub fn get_all_users_test() {
        dao::init();
        let res = UserMapper::get_all_users();
        match res {
            None => {
                dbg!("None");
            }
            Some(_) => {
                dbg!(res);
            }
        }
    }

    #[test]
    pub fn delete_by_id_test() {
        dao::init();
        let res = UserMapper::delete_by_id("174288869192519680");
        assert_eq!(res, Ok(1));
    }

    // 表名大小写敏感
    #[test]
    pub fn update_test() {
        dao::init();
        let data_time = chrono::offset::Utc::now();
        let res = UserMapper::update(User {
            id: String::from("174278592971558912"),
            name: String::from("msw"),
            password: String::from("123456"),
            enabled: 1,
            create_time: NaiveDateTime::from_timestamp_opt(data_time.timestamp(), 0).unwrap(),
            modify_time: NaiveDateTime::from_timestamp_opt(data_time.timestamp(), 0).unwrap(),
        });
        assert_eq!(res, Ok(1));
    }
}
