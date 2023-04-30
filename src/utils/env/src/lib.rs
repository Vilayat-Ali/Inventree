use serde::Deserialize;

// #[derive(Deserialize, Debug, PartialEq)]
// #[serde(rename_all = "lowercase")]
// pub enum Size {
//     Small,
//     Medium,
//     Large,
// }

#[derive(Deserialize, Debug)]
pub struct FrontendConfig {
    pub foo: String,
}

#[derive(Deserialize, Debug)]
pub struct BackendConfig {
    pub server: u16,
}

pub fn import_envs<T>() -> Option<T>
where
    T: for<'a> Deserialize<'a>,
{
    match envy::from_env::<BackendConfig>() {
        Ok(config) => println!("{:#?}", config),
        Err(e) => eprintln!("{:#?}", e),
    }
    match envy::from_env::<T>() {
        Ok(config) => Some(config),
        Err(_e) => None,
    }
}
