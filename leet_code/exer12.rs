//1108. Defanging an IP Address

pub fn defang_i_paddr(address: String) -> String {
    address.replace('.', "[.]")
}

fn main() {
    println!("{}", defang_i_paddr("1.1.1.1".to_string()));
}
