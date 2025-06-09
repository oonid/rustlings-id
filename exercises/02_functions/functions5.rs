// TODO: Perbaiki isi fungsi tanpa mengubah bagian tanda tangan (signature) fungsi.

fn square(num: i32) -> i32 {
    num * num;
}

fn main() {
    let answer = square(3);
    println!("Kuadrat dari 3 adalah {answer}");
}

// Penjelasan:
// Fungsi `square` seharusnya mengembalikan hasil perkalian `num * num` sebagai nilai kembalian.
// Namun, Rust membedakan antara ekspresi (expression) dan pernyataan (statement).
// Baris `num * num;` diakhiri dengan titik koma, sehingga dianggap sebagai statement
// dan hasilnya tidak dikembalikan (mengembalikan unit `()` secara implisit).
//
// Petunjuk:
// Ada dua cara untuk memperbaiki ini:
// 1. Menghapus titik koma di akhir baris `num * num`, agar nilainya dikembalikan secara implisit.
// 2. Atau, menambahkan kata kunci `return` sebelum ekspresi dan menutupnya dengan titik koma.
