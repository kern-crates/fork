#![no_std]
#![no_main]

#[macro_use]
extern crate axlog2;
extern crate alloc;

use core::panic::PanicInfo;
use fork::user_mode_thread;
use fork::CloneFlags;

/// Entry
#[no_mangle]
pub extern "Rust" fn runtime_main(cpu_id: usize, dtb: usize) {
    init(cpu_id, dtb);
    start(cpu_id, dtb);
    panic!("Never reach here!");
}

pub fn init(cpu_id: usize, dtb: usize) {
    axlog2::init("info");
    fork::init(cpu_id, dtb);
}

pub fn start(_cpu_id: usize, _dtb: usize) {
    info!("start thread ...");
    let tid = user_mode_thread(
        move || {
            kernel_init();
        },
        CloneFlags::CLONE_FS,
    );
    assert_eq!(tid, 1);

    schedule_preempt_disabled();

    info!("[rt_fork]: ok!");
    axhal::misc::terminate();
}

fn schedule_preempt_disabled() {
    task::yield_now();
}

/// Prepare for entering first user app.
fn kernel_init() {
    info!("[new process]: enter ...");
    let task = task::current();
    task.set_state(taskctx::TaskState::Blocked);
    let rq = run_queue::task_rq(&task.sched_info);
    info!("[new process]: yield ...");
    rq.lock().resched(false);
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    error!("{}", info);
    arch_boot::panic(info)
}
