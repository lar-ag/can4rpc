extern "C" {
    pub fn read_ain() -> i32;
}


#[cfg(test)]
mod tests {
    use super::*;

     #[test]
    fn test_can() {
        println!("read_ain()={}",read_ain());

    }
}
