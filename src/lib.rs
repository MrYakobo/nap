macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

mod main;

#[cfg(test)]
mod tests {
    use crate::main;

    #[test]
    fn parse_valid_strings() {
        let input_output = map! {
            "1h" => 3600.0,
            "44s" => 44.0,
            ".01" => 0.01,
            "365d" => 31536000.0,
            "2m" => 120.0,
            ".5m" => 30.0,
            "100" => 100.0
        };

        for (k, v) in input_output {
            assert_eq!(main::parse(k).unwrap(), v);
        }
    }

    #[test]
    fn parse_invalid_strings() {
        for v in vec!["", ".", "infinity", "abc", "0x00"] {
            assert_eq!(main::parse(v).is_none(), true);
        }
    }
}
