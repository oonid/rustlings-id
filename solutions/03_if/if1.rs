fn bigger(a: i32, b: i32) -> i32 {
    // Menggunakan ekspresi `if` sebagai nilai kembalian.
    // Jika `a` lebih besar dari `b`, maka kembalikan `a`.
    // Jika tidak, maka kembalikan `b` (termasuk saat nilainya sama).
    if a > b {
        a
    } else {
        b
    }
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
