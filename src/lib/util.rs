use std::collections::HashMap;

pub fn parse_kv_string(string: &str) -> HashMap<String, String>
{
    string
        .split("&")
        .map( |pair| {
            let kv: Vec<&str> = pair.split("=").collect();
            (String::from(kv[0]), String::from(kv[1]))
        })
        .collect()
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn parse_kv_string()
    {
        const STRING: &str = "foo=bar&baz=qux&zap=zazzle";
        let result = super::parse_kv_string(&STRING);
        let expected: HashMap<String, String> = 
            [
                (String::from("foo"), String::from("bar")),
                (String::from("baz"), String::from("qux")),
                (String::from("zap"), String::from("zazzle"))
            ].iter().cloned().collect();

        assert_eq!(expected, result);
    }
}