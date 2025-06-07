// TODO: Perbaiki error dari kompiler.
fn main() {
    // Variabel `x` dideklarasikan sebagai tidak dapat diubah (immutable) secara default.
    // Itu berarti nilainya tidak boleh diubah setelah diberi nilai awal.
    // Namun di bawah ini, kita mencoba mengubah nilai `x`, sehingga kompiler akan memberikan error.
    // Petunjuk: agar `x` bisa diubah nilainya, perlu ada kata kunci tambahan saat deklarasi.
    let x = 3;
    println!("Angka {x}");

    // Baris ini tidak boleh diubah.
    x = 5;
    println!("Angka {x}");
}
