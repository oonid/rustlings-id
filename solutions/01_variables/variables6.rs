// Tipe data untuk konstanta wajib selalu dituliskan secara eksplisit.
// Berbeda dengan variabel biasa (`let`) yang bisa menggunakan tipe hasil inferensi,
// konstanta (`const`) di Rust mengharuskan adanya anotasi tipe.
//
// Selain itu, konstanta selalu bersifat tidak dapat diubah (immutable),
// dan nilainya harus ditentukan saat deklarasi.
// Konstanta juga dapat digunakan di seluruh program selama berada dalam cakupan (scope).
const NUMBER: u64 = 3;

fn main() {
    println!("Angka: {NUMBER}");
}
