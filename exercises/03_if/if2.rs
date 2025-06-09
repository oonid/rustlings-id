// TODO: Perbaiki error dari kompiler pada fungsi ini.

fn picky_eater(food: &str) -> &str {
    if food == "strawberry" {
        "Yummy!"
    } else {
        1
    }
}

fn main() {
    // Kamu boleh bereksperimen di sini (opsional).
}

// TODO: Baca bagian pengujian (tests) untuk memahami perilaku yang diharapkan.
// Buat semua pengujian lulus tanpa mengubah isi test-nya.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yummy_food() {
        // Artinya, memanggil `picky_eater("strawberry")` harus menghasilkan "Yummy!".
        assert_eq!(picky_eater("strawberry"), "Yummy!");
    }

    #[test]
    fn neutral_food() {
        assert_eq!(picky_eater("potato"), "I guess I can eat that.");
    }

    #[test]
    fn default_disliked_food() {
        assert_eq!(picky_eater("broccoli"), "No thanks!");
        assert_eq!(picky_eater("gummy bears"), "No thanks!");
        assert_eq!(picky_eater("literally anything"), "No thanks!");
    }
}

// Penjelasan:
// - Fungsi `picky_eater` mengembalikan nilai bertipe `&str`, yaitu string literal.
// - Namun, blok `else` di dalam fungsi mengembalikan angka (`1`), yang tidak sesuai dengan tipe pengembalian.
// - Selain itu, fungsi belum menangani semua kemungkinan masukan (`potato`, `broccoli`, dll.).
//
// Petunjuk:
// - Pastikan semua jalur percabangan `if` dan `else` menghasilkan tipe yang sama, yaitu `&str`.
// - Gunakan percabangan `else if` untuk menangani kondisi tambahan seperti `"potato"`.
// - Kembalikan `"No thanks!"` sebagai nilai default untuk masukan lainnya.
