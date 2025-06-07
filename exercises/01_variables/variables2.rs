fn main() {
    // TODO: Ubah baris di bawah ini untuk memperbaiki error dari kompiler.
    // Rust tidak mengizinkan penggunaan variabel yang belum memiliki nilai.
    // Variabel `x` sudah dideklarasikan, tetapi belum diinisialisasi (belum diberi nilai).
    // Kompiler akan menolak kode seperti ini karena dapat menyebabkan perilaku yang tidak terduga.
    // Petunjuk: berikan nilai awal pada `x` saat deklarasi.
    let x;

    if x == 10 {
        println!("x bernilai sepuluh!");
    } else {
        println!("x tidak bernilai sepuluh!");
    }
}
