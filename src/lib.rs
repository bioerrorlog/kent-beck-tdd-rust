struct Doller {
    amount: i32,
}

impl Doller {
    fn new(amount: i32) -> Doller {
        Doller { amount }
    }

    fn times(&mut self, multiplier: i32) {
        self.amount *= multiplier;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplication() {
        let mut five = Doller::new(5);
        five.times(2);
        assert_eq!(10, five.amount);
    }
}
