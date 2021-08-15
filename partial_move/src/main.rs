#[derive(Debug)]
struct Deal {
    deal_name: String,
    amount: u32,
    desi: String,
}

fn main() {
    println!("Hello, world!");
    let deal = Deal {
        deal_name: String::from("testdealname"),
        amount: 123,
        desi: String::from("testdesiname"),
    };

    let Deal { deal_name, amount, ref desi } = deal;

    println!("{}", desi);
    println!("{:?}", deal.desi);
    //println!("{:?}", deal.deal_name);///value moved

}
