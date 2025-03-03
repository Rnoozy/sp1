extern crate cc;

fn main() {
    cc::Build::new()
        .file("cpp/extern.cpp") // File C++ yang akan dikompilasi
        .cpp(true) // Aktifkan mode C++
        .flag("-std:c++20") // Tambahkan flag untuk menggunakan C++20
        .compile("sp1-recursion-core"); // Nama library yang dihasilkan
}
