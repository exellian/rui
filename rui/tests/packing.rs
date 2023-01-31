use rui::util::{pack, unpack};

fn main() {
    for i in 0..1024 {
        let x = pack(i, i + 1);
        let (left, right) = unpack(x);
        assert!(left == i && right == i + 1);
    }
    println!("Packing test succeeded!");
}
