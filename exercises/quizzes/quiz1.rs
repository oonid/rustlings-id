// Kuis ini mencakup topik-topik berikut:
// - Variabel
// - Fungsi
// - If
//
// Mary sedang membeli apel. Harga sebuah apel dihitung sebagai berikut:
// - Satu apel seharga 2 rustbuck.
// - Namun, jika Mary membeli lebih dari 40 apel, maka harga tiap apel
//   (untuk seluruh pembelian) hanya 1 rustbuck!

// TODO:
// Tulis sebuah fungsi bernama `calculate_price_of_apples` yang menerima jumlah
// apel yang dibeli, dan mengembalikan total harga yang harus dibayar.
//
// Signature fungsi dapat ditulis seperti:
// fn calculate_price_of_apples(...) -> ... { ... }
//
// Penjelasan:
// - Gunakan tipe data bilangan bulat tanpa tanda seperti `u64`
// - Gunakan ekspresi `if` untuk memeriksa apakah jumlahnya lebih dari 40
//
// Petunjuk:
// - Jika jumlahnya > 40, maka harga per apel = 1 rustbuck
// - Jika jumlahnya <= 40, maka harga per apel = 2 rustbuck
// - Total harga = jumlah apel × harga per apel

fn main() {
    // Kamu boleh bereksperimen di sini (opsional).
}

// Jangan ubah bagian pengujian berikut!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}
