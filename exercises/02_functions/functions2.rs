// TODO: Tambahkan tipe data yang hilang untuk argumen `num` setelah tanda titik dua `:`.
//
// Penjelasan:
// - Di Rust, setiap argumen fungsi wajib memiliki anotasi tipe yang eksplisit.
// - Tidak seperti variabel dengan `let` yang bisa menggunakan penyimpulan tipe,
//   parameter fungsi tidak bisa dibiarkan tanpa tipe data.
// - Pada fungsi `call_me`, parameter `num` belum memiliki tipe,
//   sehingga menyebabkan error saat dikompilasi.
//
// Petunjuk:
// - Tentukan tipe data bilangan bulat yang cocok, misalnya `u64` atau `i32`.
// - Perhatikan bahwa tipe yang dipilih harus mendukung perulangan `0..num`.

fn call_me(num:) {
    for i in 0..num {
        println!("Dering! Panggilan ke-{}", i + 1);
    }
}

fn main() {
    call_me(3);
}
