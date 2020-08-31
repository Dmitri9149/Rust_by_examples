// slice of window the window has type &[{integer}]

fn main() {
    let ints = [1, 2, 3, 4, 5, 6, 7];
    let slice = &ints;

    for s in slice.windows(2) {
        println!("window {:?}", s);
// check the type of the iter 
//         let () = s
    }

    for s in slice.chunks(2) {
        println!("chunks {:?}",s);
    }
}
