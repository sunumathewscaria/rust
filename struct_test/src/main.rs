use std::fmt::Debug;

#[derive(Debug)]
struct Foo {
    x: (u32, u32),
    y: u32,
}

#[derive(Debug)]
struct Deal {
    dealName: String,
    amount: u32,
    desi: String,
}


fn main() {
    let foo = Foo { x: (1, 2), y: 3 };

    println!("foo object is {:?}", foo);

    match foo {
        Foo { x: (1, 2), y: _a } => println!("x match matched Foo {:?}", foo),
        Foo { .. } => println!("anymatch matched Foo {:?}", foo)
    }
    display(foo as Foo);

    let mut deal = Deal {
        dealName: String::from("testDealName"),
        amount: 123,
        desi: String::from("testDesi"),
    };
    displayDeal(&deal);
    changeDesi(&mut deal);
    displayDeal(deal);
    //displayDeal(deal);  -- deal goes out of scope on previous line


}

fn changeDesi(deal: &mut Deal) {
    deal.desi = String::from("testDesiChange");
}

fn display<T: Debug>(foo: T) {
    println!("{:?}", foo);
}

fn displayDeal<T: Debug>(foo: T) {
    println!("{:?}", foo);
}
