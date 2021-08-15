use std::fmt::Debug;

#[derive(Debug)]
struct Foo {
    x: (u32, u32),
    y: u32,
}

#[derive(Debug)]
struct Deal {
    deal_name: String,
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
        deal_name: String::from("testDealName"),
        amount: 123,
        desi: String::from("testDesi"),
    };
    display_deal(&deal);
    change_desi(&mut deal);
    display_deal(deal);
    //display_deal(deal);  -- deal goes out of scope on previous line


}

fn change_desi(deal: &mut Deal) {
    deal.desi = String::from("testDesiChange");
}

fn display<T: Debug>(foo: T) {
    println!("{:?}", foo);
}

fn display_deal<T: Debug>(foo: T) {
    println!("{:?}", foo);
}
