// Penjelasan singkat (komentar):
// - Closure di Rust adalah fungsi anonim yang dapat disimpan ke variabel.
// - Contoh sederhana:
//     let add_two = |x: i32| x + 2;
//   Di sini `|x: i32| x + 2` adalah closure anonim yang mengambil i32 lalu menambahkan 2.
//
// Soal: Ubah kode di bawah sehingga `square` bukan lagi fungsi `fn` terpisah,
//       melainkan sebuah closure anonim yang disimpan ke variabel `square`.
//       Jangan ubah cara pemanggilan (`square(3)`) atau `println!`.

fn main() {
    // Contoh closure lain:
    // let add_two = |x: i32| x + 2;

    // TODO: Buat closure anonim bernama `square` di sini,
    //       mirip dengan `add_two` di atas, tapi melakukan `num * num`.
    let square = unimplemented!();

    let answer = square(3);
    println!("Kuadrat dari 3 adalah {answer}");
}
