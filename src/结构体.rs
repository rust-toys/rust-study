#[derive(Debug)]
pub struct Rectangular {
    width: u32,
    height: u32,
}

impl Rectangular {
    fn get_area(&self) -> u32 {
        self.height * self.width
    }
    // 关联函数:
    pub fn test(s: u32) -> Rectangular {
        Rectangular {
            width: s,
            height: s,
        }
    }
}
