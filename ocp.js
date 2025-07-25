let Color = Object.freeze({
    red: 'red',
    green: 'green',
    blue: 'blue'
});

let Size = Object.freeze({
    small: 'small',
    medium: 'medium',
    large: 'large'
})

class Product {
    constructor(name, color, size) {
        this.name = name;
        this.color = color;
        this.size = size
    }
}

//open for extension, closed for modification

class ProductFilter {
    filrterByColor(products, color) {
        return products.filter(p => p.color === color);
    }

    filterBySize(products, size) {
        return products.filter(p => p.size === size);

    }

    //state space explosion

}

//specification

class ColorSpecification {
    constructor(color) {
        this.color = color;
    }
    isSatisfided(item) {
        return item.color === this.color;
    }
}

class SizeSpecification {
    constructor(size) {
        this.size = size;
    }
    isSatisfided(item) {
        return item.size === this.size;
    }
}

class BetterFilter {
    filter(items, spec) {
        return items.filter(x => spec.isSatisfided(x));
    }
}

class AndSpecification {
    constructor(...specs) {
        this.specs = specs;
    }

    isSatisfided(item) {
        return this.specs.every(x => x.isSatisfided(item))
    }
}

let apple = new Product('Apple', Color.green, Size.small);
let tree = new Product('Tree', Color.green, Size.large);
let house = new Product('House', Color.blue, Size.large);

let products = [apple, tree, house];
let pf = new ProductFilter()

console.log(`Green products (old):`);
for (let p of pf.filrterByColor(products, Color.green))
    console.log(` * ${p.name} is green`)
let bf = new BetterFilter()
console.log("better")
for (let p of bf.filter(products,
    new ColorSpecification(Color.green))) {
    console.log(` * ${p.name} is green`)
}
console.log(`Large and green`)
let spec = new AndSpecification(
    new ColorSpecification(Color.green),
    new SizeSpecification(Size.large)
);
for (let p of bf.filter(products, spec)) {
    console.log(` * ${p.name} is large and green`)
}