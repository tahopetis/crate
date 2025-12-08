use serde::{Deserialize, Serialize};
use serde_json::Value;
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonDiff {
    pub added: HashMap<String, Value>,
    pub removed: HashMap<String, Value>,
    pub modified: HashMap<String, (Value, Value)>, // (old, new)
}

pub fn calculate_json_diff(old: &Value, new: &Value) -> Result<JsonDiff> {
    let mut diff = JsonDiff {
        added: HashMap::new(),
        removed: HashMap::new(),
        modified: HashMap::new(),
    };

    if let (Value::Object(old_obj), Value::Object(new_obj)) = (old, new) {
        // Find added and modified keys
        for (key, new_val) in new_obj {
            match old_obj.get(key) {
                Some(old_val) => {
                    if old_val != new_val {
                        diff.modified.insert(key.clone(), (old_val.clone(), new_val.clone()));
                    }
                }
                None => {
                    diff.added.insert(key.clone(), new_val.clone());
                }
            }
        }

        // Find removed keys
        for (key, old_val) in old_obj {
            if !new_obj.contains_key(key) {
                diff.removed.insert(key.clone(), old_val.clone());
            }
        }
    }

    Ok(diff)
}

pub fn apply_json_diff(base: &Value, diff: &JsonDiff) -> Result<Value> {
    let mut result = base.clone();

    if let Value::Object(ref mut obj) = result {
        // Remove keys
        for key in diff.removed.keys() {
            obj.remove(key);
        }

        // Add or modify keys
        for (key, value) in &diff.added {
            obj.insert(key.clone(), value.clone());
        }

        for (key, (_, new_val)) in &diff.modified {
            obj.insert(key.clone(), new_val.clone());
        }
    }

    Ok(result)
}