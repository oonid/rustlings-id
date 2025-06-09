// TODO: Lengkapi fungsi `vec_loop` dan `vec_map` untuk mengalikan setiap elemen input dengan 2.
//
// Penjelasan:
// - Fungsi `vec_loop` menggunakan pendekatan imperatif (perulangan `for` dan `push` ke `Vec`).
// - Fungsi `vec_map` menggunakan pendekatan deklaratif (iterator dan `map`).
//
// Petunjuk:
// - Untuk `vec_loop`, kamu bisa menggunakan `for element in input` lalu `output.push(...)`.
// - Untuk `vec_map`, kamu bisa meniru pola pada `vec_map_example`, yaitu gunakan `iter().map(...).collect()`
// - `element` dalam `iter()` adalah referensi, jadi kamu bisa menulis `2 * element` karena tipe `i32` mendukung operasi ini langsung melalui `Copy`.

fn vec_loop(input: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();

    for element in input {
        // TODO: Kalikan `element` dengan 2 dan masukkan ke dalam vektor output.
    }

    output
}

fn vec_map_example(input: &[i32]) -> Vec<i32> {
    // Contoh mengumpulkan hasil map menjadi vektor.
    // Jika input adalah [1, 2, 3], maka hasilnya adalah [2, 3, 4].
    input.iter().map(|element| element + 1).collect()
}

fn vec_map(input: &[i32]) -> Vec<i32> {
    // TODO: Gunakan iter dan map untuk mengalikan setiap elemen dengan 2
    // Lihat contoh pada `vec_map_example`
    input
        .iter()
        .map(|element| {
            // ???
        })
        .collect()
}

fn main() {
    // Opsional: bereksperimen di sini.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_loop(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }

    #[test]
    fn test_vec_map_example() {
        let input = [1, 2, 3];
        let ans = vec_map_example(&input);
        assert_eq!(ans, [2, 3, 4]);
    }

    #[test]
    fn test_vec_map() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_map(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }
}
