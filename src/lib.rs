#[derive(Debug, PartialEq)]
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

    fn equals(&self, doller: &Doller) -> bool {
        self.amount == doller.amount
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
    fn equality() {
        assert!(Doller::new(5).equals(&Doller::new(5)));
        assert!(!Doller::new(5).equals(&Doller::new(6)));
    }
}
