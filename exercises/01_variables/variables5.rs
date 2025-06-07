fn main() {
    let number = "T-H-R-E-E"; // Jangan ubah baris ini
    println!("Ejaan angka: {number}");

    // TODO: Perbaiki error dari kompiler dengan mengubah baris di bawah ini
    // tanpa mengganti nama variabel.
    // Penjelasan:
    // Variabel `number` awalnya menyimpan nilai bertipe string (&str).
    // Namun, baris berikut mencoba mengganti nilainya dengan angka (tipe integer),
    // sehingga terjadi kesalahan karena Rust tidak mengizinkan perubahan tipe
    // pada variabel yang sudah dideklarasikan dengan tipe tertentu.
    //
    // Petunjuk:
    // Untuk mengganti nilai dengan tipe berbeda tanpa mengganti nama variabel,
    // kamu bisa menggunakan teknik yang disebut "shadowing" —
    // yaitu mendeklarasikan ulang variabel dengan nama yang sama.
    number = 3;
    println!("Angka ditambah dua adalah: {}", number + 2);
}
