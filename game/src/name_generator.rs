use std::collections::HashMap;

pub(crate) struct NameGenerator {
    generated_names: HashMap<String, u32>,
}

impl NameGenerator {
    pub(crate) fn new() -> Self {
        NameGenerator {
            generated_names: HashMap::new(),
        }
    }

    pub(crate) fn generate_name(&mut self, base_name: &str) -> String {
        let count = self
            .generated_names
            .entry(base_name.to_string())
            .or_insert(0);

        *count += 1;

        if *count > 1 {
            format!("{}{}", base_name, count)
        } else {
            base_name.to_string()
        }
    }
}
