#[derive(Debug, PartialEq)]
struct Money {
    amount: i32,
}

impl Money {
    fn new(amount: i32) -> Self {
        Money { amount }
    }
}

#[derive(Debug, PartialEq)]
struct Doller {
    money: Money,
}

impl Doller {
    fn new(amount: i32) -> Self {
        Doller {
            money: Money::new(amount),
        }
    }

    fn times(&self, multiplier: i32) -> Self {
        Doller {
            money: Money::new(self.money.amount * multiplier),
        }
    }

    fn equals(&self, doller: &Doller) -> bool {
        self.money.amount == doller.money.amount
    }
}

#[derive(Debug, PartialEq)]
struct Franc {
    amount: i32,
}

impl Franc {
    fn new(amount: i32) -> Franc {
        Franc { amount }
    }

    fn times(&self, multiplier: i32) -> Franc {
        Franc {
            amount: self.amount * multiplier,
        }
    }

    fn equals(&self, franc: &Franc) -> bool {
        self.amount == franc.amount
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
    }
}
