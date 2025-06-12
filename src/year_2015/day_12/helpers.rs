use serde_json::{Map, Value};

#[derive(Default)]
pub struct JsonInterpreter {
    excluded_string: Option<String>,
}

impl JsonInterpreter {
    pub fn without(excluded_string: &str) -> Self {
        let excluded_string = Some(String::from(excluded_string));

        Self { excluded_string }
    }

    pub fn add_numbers(&self, input: &Value) -> i64 {
        match input {
            Value::Number(num) => num.as_i64().unwrap(),

            Value::Array(array) => self.process_array(array),

            Value::Object(object) => self.process_object(object),

            _ => 0,
        }
    }

    fn process_array(&self, array: &Vec<Value>) -> i64 {
        let mut sum = 0;

        for value in array {
            sum += self.add_numbers(value);
        }

        sum
    }

    fn process_object(&self, object: &Map<String, Value>) -> i64 {
        let mut sum = 0;

        for value in object.values() {
            match value {
                Value::String(text) => {
                    if let Some(excluded) = &self.excluded_string {
                        if text == excluded {
                            return 0;
                        }
                    }
                }
                other => sum += self.add_numbers(other),
            }
        }

        sum
    }
}
