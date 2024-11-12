# fork

Process and thread creation/cloning functionality for a no_std environment.

This module provides implementations for process and thread creation operations similar to Linux’s clone() system call. It supports various cloning modes including fork, vfork, and thread creation with configurable sharing of resources between parent and child processes/threads.

Features

+ Process creation with configurable resource sharing
+ Thread creation with shared address space
+ Support for Linux-compatible clone flags
+ Virtual memory management during process creation
+ File descriptor inheritance control
+ Signal handling inheritance
+ Thread Local Storage (TLS) support

## Examples

```rust
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

```

## Structs

### `CloneFlags`

```rust
pub struct CloneFlags(/* private fields */);
```

These flags determine which resources are shared between the parent and child processes/threads, matching the semantics of Linux’s clone system call.
Clone flags that control the behavior of process/thread creation.

## Functions

### `init`

```rust
pub fn init(cpu_id: usize, dtb_pa: usize)
```

Initializes the process/thread management subsystem.

### `set_tid_address`

```rust
pub fn set_tid_address(tidptr: usize) -> usize
```

Sets the clear child TID address for the current thread.

### `sys_clone`

```rust
pub fn sys_clone(
    flags: usize,
    stack: usize,
    tls: usize,
    ptid: usize,
    ctid: usize
) -> usize
```

Clone thread according to SysCall requirements

### `sys_vfork`

```rust
pub fn sys_vfork() -> usize
```

System call interface for vfork operation.

### `user_mode_thread`

```rust
pub fn user_mode_thread<F>(f: F, flags: CloneFlags) -> Tid
where
    F: FnOnce() + 'static,
```

Creates a new user mode thread with the specified function and flags.
