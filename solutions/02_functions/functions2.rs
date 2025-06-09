// Tipe data untuk argumen fungsi harus selalu dituliskan secara eksplisit.
//
// Dalam solusi ini, kita menambahkan anotasi tipe `u64` pada parameter `num`.
// Ini membuat fungsi dapat dikompilasi dan dijalankan dengan benar.
//
// Catatan:
// - Tipe `u64` dipilih karena mendukung operasi rentang `0..num`.
// - Kamu juga bisa mencoba tipe bilangan bulat lainnya seperti `u32` atau `i32`
//   selama sesuai dengan konteks penggunaannya.

fn call_me(num: u64) {
    for i in 0..num {
        println!("Dering! Panggilan ke-{}", i + 1);
    }
}

fn main() {
    call_me(3);
}
