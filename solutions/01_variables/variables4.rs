fn main() {
    // Di Rust, variabel secara default bersifat tidak dapat diubah (immutable).
    // Artinya, setelah diberi nilai, variabel tersebut tidak bisa diubah lagi.
    //
    // Untuk membuat variabel bisa diubah (mutable), kita tambahkan kata kunci `mut`
    // setelah `let` saat deklarasi variabel.
    let mut x = 3;
    println!("Angka {x}");

    // Karena variabel `x` sudah dideklarasikan sebagai mutable,
    // maka kita bisa mengganti nilainya di baris ini tanpa menyebabkan error.
    x = 5;
    println!("Angka {x}");
}
