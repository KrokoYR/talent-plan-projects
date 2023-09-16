use bitflags::bitflags;

bitflags! {
    pub struct Flags: u32 {
        const FLAG_A = 0b00000001;
        const FLAG_B = 0b00000010;
        const FLAG_C = 0b00000100;
    }
}

pub fn f(settings: Flags) {
    if settings.contains(Flags::FLAG_A) {
        println!("doing thing A");
    }
    if settings.contains(Flags::FLAG_B) {
        println!("doing thing B");
    }
    if settings.contains(Flags::FLAG_C) {
        println!("doing thing C");
    }
}
