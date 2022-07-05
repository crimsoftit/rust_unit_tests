mod vectors;
mod panik;


pub fn main() {

    println!("              ");
    println!("*** VECTORS ***");
    vectors::run_vectors();

    println!("              ");
    println!("*** PANIC ***");
    panik::test_panik(60);
}



#[cfg(test)]
mod tests {
    #[test]
    fn test_assert () {
        assert!(5 > 4);
    }

    #[test]
    fn test_assert_eq () {
        assert_eq!(4, 4);
    }

    #[test]
    fn test_assert_ne () {
        assert_ne!(3, 2);
    }
}
