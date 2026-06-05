use toml::Value;

use crate::ConfigError;

pub(crate) fn merge_values(base: &mut Value, overlay: Value) {
    match (base, overlay) {
        (Value::Table(base), Value::Table(overlay)) => {
            for (key, value) in overlay {
                match base.get_mut(&key) {
                    Some(existing) => merge_values(existing, value),
                    None => {
                        base.insert(key, value);
                    }
                }
            }
        }
        (base, overlay) => {
            *base = overlay;
        }
    }
}

pub(crate) fn select_namespaced_or_legacy(value: Value, namespace: &str) -> Value {
    let Value::Table(mut root) = value else {
        return value;
    };

    let namespaced = root.remove(namespace);
    let mut selected = Value::Table(root);
    if let Some(namespaced) = namespaced {
        merge_values(&mut selected, namespaced);
    }

    selected
}

pub(crate) fn set_dotted_value(
    root: &mut Value,
    dotted_key: &str,
    value: Value,
) -> Result<(), ConfigError> {
    let parts: Vec<&str> = dotted_key.split('.').collect();
    if parts.iter().any(|part| part.trim().is_empty()) {
        return Err(ConfigError::EmptyOverrideKey(dotted_key.to_owned()));
    }

    let mut current = root;
    for part in &parts[..parts.len() - 1] {
        let Value::Table(table) = current else {
            return Err(ConfigError::NonTableOverride(dotted_key.to_owned()));
        };
        current = table
            .entry((*part).to_owned())
            .or_insert_with(|| Value::Table(Default::default()));
    }

    let Value::Table(table) = current else {
        return Err(ConfigError::NonTableOverride(dotted_key.to_owned()));
    };
    table.insert(parts[parts.len() - 1].to_owned(), value);
    Ok(())
}
