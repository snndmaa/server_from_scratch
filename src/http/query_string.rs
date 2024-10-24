use std::collections::HashMap;

// a=1&b=2&c&d=&e===&d=7&d=abc

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl <'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

// a=1&b=2&c&d=&e===&d=7&d=abc
impl<'buf> From<&'buf str> for QueryString<'buf> {  // We do not use FromStr because it doesn't support lifetime annotations and we do not use TryFrom because this conversion cannot fail i.e we would always be able to convert a string to our QueryString struct
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();

        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";

            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..]; 
            }

            data.entry(key)
            .and_modify(|existing: &mut Value | match existing {
                Value::Single(prev_val) => {
                    // let mut vec = Vec::new();
                    // vec.push(val);
                    // vec.push(&prev_val)

                    // Alternative way for doing the above:
                    *existing = Value::Multiple(vec![prev_val, val])    // Our existing is a &mut therefore in order to change it's value we must dereference it here // Changing from single to multiple works because all the variants in an enum take up the same space
                }
                Value::Multiple(vec) => vec.push(val)
            })
            .or_insert(Value::Single(val));
        }

        QueryString {data} // data variable type assignment can be inferred due to this line
    }
}