const RAM_SIZE: usize = 4096;

pub struct Emu {
    pc: u16,
    ram: [u8; RAM_SIZE],
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/
