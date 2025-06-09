fn is_even(num: i64) -> bool {
    num % 2 == 0
}

// Tipe nilai kembalian wajib dituliskan secara eksplisit jika fungsi mengembalikan nilai.
// Di sini, `sale_price` mengembalikan bilangan bulat bertipe `i64`, sesuai hasil perhitungan.
fn sale_price(price: i64) -> i64 {
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
