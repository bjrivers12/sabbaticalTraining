class Journal {
    constructor() {
        this.entries = {};
    }

    addEntry(text) {
        let c = ++Journal.count;
        let entry = `${c}: ${text}`l
        this.entries[c] = entry;
        return c;
    }

    removeEntry(index) {
        delete this.entries[index];
    }

    toString() {
        return Object.values(this.entries).join(`\n`)
    }

}
module.exports = Journal;