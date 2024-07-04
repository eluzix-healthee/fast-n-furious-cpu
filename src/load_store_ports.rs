use crate::repetition_tester::RepetitionTester;
use libc::{mmap, size_t, MAP_ANON, MAP_PRIVATE, PROT_READ, PROT_WRITE};
use std::cell::RefCell;
use std::error::Error;
use std::ptr;
use std::rc::Rc;

mod perf_metrics;
mod repetition_tester;

extern "C" {
    fn Read_x1(count: u64, data: *mut u8);
    fn Read_x2(count: u64, data: *mut u8);
    fn Read_x3(count: u64, data: *mut u8);
    fn Read_x4(count: u64, data: *mut u8);
}

struct TestParams {
    expected_bytes: u64,
    seconds_to_try: u64,
}

struct TesterFunction {
    name: &'static str,
    function: unsafe extern "C" fn(count: u64, data: *mut u8),
    tester: Rc<RefCell<RepetitionTester>>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let total_size = 1024 * 1024 * 1024;
    let addr = unsafe {
        mmap(
            ptr::null_mut(), // Address at which to start the mapping (nullptr lets the OS choose)
            1024 * 16,       // Number of bytes to map
            PROT_READ | PROT_WRITE, // Enable read and write access
            MAP_PRIVATE | MAP_ANON,
            -1, // File descriptor not used with MAP_ANON
            0,  // Offset not used with MAP_ANON
        )
    };

    let functions = vec![
        TesterFunction {
            name: "Read_x1",
            function: Read_x1,
            tester: Rc::new(RefCell::new(RepetitionTester::new())),
        },
        TesterFunction {
            name: "Read_x2",
            function: Read_x2,
            tester: Rc::new(RefCell::new(RepetitionTester::new())),
        },
        TesterFunction {
            name: "Read_x3",
            function: Read_x3,
            tester: Rc::new(RefCell::new(RepetitionTester::new())),
        },
        TesterFunction {
            name: "Read_x4",
            function: Read_x4,
            tester: Rc::new(RefCell::new(RepetitionTester::new())),
        },
    ];

    let params = TestParams {
        expected_bytes: total_size as u64,
        seconds_to_try: 2,
    };

    loop {
        for ft in &functions {
            println!("\n----- {} -----", ft.name);
            let mut tester = ft.tester.borrow_mut();
            tester.start_test_wave(params.seconds_to_try, params.expected_bytes);
            let total_size = params.expected_bytes as size_t;
            let buffer = unsafe { std::slice::from_raw_parts_mut(addr, total_size) };
            while tester.is_testing() {
                tester.begin_time();
                unsafe {
                    (ft.function)(total_size as u64, buffer.as_mut_ptr() as *mut u8);
                }
                tester.end_time();
                tester.count_bytes(total_size as u64);
            }
        }
    }
}
