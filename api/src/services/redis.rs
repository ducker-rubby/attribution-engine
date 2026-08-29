use redis::Commands;

pub fn connect_redis() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1:6379/")?;
    let mut con = client.get_connection()?;

    redis::cmd("SET").arg("test_key").arg("42").exec(&mut con)?;
    let value: String = con.get("test_key")?;
    println!("{:?}", value);

    Ok(())
}
