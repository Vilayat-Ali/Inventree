use serde::Deserialize;

// #[derive(Deserialize, Debug, PartialEq)]
// #[serde(rename_all = "lowercase")]
// pub enum Size {
//     Small,
//     Medium,
//     Large,
// }

#[derive(Deserialize, Debug)]
pub struct Config {
    pub frontend_envs: FrontendConfig,
    pub backend_envs: BackendConfig,
}

#[derive(Deserialize, Debug)]
pub struct FrontendConfig {
    pub foo: String,
}

#[derive(Deserialize, Debug)]
pub struct BackendConfig {
    pub foo: String,
}

pub fn import_envs<T>() -> Result<T, envy::Error>
where
    T: for<'a> Deserialize<'a>,
{
    let config = envy::from_env::<T>()?;
    Ok(config)
}
