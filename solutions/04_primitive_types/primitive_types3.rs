fn main() {
    // Membuat array berisi 100 elemen, semuanya bernilai 42.
    // Notasi `[42; 100]` artinya kita membuat array dengan 100 salinan dari angka 42.
    let a = [42; 100];

    if a.len() >= 100 {
        println!("Wow, array-nya besar sekali!");
    } else {
        println!("Halah, array segitu mah sarapan saya.");
        panic!("Array tidak cukup besar, perlu lebih banyak elemen");
    }
}
