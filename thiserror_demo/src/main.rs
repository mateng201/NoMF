use thiserror::Error;

#[derive(Debug, Error)]
enum DemoError {
    #[error("registry {registry} not configured")]
    RegistryNotConfigured { registry: &'static str },

    #[error("failed to parse number")]
    ParseNumber(#[from] std::num::ParseIntError),
}

type Result<T> = std::result::Result<T, DemoError>;//定义类型别名

fn get_registry_address(registry: &'static str) -> Result<&'static str> {
    Err(DemoError::RegistryNotConfigured { registry })
}

fn parse_user_id(raw: &str) -> Result<u32> {
    let user_id = raw.parse::<u32>()?;
    Ok(user_id)
}

fn main() {
    let err1 = get_registry_address("identity").unwrap_err();
    println!("manual error: {err1}");
    println!("debug form: {err1:?}");

    let err2 = parse_user_id("abc").unwrap_err();
    println!("from error: {err2}");
    println!("debug form: {err2:?}");
}
