fn main () = {
    let ints = [1,2,3,4,5,6];
    let slice = &ints;

    for s in slice.window(2) {
        println!("window {:?}", s);
    }
} 
