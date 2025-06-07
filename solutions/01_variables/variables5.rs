fn main() {
    let number = "T-H-R-E-E";
    println!("Ejaan angka: {number}");

    // Menggunakan teknik shadowing (penimpaan variabel).
    // Shadowing memungkinkan kita mendeklarasikan ulang variabel dengan nama yang sama,
    // tetapi dengan tipe atau nilai yang berbeda.
    // Variabel sebelumnya akan "ditimpa", dan yang aktif adalah variabel terbaru.
    //
    // Ini berbeda dengan penggunaan `mut`, karena shadowing memungkinkan kita
    // mengubah tipe data, sementara `mut` hanya mengubah nilai dengan tipe yang sama.
    //
    // Dokumentasi terkait:
    // https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing
    let number = 3;
    println!("Angka ditambah dua adalah: {}", number + 2);
}
