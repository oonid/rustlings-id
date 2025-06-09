fn call_me(num: u8) {
    for i in 0..num {
        println!("Dering! Panggilan ke-{}", i + 1);
    }
}

fn main() {
    // TODO: Perbaiki pemanggilan fungsi.

    // Penjelasan:
    // Fungsi `call_me` didefinisikan dengan satu parameter bertipe `u8`.
    // Artinya, saat fungsi dipanggil, kita perlu memberikan satu argumen berupa bilangan bulat.

    // Petunjuk:
    // Tambahkan sebuah nilai argumen (misalnya 3, 5, atau angka lain) ke dalam tanda kurung
    // agar pemanggilan fungsi sesuai dengan definisinya.
    call_me();
}
