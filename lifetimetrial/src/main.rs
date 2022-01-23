
#[derive(Debug, Clone)]
struct Address {
    housename: String,
    street: String,
}
#[derive(Debug)]
struct Employee {
    name: String,
    home_address: Address,
    comm_address: Option<Address>,
    order: Vec<Product>,
    amount: i32,
}
#[derive(Debug)]
struct Product {
    product_type: String,
    price: i32,
}

fn main() {
    let first_prod = Product {
        product_type: String::from("car"),
        price: 30,
    };
    let second_prod = Product {
        product_type: String::from("bus"),
        price: 50,
    };
    let emp_addr: Address = Address {
        housename: String::from("sunu"),
        street: String::from("stret"),
    };
    let mut emp: Employee = Employee {
        name: String::from("sunu"),
        home_address: emp_addr,
        comm_address: None,
        order: vec![first_prod, second_prod],
        amount: 0,
    };
    let comm_addr: Address = Address {
        housename: String::from("sunu"),
        street: String::from("stret"),
    };
    let comm_addr1: Address = Address {
        housename: String::from("sunu1"),
        street: String::from("stret1"),
    };
    println!("{:?}", emp.name);

    calculate_amount(&mut emp, comm_addr, &comm_addr1);
    println!("{:?}", emp.name);
    println!("{:?}", emp.comm_address);
    println!("{:?}", emp);


    let third = compare(1, 2);
    println!("{:?}", third);

    let third_ref = compare_reference(&1, &2);
    println!("{:?}", third_ref);

}

fn calculate_amount(emp: &mut Employee, comm_addr: Address, comm_addr_ref: &Address) {
    let total_amount: i32 = emp.order.iter().map(|f| f.price).sum();
    emp.amount = total_amount;
    emp.name = String::from("change");
    emp.comm_address = Option::Some(comm_addr);
    let comm_addr_ref_clone = comm_addr_ref.clone();
    emp.comm_address = Option::Some(comm_addr_ref_clone);

    emp.order.iter().for_each(|f| println!("{:?}", f.product_type));

    println!("{:?}", comm_addr_ref.housename);
    println!("{:?}", emp.home_address);
    println!("{:?}", comm_addr_ref.street);



    println!("{:?}", emp.amount);
}
fn compare(first: i32, second: i32) -> i32 {
    if first > second {
        first
    } else {
        second
    }
}

fn compare_reference<'a>(first: &'a i32, second: &'a i32) -> &'a i32 {
    if first > second {
        first
    } else {
        second
    }
}
