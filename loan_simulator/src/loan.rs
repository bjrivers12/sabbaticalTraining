use crate::payments::PaymentEntry;

pub fn simulate_payoff(
    mut principal: f64,
    apr: f64,
    base_payment: f64,
    extra_payment: f64,
    days_per_payment: usize,
    max_payments: usize,
) -> (Vec<PaymentEntry>, f64, usize, usize) {
    let daily_rate = apr / 365.0;
    let mut schedule: Vec<PaymentEntry> = Vec::new();
    let mut payment_number = 0usize;
    let mut total_interest_paid = 0.0;
    let mut total_days = 0usize;

    if principal <= 0.0 {
        return (schedule, 0.0, 0, 0);
    }

    while principal > 0.0 && payment_number < max_payments {
        payment_number += 1;
        let balance_before = principal;

        for _day in 0..days_per_payment {
            let interest_for_day = principal * daily_rate;
            principal += interest_for_day;
            total_days += 1;
        }

        let interest_accrued = principal - balance_before;
        let mut payment_total = base_payment + extra_payment;

        if payment_total >= principal {
            payment_total = principal;
        }

        let interest_paid = interest_accrued.min(payment_total);
        let principal_paid = payment_total - interest_paid;
        principal -= payment_total;
        total_interest_paid += interest_paid;

        schedule.push(PaymentEntry {
            payment_number,
            days_in_period: days_per_payment,
            period_interest: interest_accrued,
            payment_amount: payment_total,
            principal_paid,
            remaining_balance: if principal < 0.0 { 0.0 } else { principal },
        });

        if principal.abs() < 1e-8 {
            principal = 0.0;
        }
    }

    (schedule, total_interest_paid, payment_number, total_days)
}
