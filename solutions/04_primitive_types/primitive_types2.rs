fn main() {
    // Huruf awal nama saya
    let my_first_initial = 'C';

    // Mengecek apakah `my_first_initial` adalah karakter alfabet
    if my_first_initial.is_alphabetic() {
        println!("Bersifat alfabet!");
    } else if my_first_initial.is_numeric() {
        println!("Bersifat numerik!");
    } else {
        println!("Bukan alfabet maupun numerik!");
    }

    // Kita memilih emoji 🦀 (kepiting) sebagai karakter favorit
    // Karakter ini bukan huruf dan bukan angka, sehingga akan masuk kategori terakhir
    let your_character = '🦀';

    if your_character.is_alphabetic() {
        println!("Bersifat alfabet!");
    } else if your_character.is_numeric() {
        println!("Bersifat numerik!");
    } else {
        println!("Bukan alfabet maupun numerik!");
    }
}
