// Toko ini sedang mengadakan diskon: jika harga merupakan bilangan genap, kamu mendapatkan
// potongan 10 Rustbuck, dan jika ganjil, potongannya 3 Rustbuck.
// Jangan khawatir soal isi fungsi — fokus latihan ini adalah pada penulisan _signature_ fungsi.

fn is_even(num: i64) -> bool {
    num % 2 == 0
}

// TODO: Perbaiki _signature_ fungsi `sale_price`.

fn sale_price(price: i64) -> {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn main() {
    let original_price = 51;
    println!("Harga setelah diskon adalah {}", sale_price(original_price));
}

// Penjelasan:
// Setiap fungsi di Rust yang mengembalikan nilai harus menyertakan anotasi tipe nilai kembalian.
// Fungsi `sale_price` menghasilkan nilai berupa bilangan bulat (hasil pengurangan),
// tetapi tipe nilainya belum ditentukan di bagian _signature_ fungsi.
//
// Petunjuk:
// Tambahkan tipe nilai kembalian (`i64`) setelah tanda panah `->` pada deklarasi fungsi `sale_price`.
