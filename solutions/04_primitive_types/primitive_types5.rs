fn main() {
    let cat = ("Furry McFurson", 3.5);

    // Menguraikan isi tuple menjadi dua variabel: name dan age.
    let (name, age) = cat;

    println!("{name} is {age} years old");
}

/*
Penjelasan:
- Baris `let (name, age) = cat;` membagi tuple menjadi dua variabel terpisah.
- Variabel `name` akan berisi string `"Furry McFurson"` dan `age` akan berisi `3.5`.
- Ini membuat pemanggilan `{name}` dan `{age}` di `println!` dapat digunakan dengan benar.
*/
