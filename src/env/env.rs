#[derive(Clone, Debug)]
pub struct Env {
    pub registers: [f64; 16],
    fpscr: u32,
    fpexc: u32,
    fpsid: u32,
    cpsr: u32,
}

impl Env {
    pub fn new() -> Self {
        Self {
            registers: [0.0; 16],
            fpscr: 0,
            fpexc: 0,
            fpsid: 0,
            cpsr: 0
        }
    }

    pub fn get_cpsr_n(&self) -> bool {
        (self.cpsr >> 31) == 1
    }

    pub fn get_cpsr_z(&self) -> bool {
        ((self.cpsr >> 30) & 1) == 1
    }

    pub fn get_cpsr_c(&self) -> bool {
        ((self.cpsr >> 29) & 1) == 1
    }

    pub fn get_cpsr_v(&self) -> bool {
        ((self.cpsr >> 28) & 1) == 1
    }

    pub fn get_single(&self, index: usize) -> f32 {
        assert!(index >= 0 && index < 32);

        unsafe {
            let ptr = &self.registers as *const f64 as *const f32;
            let single_slice = std::slice::from_raw_parts(ptr, 32);
            single_slice[index]
        }
    }

    pub fn get_double(&self, index: usize) -> f64 {
        assert!(index >= 0 && index < 16);

        self.registers[index]
    }

    pub fn set_single(&mut self, index: usize, value: f32) {
        assert!(index >= 0 && index < 32);

        unsafe {
            let ptr = &mut self.registers as *mut f64 as *mut f32;
            let single_slice = std::slice::from_raw_parts_mut(ptr, 32);
            single_slice[index] = value;
        }
    }

    pub fn set_double(&mut self, index: usize, value: f64) {
        assert!(index >= 0 && index < 16);

        self.registers[index] = value;
    }
}
