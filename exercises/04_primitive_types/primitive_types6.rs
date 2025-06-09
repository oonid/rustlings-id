fn main() {
    // Kamu boleh bereksperimen di sini (opsional).
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);

        // TODO: Gunakan indeks tuple untuk mengakses elemen kedua dari `numbers`
        // dan simpan ke dalam variabel bernama `second`.
        // let second = ???;

        assert_eq!(second, 2, "Ini bukan elemen ke-2 dari tuple!");
    }
}

/*
Penjelasan:
- Tuple di Rust dapat diakses menggunakan indeks, seperti `tuple.0`, `tuple.1`, dst.
- Dalam tuple `(1, 2, 3)`, elemen pertama adalah `1` (dengan indeks `.0`), elemen kedua adalah `2` (dengan indeks `.1`), dan seterusnya.

Petunjuk:
- Gunakan sintaks `numbers.1` untuk mendapatkan elemen kedua dari tuple.
- Pastikan nilai tersebut disimpan ke dalam variabel `second` agar pengujian (`test`) lulus.

Referensi:
https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type
*/
