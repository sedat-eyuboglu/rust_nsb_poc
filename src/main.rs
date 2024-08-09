use sysinfo::{Process, System, Pid, RefreshKind, ProcessRefreshKind, Components};
use native_dialog::{MessageDialog, MessageType};
use std::process;


const BLACK_LIST_OF_PROCCESS: [&str; 2] = ["firefox", "msedge"];

fn main() {
    if check_system() == 1 {
        show_blacklist_fail();
        process::exit(0);
    } else {
        dialog_alert("title", "msg");
    }
}

fn check_system() -> u32 {
    let mut sys = System::new_with_specifics(RefreshKind::new().with_processes(ProcessRefreshKind::everything()));
    sys.refresh_processes_specifics(ProcessRefreshKind::new());

    //let mut sys = System::new_all();
    //sys.refresh_all();
    
    let mut black_listed_pids: Vec<Pid> = Vec::with_capacity(100);
    for (_, proc) in sys.processes() {
        if is_blacklisted(proc) {
            black_listed_pids.push(proc.pid());
        }
    }

    if !black_listed_pids.is_empty() {
        let msg = format!("You can NOT continue while processes listed below are running.\n\n{}\nDo you want to continue and terminate the processes?"
            ,get_proc_names(&black_listed_pids, &sys));
        if !dialog_confirm("Terminate and continue?", &msg) {
            return 1;
        }
        
        let mut killed_pids: Vec<u32> = Vec::with_capacity(100);
        for pid in black_listed_pids {
            if sys.refresh_process(pid) {
                if let Some(proc) = sys.process(pid) {
                    if !is_blacklisted(proc) {
                        continue;
                    }
                    if let Some(parent) = proc.parent() {
                        if killed_pids.contains(&parent.as_u32()) {
                            continue;
                        }
                    }
                    if !proc.kill() {
                        return 1;
                    }
                    killed_pids.push(pid.as_u32());
                }
            }
        }
    }

    // let start = Instant::now();
    

    // let display_infos = DisplayInfo::all().unwrap();
    // for display_info in display_infos {
    //     println!("display_info {display_info:?}");
    // }

    


    0
}

fn is_blacklisted(process: &Process) -> bool {
    let name = process.name().split('.').next().unwrap_or("");
    let len = BLACK_LIST_OF_PROCCESS.len();
    for i in 0..len {
        if name.contains(BLACK_LIST_OF_PROCCESS[i]) {
            return true;
        }
    }
    false
}

fn get_proc_names(pids: &[Pid], sys: &System) -> String {
    let mut names: String = String::new();
    for pid in pids {
        if let Some(proc) = sys.process(*pid) {
            if !names.contains(proc.name()) {
                names.push_str(&format!("{}\n", proc.name()));
            }
        } 
    }
    names
}

fn show_blacklist_fail() {
    dialog_alert("Can not continue!", "You have running processes that are NOT allowed. To continue terminate them.");
}

fn dialog_confirm(title: &str, msg: &str) -> bool {
    MessageDialog::new()
        .set_type(MessageType::Info)
        .set_title(title)
        .set_text(msg)
        .show_confirm()
        .unwrap()
}

fn dialog_alert(title: &str, msg: &str) {
    MessageDialog::new()
        .set_type(MessageType::Info)
        .set_title(title)
        .set_text(msg)
        .show_alert()
        .unwrap();
}