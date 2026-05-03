// TODO: Add the missing type of the argument `num` after the colon `:`.
fn call_me(num: [i32; 1]) {
    for i in num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me([1]);
}
