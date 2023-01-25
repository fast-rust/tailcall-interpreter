
pub struct State {
    pc: * const u8,
    sp: * mut i32,
}

static DISPATCH : [fn (State) -> State; 256] = [
    add, sub, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop,
    stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop,
    stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop,
    stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop,
    stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop,
    stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop,
    stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop,
    stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop,
    stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop,
    stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop,
    stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop,
    stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop,
    stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop,
    stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop,
    stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop,
    stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop, stop,
];

#[inline]
fn dispatch(mut state: State) -> State {
    let opcode = unsafe { *state.pc };
    state.pc = unsafe { state.pc.offset(1) };
    DISPATCH[opcode as usize](state)
}


pub fn add(state: State) -> State {
    unsafe { *state.sp += 1 };
    dispatch(state)
}

pub fn sub(state: State) -> State {
    unsafe { *state.sp -= 1 };
    dispatch(state)
}

pub fn stop(state: State) -> State {
    state
}

pub fn main() {
    // add sub add stop
    let code = [0, 1, 0, 2];

    let mut stack = [0; 32];
    let state = State {
        pc: code[..].as_ptr(),
        sp: stack[..].as_mut_ptr(),
    };

    { dispatch(state) };

    assert_eq!(stack[0], 1);
}
