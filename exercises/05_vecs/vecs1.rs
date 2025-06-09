// Vektor (`Vec<T>`) dan Array (`[T; N]`)

/// Penjelasan:
/// - Di Rust, array (`[i32; 4]`) memiliki ukuran tetap dan disimpan di stack.
/// - Vektor (`Vec<i32>`) dapat tumbuh dan menyusut secara dinamis, serta disimpan di heap.
/// - Vektor biasanya dibuat menggunakan makro `vec![]`.
///
/// Pada latihan ini, kamu diminta membuat sebuah vektor yang isinya **sama persis**
/// dengan array `a`, lalu mengembalikannya bersama array dalam bentuk tuple.
///
/// Petunjuk:
/// - Gunakan makro `vec![]` dan isi vektor dengan elemen yang sama seperti array.
/// - Tidak perlu menggunakan `Vec::new()` atau `.push()`, cukup gunakan `vec![10, 20, 30, 40]`.
/// - Dokumentasi vektor: https://doc.rust-lang.org/book/ch08-01-vectors.html

fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // Array

    // TODO: Buat vektor `v` yang berisi elemen sama dengan `a`.
    // Gunakan makro `vec![]`
    // let v = ???;

    (a, v)
}

fn main() {
    // Kamu boleh bereksperimen di sini (opsional).
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, *v);
    }
}
