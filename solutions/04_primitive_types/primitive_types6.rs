fn main() {
    // Kamu boleh bereksperimen di sini (opsional).
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);

        // Mengakses elemen kedua menggunakan indeks tuple `.1`.
        let second = numbers.1;

        assert_eq!(second, 2, "Ini bukan elemen ke-2 dari tuple!");
    }
}

/*
Penjelasan:
- Dengan `numbers.1`, kita mengambil elemen pada indeks ke-1 dari tuple `numbers`.
- Ingat bahwa indeks di Rust mulai dari 0, jadi:
  - `numbers.0` = 1
  - `numbers.1` = 2
  - `numbers.2` = 3
*/
