use bit_field::BitField;

pub enum EntropyStatus {
    BIST = 0,
    ES16 = 1,
    WAIT = 2,
    DEAD = 3,
}

pub enum Entropy {
    Bist,
    Es16(u16),
    Wait,
    Dead,
}

unsafe fn read_raw() -> usize {
    let r: usize;
    llvm_asm!("csrrs $0, $1, x0" : "=r"(r) : "i"(0xDBF) :: "volatile");
    r
}

pub fn read() -> Option<Entropy> {
    let raw_data = unsafe { read_raw() };
    match (raw_data >> 30) & 0b11 {
        EntropyStatus::BIST => Some(Entropy::Bist),
        EntropyStatus::ES16 => Some(Entropy::Es16(raw_data as u16)),
        EntropyStatus::WAIT => Some(Entropy::Wait),
        EntropyStatus::DEAD => Some(Entropy::Dead),
        _ => None
    }
}
