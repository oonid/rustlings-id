fn call_me(num: u8) {
    for i in 0..num {
        println!("Dering! Panggilan ke-{}", i + 1);
    }
}

fn main() {
    // Fungsi `call_me` membutuhkan satu argumen bertipe `u8`.
    // Di sini kita memberikan nilai `5` sebagai argumen saat memanggilnya.
    call_me(5);
}
