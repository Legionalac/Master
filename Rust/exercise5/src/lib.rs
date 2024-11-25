pub fn is_even(a:i32)-> bool {
    if a%2==0{
         return true;
    }  return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_even(){
        let even_numbers = vec![-10, -2, 2, 10, 100];
        for num in even_numbers{
            assert_eq!(is_even(num), true);
        }
    }

    #[test]
    fn test_is_not_even(){
        let uneven_numbers = vec![-7, -3, 1, 3, 5];
        for num in uneven_numbers{
            assert_eq!(is_even(num), false);
        }
    }

    #[test]
    #[should_panic]
    fn test_is_even_panic(){
        assert!(is_even(4) == false)
    }
}
