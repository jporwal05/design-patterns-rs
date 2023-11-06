fn main() {
    // let payment_processor = PaymentProcessor::new(Box::new(CreditCard::new()));
    let payment_processor = PaymentProcessor::new(Box::new(Wallet::new()));
    println!(
        "Total amount after processing fee is: {}",
        payment_processor.process(100.0)
    );
}

/// Payment processor utilizing a mode of payment, [PaymentMode] to process a payment.
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

/// This is a strategy. In this example's context it is simply a mode of payment using which
/// one can process a payment or pay.
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

/// This is an implementation of [PaymentMode]. In this example's context it is a credit card.
impl PaymentMode for CreditCard {
    fn process(self: &CreditCard, amount: f32) -> f32 {
        amount + ((self.processing_fee / 100.0) * amount)
    }
}

/// This is an implementation of [PaymentMode]. In this example's context it is an e-wallet.
impl PaymentMode for Wallet {
    fn process(&self, amount: f32) -> f32 {
        amount + ((self.processing_fee / 100.0) * amount)
    }
}
