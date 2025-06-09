fn main() {
    // TODO: Buat array bernama `a` yang memiliki setidaknya 100 elemen.

    // Petunjuk:
    // - Di Rust, kamu bisa menggunakan notasi `[nilai; jumlah]` untuk membuat array besar dengan elemen yang sama.
    //   Misalnya: `[1; 3]` artinya array `[1, 1, 1]`.
    // - Fungsi `.len()` mengembalikan panjang array.
    // - Coba buat array dengan 100 atau lebih elemen.

    // let a = ???;

    if a.len() >= 100 {
        println!("Wow, array-nya besar sekali!");
    } else {
        println!("Halah, array segitu mah sarapan saya.");
        panic!("Array tidak cukup besar, perlu lebih banyak elemen");
    }
}
