struct Calc {}

impl Calc {
    fn new() -> Calc {
        return Calc {};
    }

    unsafe fn new_ptr() -> *mut Calc {
        Box::into_raw(Box::new(Calc::new()))
    }

    fn dispose(&mut self) {
        unsafe {
            Box::from_raw(self as *mut Calc);
        }
    }

    fn add(&mut self, v1: i32, v2: i32) -> i32 {
        return v1 + v2;
    }

    fn substract(&mut self, v1: i32, v2: i32) -> i32 {
        return v1 - v2;
    }
}
