use anyhow::{bail, Result};
use std::collections::HashMap;

#[macro_export]
macro_rules! derive_getters {
    () => {
        fn set_params(&mut self, params: HashMap<String, String>) {
            self.params = params;
        }

        fn get_params(&self) -> &HashMap<String, String> {
            &self.params
        }

        fn get_params_mut(&mut self) -> &mut HashMap<String, String> {
            &mut self.params
        }
    };
}

pub trait Command: Runnable {
    fn assert_params(
        &mut self,
        keys: Vec<&str>,
        params: &[String],
    ) -> Result<&HashMap<String, String>> {
        for (i, key) in keys.into_iter().enumerate() {
            if let Some(value) = params.get(i) {
                self.get_params_mut()
                    .insert(key.to_string(), value.to_string());
            } else {
                bail!(format!("No value entered for the param: {}", key));
            }
        }

        Ok(self.get_params())
    }
    fn get_param(&self, key: &str) -> &str {
        self.get_params()
            .get(key)
            .expect(&format!("Can't find expected param: {}", key))
    }
    fn set_params(&mut self, args: HashMap<String, String>);
    fn get_params(&self) -> &HashMap<String, String>;
    fn get_params_mut(&mut self) -> &mut HashMap<String, String>;
    fn help(&self) -> Result<()>;
}

pub trait Runnable {
    fn run(&mut self, action: &String, params: &[String]) -> Result<()>;
}

pub(crate) use derive_getters;
