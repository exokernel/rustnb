#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod dcim {
    mod devices {
        fn get() {
            println!("get some devices");
        }
    }
}
