// Baris berikut menonaktifkan peringatan dari Clippy (alat pemeriksa kualitas kode).
// Dalam kasus ini, kita menonaktifkan peringatan `needless_late_init`, yaitu saran
// untuk tidak memisahkan deklarasi dan inisialisasi variabel jika tidak perlu.
// Hal ini dilakukan agar kita bisa menunjukkan bahwa Rust mendukung inisialisasi terlambat.
#![allow(clippy::needless_late_init)]

fn main() {
    // Membaca nilai dari variabel yang belum diinisialisasi tidak diperbolehkan di Rust.
    // Oleh karena itu, kita harus memberikan nilai terlebih dahulu sebelum digunakan.
    let x: i32 = 42;

    println!("Angka {x}");

    // Rust memperbolehkan kita mendeklarasikan variabel terlebih dahulu,
    // lalu menginisialisasikannya di baris berikutnya.
    // Namun, variabel tersebut TIDAK BOLEH digunakan sebelum diberi nilai.
    let y: i32;
    y = 42;
    println!("Angka {y}");

    // Catatan:
    // Praktik terbaik (best practice) di Rust adalah menginisialisasi variabel
    // langsung saat deklarasi, jika memungkinkan.
    // Contoh yang lebih disarankan:
    // let y: i32 = 42;
}
