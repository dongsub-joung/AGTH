use tasklist;

// https://docs.rs/tasklist/latest/tasklist/#:~:text=Get%20all%20process%20pid%20%2C%20process,%2Ccpn%2Cdes)%20%7D%20%7D
pub fn init_listup(){
    unsafe{
        let tl = tasklist::Tasklist::new();
        for i in tl{
            println!("{} {} {}",i.get_pid(),i.get_pname(),i.get_user());
        }
    }
}

pub fn init_selecting(pid: String){

    let pid= pid.parse::<u32>().unwrap();
    // pid
    unsafe{
        let pname = tasklist::find_process_name_by_id(pid);
        let mem= tasklist::get_proc_memory_info(pid);
        
        println!("pname: {:#?}",pname);
        println!("get_proc_threads: {:?}",tasklist::get_proc_threads(pid));
        println!("get_quota_peak_non_paged_pool_usage: {:?}",mem.get_quota_peak_non_paged_pool_usage());
    }
}