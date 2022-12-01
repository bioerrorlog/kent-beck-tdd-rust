struct Doller {
    amount: i32,
}

impl Doller {
    fn new(amount: i32) -> Doller {
        Doller { amount }
    }

    fn times(&self, multiplier: i32) -> Doller {
        Doller {
            amount: self.amount * multiplier,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplication() {
        let five = Doller::new(5);
        let product = five.times(2);
        assert_eq!(10, product.amount);
        let product = five.times(3);
        assert_eq!(15, product.amount);
    }
}
