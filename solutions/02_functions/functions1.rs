// Sebuah fungsi dengan nama `call_me` tanpa argumen dan tanpa nilai kembalian.
//
// Penjelasan:
// - Fungsi dideklarasikan menggunakan kata kunci `fn`, diikuti dengan nama fungsi dan tanda kurung `()`.
// - Karena fungsi ini tidak menerima argumen, tanda kurungnya kosong.
// - Fungsi ini juga tidak memiliki nilai kembalian, sehingga tidak perlu menuliskan `-> T` atau tipe setelah tanda panah.
// - Nilai kembalian hanya diperlukan jika fungsi dimaksudkan untuk menghasilkan suatu nilai (contohnya `-> i32`).
// - Fungsi ini hanya mencetak teks ke layar menggunakan `println!`.
fn call_me() {
    println!("Hello world!");
}

fn main() {
    call_me();
}
