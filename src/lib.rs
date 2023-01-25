
static DISPATCH : [unsafe fn (pc: * const u8) -> * const u8; 256] = [
    add, sub, add, sub, add, sub, add, sub, add, sub, add, sub, add, sub, add, sub,
    add, sub, add, sub, add, sub, add, sub, add, sub, add, sub, add, sub, add, sub,
    add, sub, add, sub, add, sub, add, sub, add, sub, add, sub, add, sub, add, sub,
    add, sub, add, sub, add, sub, add, sub, add, sub, add, sub, add, sub, add, sub,
    add, sub, add, sub, add, sub, add, sub, add, sub, add, sub, add, sub, add, sub,
    add, sub, add, sub, add, sub, add, sub, add, sub, add, sub, add, sub, add, sub,
    add, sub, add, sub, add, sub, add, sub, add, sub, add, sub, add, sub, add, sub,
    add, sub, add, sub, add, sub, add, sub, add, sub, add, sub, add, sub, add, sub,
    add, sub, add, sub, add, sub, add, sub, add, sub, add, sub, add, sub, add, sub,
    add, sub, add, sub, add, sub, add, sub, add, sub, add, sub, add, sub, add, sub,
    add, sub, add, sub, add, sub, add, sub, add, sub, add, sub, add, sub, add, sub,
    add, sub, add, sub, add, sub, add, sub, add, sub, add, sub, add, sub, add, sub,
    add, sub, add, sub, add, sub, add, sub, add, sub, add, sub, add, sub, add, sub,
    add, sub, add, sub, add, sub, add, sub, add, sub, add, sub, add, sub, add, sub,
    add, sub, add, sub, add, sub, add, sub, add, sub, add, sub, add, sub, add, sub,
    add, sub, add, sub, add, sub, add, sub, add, sub, add, sub, add, sub, add, sub,
];

#[inline]
unsafe fn dispatch(pc: * const u8) -> * const u8 {
    DISPATCH[*pc as usize](pc.offset(1))
}


pub unsafe fn add(pc: * const u8) -> * const u8 {
    // do_add
    dispatch(pc)
}

pub unsafe fn sub(pc: * const u8) -> * const u8 {
    // do_sub
    dispatch(pc)
}

pub fn main() {
    let code = [0, 1, 1, 0];
    let pc = code[..].as_ptr();

    unsafe { dispatch(pc) };
}
