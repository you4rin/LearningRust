use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32>{
    let mut basket = HashMap::new();

    basket.insert(String::from("banana"), 2);
    basket.insert(String::from("melon"), 1);
    basket.insert(String::from("plum"), 42);

    basket
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits(){
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits(){
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}