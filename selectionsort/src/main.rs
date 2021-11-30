fn main() {
    let mut arr: [i32; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 67, 8, 9, 1, 2];
    display(arr);

    let mut j: usize = 0;


    while j < arr.len() {
        let mut i: usize = j+1;
        let mut selected = arr[j];
        let mut selectindex: usize =j;

        while i < arr.len() {
            if arr[j] < arr[i] {
                arr[j] = arr[i];
                selectindex = i;
            }

            i = i + 1;
        }
        arr[selectindex] = selected;
        println!("");
        display(arr);
        j = j + 1;
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
