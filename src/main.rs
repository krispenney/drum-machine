mod sequence;

fn main() {
    let pattern: [u32; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let sequence = sequence::Sequence::new(pattern);

    println!("{:?}", sequence);

    for v in sequence.into_iter() {
        println!("{:?}", v);
    }

    for v in sequence.into_iter() {
        println!("{:?}", v);
    }
}
