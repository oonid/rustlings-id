fn main() {
    // Kamu boleh bereksperimen di sini (opsional).
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];
        //       0  1  2  3  4   <- indeks
        //          -------
        //             |
        //             +--- slice dari indeks 1 sampai sebelum 4

        // Mengambil slice yang berisi elemen ke-2, ke-3, dan ke-4.
        let nice_slice = &a[1..4];

        assert_eq!([2, 3, 4], nice_slice);

        // Catatan tambahan:
        // Jika ingin menyertakan batas akhir (inklusif), gunakan `..=`:
        let nice_slice = &a[1..=3];
        assert_eq!([2, 3, 4], nice_slice);
    }
}
