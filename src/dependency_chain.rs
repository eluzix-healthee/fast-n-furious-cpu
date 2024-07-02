use std::cell::RefCell;
use std::rc::Rc;
use crate::repetition_tester::repetition_tester::RepetitionTester;

mod perf_metrics;
mod repetition_tester;

extern "C" {
    fn TestExample1(outer_loop_count: u64, data: *mut u8);
    fn TestExample2(outer_loop_count: u64, data: *mut u8);
}

struct TestParams {
    expected_bytes: u64,
    seconds_to_try: u64,
}

struct TesterFunction {
    name: &'static str,
    count: u64,
    function: unsafe extern "C" fn(outer_loop_count: u64, data: *mut u8),
    tester: Rc<RefCell<RepetitionTester>>,
}

impl TesterFunction {
    fn new(
        name: &'static str,
        count: u64,
        function: unsafe extern "C" fn(outer_loop_count: u64, data: *mut u8),
    ) -> Self {
        TesterFunction {
            name,
            count,
            function,
            tester: Rc::new(RefCell::new(RepetitionTester::new())),
        }
    }
}

fn main() {
    let total_size: u64 = 1024 * 1024 * 1024;
    let mut var: u8 = 0;

    let functions = vec![
        TesterFunction::new(
            "Example 1",
            total_size,
            TestExample1
        ),
        TesterFunction::new(
            "Example 2",
            total_size,
            TestExample2
        )
    ];

    let params = TestParams {
        expected_bytes: total_size,
        seconds_to_try: 2,
    };

    for ft in &functions {
        let mut tester = ft.tester.borrow_mut();
        println!("Running {}", ft.name);
        tester.start_test_wave(params.seconds_to_try, params.expected_bytes);
        while tester.is_testing() {
            tester.begin_time();
            unsafe {
                (ft.function)(ft.count, &mut var);
            }
            tester.end_time();
            tester.count_bytes(total_size as u64);
        }
    }
}
