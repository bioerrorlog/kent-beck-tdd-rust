trait Money {
    fn new(amount: i32) -> Self;

    fn amount(&self) -> i32;

    fn times(&self, multiplier: i32) -> Self;

    fn equals(&self, money: &impl Money) -> bool;
}

#[derive(Debug, PartialEq)]
struct Doller {
    amount: i32,
}

impl Money for Doller {
    fn new(amount: i32) -> Self {
        Doller { amount }
    }

    fn amount(&self) -> i32 {
        self.amount
    }

    fn times(&self, multiplier: i32) -> Self {
        Doller::new(self.amount * multiplier)
    }

    fn equals(&self, money: &impl Money) -> bool {
        self.amount == money.amount()
    }
}

#[derive(Debug, PartialEq)]
struct Franc {
    amount: i32,
}

impl Money for Franc {
    fn new(amount: i32) -> Self {
        Franc { amount }
    }

    fn amount(&self) -> i32 {
        self.amount
    }

    fn times(&self, multiplier: i32) -> Self {
        Franc::new(self.amount * multiplier)
    }

    fn equals(&self, money: &impl Money) -> bool {
        self.amount == money.amount()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplication() {
        let five = Doller::new(5);
        assert_eq!(Doller::new(10), five.times(2));
        assert_eq!(Doller::new(15), five.times(3));
    }

    #[test]
    fn franc_multiplication() {
        let five = Franc::new(5);
        assert_eq!(Franc::new(10), five.times(2));
        assert_eq!(Franc::new(15), five.times(3));
    }

    #[test]
    fn equality() {
        assert!(Doller::new(5).equals(&Doller::new(5)));
        assert!(!Doller::new(5).equals(&Doller::new(6)));
        assert!(Franc::new(5).equals(&Franc::new(5)));
        assert!(!Franc::new(5).equals(&Franc::new(6)));
    }
}
