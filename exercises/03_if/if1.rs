fn bigger(a: i32, b: i32) -> i32 {
    // TODO: Lengkapi fungsi ini untuk mengembalikan angka yang lebih besar!
    //
    // Jika kedua angka bernilai sama, kamu boleh mengembalikan salah satunya.
    //
    // Petunjuk:
    // - Gunakan ekspresi `if` untuk membandingkan kedua nilai.
    // - Kamu tidak perlu (dan tidak boleh) menggunakan fungsi tambahan atau membuat variabel baru.
    //
    // Ingat:
    // - Ekspresi `if` di Rust bisa langsung menghasilkan nilai.
    //   Misalnya:
    //   let x = if kondisi { nilai1 } else { nilai2 };

    // Tulis solusi di sini.
}

fn main() {
    // Kamu boleh bereksperimen di sini (opsional).
}

// Abaikan bagian ini untuk sementara :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}
