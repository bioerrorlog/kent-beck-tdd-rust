#[derive(Debug, PartialEq)]
struct Money {
    amount: i32,
    currency: Currency,
}

impl Money {
    fn new(amount: i32, currency: Currency) -> Money {
        Money { amount, currency }
    }

    fn times(&self, multiplier: i32) -> Money {
        Money::new(self.amount * multiplier, self.currency)
    }

    fn equals(&self, money: &Money) -> bool {
        self.amount == money.amount && self.currency == money.currency
    }

    fn plus(&self, addend: &Money) -> Money{
        Money::new(self.amount + addend.amount, self.currency)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Currency {
    Doller,
    Franc,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplication() {
        let five = Money::new(5, Currency::Doller);
        assert_eq!(Money::new(10, Currency::Doller), five.times(2));
        assert_eq!(Money::new(15, Currency::Doller), five.times(3));
    }

    #[test]
    fn equality() {
        assert!(Money::new(5, Currency::Doller).equals(&Money::new(5, Currency::Doller)));
        assert!(!Money::new(5, Currency::Doller).equals(&Money::new(6, Currency::Doller)));
        assert!(!Money::new(5, Currency::Franc).equals(&Money::new(5, Currency::Doller)));
        assert!(!Money::new(5, Currency::Doller).equals(&Money::new(5, Currency::Franc)));
    }

    #[test]
    fn currency_type() {
        assert_eq!(Currency::Doller, Money::new(5, Currency::Doller).currency);
        assert_eq!(Currency::Franc, Money::new(5, Currency::Franc).currency);
    }

    #[test]
    fn simple_addition() {
        let sum = Money::new(5, Currency::Doller).plus(&Money::new(5, Currency::Doller));
        assert_eq!(Money::new(10, Currency::Doller), sum);
    }
}
