#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    use crate::dcim;

    #[test]
    fn test_module() {
        assert_eq!("get some devices", dcim::devices::get());
    }
}

pub mod dcim {
    pub mod devices {
        pub fn get() -> &'static str {
            "get some devices"
        }
    }
}
