fn main() {
    // let payment_processor = PaymentProcessor::new(Box::new(CreditCard::new()));
    let payment_processor = PaymentProcessor::new(Box::new(Wallet::new()));
    println!("{}", payment_processor.process(100.0));
}

pub struct PaymentProcessor {
    pub payment_mode: Box<dyn PaymentMode>,
}

impl PaymentProcessor {
    pub fn new(payment_mode: Box<dyn PaymentMode>) -> PaymentProcessor {
        PaymentProcessor { payment_mode }
    }

    pub fn process(self: &PaymentProcessor, amount: f32) -> f32 {
        self.payment_mode.process(amount)
    }
}

pub trait PaymentMode {
    fn process(&self, amount: f32) -> f32;
}

pub struct CreditCard {
    pub processing_fee: f32,
}

impl CreditCard {
    pub fn new() -> CreditCard {
        CreditCard {
            processing_fee: 2.5,
        }
    }
}

pub struct Wallet {
    pub processing_fee: f32,
}

impl Wallet {
    pub fn new() -> Wallet {
        Wallet {
            processing_fee: 1.0,
        }
    }
}

impl PaymentMode for CreditCard {
    fn process(self: &CreditCard, amount: f32) -> f32 {
        amount - ((self.processing_fee / 100.0) * amount)
    }
}

impl PaymentMode for Wallet {
    fn process(&self, amount: f32) -> f32 {
        amount - ((self.processing_fee / 100.0) * amount)
    }
}
