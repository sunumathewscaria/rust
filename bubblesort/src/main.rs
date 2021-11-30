fn main() {
    let mut arr: [i32; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 67, 8, 9, 1, 2];
    display(arr);

    let mut j: usize = arr.len();


    while j > 0 {
        let mut i: usize = 0;

        while i < j - 1 {
            if arr[i] < arr[i + 1] {
                let tmp: i32 = arr[i];
                arr[i] = arr[i + 1];
                arr[i + 1] = tmp;
            }
            //println!("");
            //display(arr);
            i = i + 1;
        }
        j = j - 1;
    }
    //println!("#################");
    println!("");

    display(arr);
}

fn display(arr: [i32; 20]) {
    let mut i: usize = 0;
    while i < arr.len() {
        print!("{} ", arr[i]);
        i = i + 1;
    }
}
