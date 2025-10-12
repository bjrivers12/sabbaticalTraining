use crate::payments::PaymentEntry;
use clap::Parser;
mod loan;
mod payments;
use loan::simulate_payoff;

#[derive(Parser, Debug)]
#[command(author, version, about = "Loan payoff simulator")]
struct Args {
    #[arg(short = 'P', long)]
    principal: f64,
    #[arg(short, long)]
    apr: f64,
    #[arg(short = 'p', long)]
    payment: f64,
    #[arg(short = 'x', long, default_value_t = 0.0)]
    extra: f64,
    #[arg(short, long, default_value_t = 30)]
    days: usize,
    #[arg(long, default_value_t = false)]
    full: bool,
}

fn main() {
    let args = Args::parse();

    let principal = args.principal;
    let apr = args.apr / 100.0;
    let base_payment = args.payment;
    let extra_payment = args.extra;
    let days_per_payment = args.days;

    let max_payments = 1000;

    let (schedule, total_interest, payments, days) = simulate_payoff(
        principal,
        apr,
        base_payment,
        extra_payment,
        days_per_payment,
        max_payments,
    );

    if args.full {
        for entry in &schedule {
            println!("{}", entry);
        }
    } else {
        for entry in schedule.iter().take(10) {
            println!("{}", entry);
        }
        if schedule.len() > 10 {
            println!("... ({} more payments)", schedule.len() - 10);
        }
    }

    println!(
        "\nPaid off in {} payments ({:.1} years / {} days). Total interest paid: ${:.2}",
        payments,
        days as f64 / 365.0,
        days,
        total_interest
    );
}
