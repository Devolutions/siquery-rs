use tables::{ProcessEnvsRow, ProcessesRow};

impl ProcessEnvsRow {
    pub fn get_specific() -> Vec<ProcessEnvsRow> {
        let mut process_envs_table: Vec<ProcessEnvsRow> = Vec::new();
        let pidlist = ProcessesRow::get_proc_list();
        let argmax = ProcessesRow::gen_max_args();
        for pid in pidlist {
            let proc_args = ProcessesRow::get_proc_raw_args(pid, argmax);
            for (key, value) in proc_args.env.iter() {
                process_envs_table.push(
                    ProcessEnvsRow {
                        pid,
                        key: key.to_owned(),
                        value: value.to_owned(),
                    }
                )
            }
        }
        process_envs_table
    }
}