use anyhow;
use attribution_engine::services::postgresql;

//TODO: implement background worker body
pub async fn run() -> anyhow::Result<()> {
    let connection_string = format!(
        "postgres://postgres:{}@localhost:5433/{}",
        "admin", "attributiondb"
    );

    postgresql::Postgres::build(&connection_string).await?;

    Ok(())
}
