use std::collections::HashMap;
use std::convert::From;

pub struct QueryString<'buf> {
  data: HashMap<&'buf str, Value<'buf>>
}

pub enum Value<'buf> {
  Single(&'buf str),
  Multiple(Vec<&'buf str>)
}

impl<'buf> QueryString<'buf> {
  pub fn get(&self, key: &str) -> Option<&Value> {
    self.data.get(key)
  }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
  fn from(s: &'buf str) -> Self {
    let mut data = HashMap::new();

    for sub_str in s.split('&') {
      let mut key = sub_str;
      let mut value = "";
      if let Some(i) = sub_str.find('=') {
        key = &sub_str[..i];
        value = &sub_str[i+1..];
      }

      data.entry(key)
      .and_modify(|existing: &mut Value| match existing {
        Value::Single(prev_val) => {
          // since existing is a pointer to value in hashmap, * operator changes the location 
          // it is pointing to. now instead of old value it points to new vector created in memory
          *existing = Value::Multiple(vec![prev_val, value]);
        },
        Value::Multiple(vec) => vec.push(value)
      })
      .or_insert(Value::Single(value));
    }
    QueryString { data }
  }
}