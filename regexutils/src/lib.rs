pub trait ExtractCaptured {
    fn extract_captured<T: std::str::FromStr>(&self, group_name: &str) -> T
    where
        T::Err: std::fmt::Debug;
}

impl ExtractCaptured for regex::Captures<'_> {
    fn extract_captured<T: std::str::FromStr>(&self, group_name: &str) -> T
    where
        T::Err: std::fmt::Debug,
    {
        self.name(group_name)
            .unwrap_or_else(|| panic!("No {}!", group_name))
            .as_str()
            .parse::<T>()
            .unwrap_or_else(|_| panic!("Couldn't parse {}!", group_name))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
