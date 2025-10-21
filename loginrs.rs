use std::collections::HashMap;
use std::io;

fn main() {
    let mut users: HashMap<String, String> = HashMap::new();
    println!("🔑 Sistem Login Rust Playground 🔑");

    loop {
        println!("\nPilih opsi:");
        println!("1. Daftar");
        println!("2. Login");
        println!("3. Lupa Password");
        println!("4. Keluar");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => register(&mut users),
            "2" => login(&users),
            "3" => forgot_password(&users),
            "4" => {
                println!("Terima kasih! 👋");
                break;
            }
            _ => println!("Opsi tidak valid!"),
        }
    }
}

fn register(users: &mut HashMap<String, String>) {
    let mut username = String::new();
    println!("Masukkan username:");
    io::stdin().read_line(&mut username).unwrap();
    let username = username.trim().to_string();

    if users.contains_key(&username) {
        println!("Username sudah digunakan!");
        return;
    }

    let mut password = String::new();
    println!("Masukkan password:");
    io::stdin().read_line(&mut password).unwrap();
    let password = password.trim().to_string();

    users.insert(username.clone(), password);
    println!("✅ Akun '{}' berhasil dibuat!", username);
}

fn login(users: &HashMap<String, String>) {
    let mut username = String::new();
    println!("Masukkan username:");
    io::stdin().read_line(&mut username).unwrap();
    let username = username.trim();

    let mut password = String::new();
    println!("Masukkan password:");
    io::stdin().read_line(&mut password).unwrap();
    let password = password.trim();

    match users.get(username) {
        Some(pw) if pw == password => println!("🎉 Login berhasil! Selamat datang, {}!", username),
        _ => println!("❌ Username atau password salah!"),
    }
}

fn forgot_password(users: &HashMap<String, String>) {
    let mut username = String::new();
    println!("Masukkan username untuk reset password:");
    io::stdin().read_line(&mut username).unwrap();
    let username = username.trim();

    match users.get(username) {
        Some(pw) => println!("Password akun '{}' adalah '{}'", username, pw),
        None => println!("Username tidak ditemukan!"),
    }
}
