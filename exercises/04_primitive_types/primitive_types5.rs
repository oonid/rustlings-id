fn main() {
    let cat = ("Furry McFurson", 3.5);

    // TODO: Uraikan (destructure) tuple `cat` dalam satu baris
    // agar baris `println!` di bawah dapat berfungsi.

    // let /* pola kamu di sini */ = cat;

    println!("{name} is {age} years old");
}

/*
Penjelasan:
- Tuple adalah tipe data yang dapat menyimpan beberapa nilai dengan tipe berbeda.
- Di Rust, kamu bisa "menguraikan" (destructuring) isi tuple sekaligus dalam satu pernyataan.

Petunjuk:
- Gunakan pola `(x, y) = tuple` untuk memisahkan isinya.
- Dalam kasus ini, tuple `cat` memiliki dua elemen: nama dan usia.
- Kamu bisa mengikat keduanya ke variabel `name` dan `age`.

Referensi:
https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type
*/
