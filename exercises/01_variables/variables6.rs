// TODO: Ubah baris di bawah ini untuk memperbaiki error dari kompiler.
// Penjelasan:
// Rust mewajibkan setiap konstanta (`const`) untuk memiliki anotasi tipe secara eksplisit.
// Berbeda dengan variabel biasa (`let`) yang bisa menggunakan tipe yang disimpulkan,
// konstanta harus selalu diberi tahu tipe nilainya secara langsung.
//
// Petunjuk:
// Tambahkan anotasi tipe (misalnya `i32`, `u32`, dll.) setelah nama konstanta,
// agar kompiler dapat mengenali tipe data yang digunakan.
const NUMBER = 3;

fn main() {
    println!("Angka: {NUMBER}");
}
