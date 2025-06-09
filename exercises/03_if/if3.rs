fn animal_habitat(animal: &str) -> &str {
    // TODO: Perbaiki error dari kompiler pada pernyataan di bawah ini.

    let identifier = if animal == "crab" {
        1
    } else if animal == "gopher" {
        2.0
    } else if animal == "snake" {
        3
    } else {
        "Unknown"
    };

    // Jangan ubah ekspresi di bawah ini!
    if identifier == 1 {
        "Pantai"
    } else if identifier == 2 {
        "Liang"
    } else if identifier == 3 {
        "Gurun"
    } else {
        "Tidak diketahui"
    }
}

fn main() {
    // Kamu boleh bereksperimen di sini (opsional).
}

// Jangan ubah bagian pengujian berikut!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Liang")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Gurun")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Pantai")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Tidak diketahui")
    }
}

/*
Penjelasan:
- Rust mewajibkan semua cabang dari ekspresi `if` menghasilkan nilai dengan tipe yang konsisten.
- Dalam ekspresi `identifier`, beberapa cabang menghasilkan nilai bertipe integer (`i32`),
  tetapi satu cabang (`gopher`) menghasilkan angka desimal (`f64`), dan satu cabang lainnya string (`&str`).
- Ini menyebabkan kompiler gagal menyimpulkan satu tipe tetap.

Petunjuk:
- Samakan semua nilai yang dikembalikan oleh ekspresi `identifier`, misalnya semuanya bertipe integer.
- Kamu boleh menggunakan angka seperti 4 untuk mewakili hewan yang tidak dikenal,
  selama tidak mengganggu logika `if` di bagian bawah.
- Nantinya, pendekatan yang lebih idiomatik dalam Rust untuk kasus seperti ini adalah menggunakan `enum`,
  tapi materi tersebut akan dibahas di bagian terpisah.
*/
