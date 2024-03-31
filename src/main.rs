use bevy_reflect::Reflect;
use crate::db::libsql::AioDatabase;

pub mod db;

#[derive(Reflect, Default)]
struct Test {
    name: String,
    test: i32,
    test2:i32
}

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    AioDatabase::create::<Test>("G:\\".into(), "Test".into()).await;
}