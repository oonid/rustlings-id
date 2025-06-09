# Vektor

Vektor (`Vector`) adalah salah satu struktur data yang paling sering digunakan di Rust.  
Dalam bahasa pemrograman lain, kamu mungkin mengenalnya sebagai **array dinamis**.  
Namun karena Rust bekerja lebih dekat dengan level mesin (_low-level_), maka perbedaannya cukup penting:

- **Array di Rust** disimpan di _stack_, sehingga **tidak dapat bertambah atau berkurang ukurannya**, dan ukurannya **harus diketahui saat kompilasi**.
- **Vektor di Rust** disimpan di _heap_, sehingga **ukuran elemen dapat berubah-ubah** (bisa bertambah atau berkurang secara dinamis saat runtime).

Topik tentang vektor sebenarnya dibahas di bab yang lebih lanjut dalam buku _The Rust Programming Language_,  
tapi kami merasa vektor sangat berguna sehingga layak dipelajari lebih awal.  

Untuk struktur data lainnya seperti **hash map**, akan dibahas di bagian terpisah nanti.

---

## Informasi lebih lanjut

- [Menyimpan Daftar Nilai dengan Vektor (Bab Buku)](https://doc.rust-lang.org/book/ch08-01-vectors.html)
- [`.iter_mut()` pada Slice](https://doc.rust-lang.org/std/primitive.slice.html#method.iter_mut)
- [`.map()` pada Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)
