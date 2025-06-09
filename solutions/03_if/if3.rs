fn animal_habitat(animal: &str) -> &str {
    // Semua nilai pada ekspresi `if` sekarang bertipe sama (integer).
    // Ini membuat Rust bisa menyimpulkan tipe `identifier` secara konsisten.
    let identifier = if animal == "crab" {
        1
    } else if animal == "gopher" {
        2
    } else if animal == "snake" {
        3
    } else {
        // Nilai yang tidak digunakan dalam kondisi di bawah, tapi tetap harus bertipe sama.
        4
    };

    // Penjelasan tambahan:
    // Di Rust, pendekatan yang lebih baik untuk kasus seperti ini adalah menggunakan `enum`.
    // Namun untuk latihan ini, kita hanya menggunakan nilai integer sebagai pengenal.

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
