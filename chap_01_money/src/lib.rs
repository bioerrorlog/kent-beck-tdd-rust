struct Doller {
    amount: i32,
}

impl Doller {
    fn new(amount: i32) -> Doller {
        Doller {
            amount,
        }
    }

    fn times(&self, multiplier: i32) -> Self{
        Doller::new(self.amount * multiplier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplication() {
        let five = Doller::new(5);
        let ten = five.times(2);
        assert_eq!(10, ten.amount);
    }
}
