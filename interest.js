class InterestCalculator {
    constructor() {
        this.details = {};
        this.monthlyInterest = [];
    }

    setInterest(interest) {
        let apr = interest;
        this.details['apr'] = apr;
    }

    setPrincipal(principal) {
        let originalLoan = principal;
        this.details['originalLoan'] = originalLoan;
    }

    setTerm(months) {
        let term = months;
        this.details['months'] = term;
    }

    calculateInterest() {

    }


    generateAmoritization() {

    }
}