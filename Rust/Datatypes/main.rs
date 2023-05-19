fn interger_data_types() {
    // Rust has signed integer types i8, i16, i32, i64, i128
    // by default all the variable in rust are immutable

    let a1:i8 = -45;
    let a2:i32 = 34567;
    let a3:i32 = 0xffff;
    let a4:i32 = 0o777;
    let a5:i32 = 0b1100;

    // Rust has unsigned integer types u8, u16, u32, u64, u128
    let b:u32 = 5678;

    //Rust also has platform-specific integer types: isize, usize
    let c:isize = -67800;

    println!("\n Numbers are {} {} {} {} {} {} {}", a1, a2, a3, a4, a5, b, c);
    println!("\n size of isize {}", std::mem::size_of::<isize>());
}

fn main() {
    interger_data_types();
}
