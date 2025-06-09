fn main() {
    // Kamu boleh bereksperimen di sini (opsional).
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: Ambil potongan (slice) dari array `a` dan simpan dalam variabel `nice_slice`
        // agar test `assert_eq!` di bawah ini berhasil.
        //
        // Penjelasan:
        // - Di Rust, kamu bisa mengambil bagian dari array menggunakan sintaks slice: `&a[start..end]`
        // - Batas akhir (`end`) bersifat *exclusive* — artinya indeks tersebut tidak disertakan dalam slice.
        //
        // Petunjuk:
        // - Ambil elemen 2, 3, dan 4 dari array `a` menggunakan slice.
        // - Perhatikan bahwa elemen ke-2 berada di indeks 1, dan elemen ke-4 di indeks 3.
        // - Jadi kamu perlu mengambil dari indeks 1 sampai 4 (tidak termasuk 4).

        // let nice_slice = ???;

        assert_eq!([2, 3, 4], nice_slice);
    }
}
