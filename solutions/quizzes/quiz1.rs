// Mary sedang membeli apel. Harga sebuah apel dihitung sebagai berikut:
// - Satu apel seharga 2 rustbuck.
// - Namun, jika Mary membeli lebih dari 40 apel, maka harga tiap apel
//   (untuk seluruh pembelian) hanya 1 rustbuck!

fn calculate_price_of_apples(n_apples: u64) -> u64 {
    if n_apples > 40 {
        n_apples
    } else {
        2 * n_apples
    }
}

fn main() {
    // Kamu boleh bereksperimen di sini (opsional).
}

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
