use std::fmt;

#[derive(Debug)]
pub struct PaymentEntry {
    pub payment_number: usize,
    pub days_in_period: usize,
    pub period_interest: f64,
    pub payment_amount: f64,
    pub principal_paid: f64,
    pub remaining_balance: f64,
}

impl fmt::Display for PaymentEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "#{:3} | {:2}d | interest: ${:10.2} | payment: ${:8.2} | principal: ${:9.2} | balance: ${:10.2}",
            self.payment_number,
            self.days_in_period,
            self.period_interest,
            self.payment_amount,
            self.principal_paid,
            self.remaining_balance
        )
    }
}
