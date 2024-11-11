use core::mem;
use axerrno::LinuxResult;
use crate::CloneFlags;
use crate::KernelCloneArgs;
use axhal::arch::TrapFrame;

pub fn copy_thread(
    pt_regs: &mut TrapFrame,
    args: &KernelCloneArgs,
) -> LinuxResult {
    info!("copy_thread ...");

    if args.entry.is_some() {
        *pt_regs = unsafe { mem::zeroed() };
        //pt_regs.regs.gp = gp_in_global();
        // Supervisor/Machine, irqs on:
        //pt_regs.sstatus = SR_SPP | SR_SPIE;
    } else {
        let ctx = taskctx::current_ctx();
        *pt_regs = ctx.pt_regs().clone();
        if let Some(sp) = args.stack {
            pt_regs.rsp = sp as u64; // User fork
        }
        pt_regs.rax = 0; // Return value of fork()
    }

    if args.flags.contains(CloneFlags::CLONE_SETTLS) {
        set_new_tls(args.tls);
    }


    info!("copy_thread!");
    Ok(())
}

fn set_new_tls(tls: usize) {
    // Todo: modules/sys/src/lib.rs: arch_prctl ARCH_SET_FS
    // We need to differentiate current_ctx and common taskctx.
    unimplemented!("impl set_new_tls! tls {:#X}", tls);
}
