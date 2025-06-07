fn main() {
    // Cara termudah untuk memperbaiki error dari kompiler adalah dengan
    // menginisialisasi variabel `x`. Dengan memberikan nilai berupa bilangan bulat,
    // Rust akan menyimpulkan (infer) tipe variabel tersebut sebagai `i32`,
    // yaitu tipe default untuk bilangan bulat di Rust.
    let x = 42;

    // Namun, kita juga bisa menentukan tipe yang berbeda dari default `i32`
    // dengan menambahkan anotasi tipe secara eksplisit, misalnya:
    // let x: u8 = 42;

    if x == 10 {
        println!("x bernilai sepuluh!");
    } else {
        println!("x tidak bernilai sepuluh!");
    }
}
