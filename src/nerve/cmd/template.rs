//! Upload template command
//!
//! It returns a dummy CSV

use std::fs::File;
use std::io::prelude::*;


pub fn run(output: Option<&str>) {
        let template = r#"title,body,labels,assignees,milestone_id
"A nice title","A descriptive body","in_backlog,feature",arnau,1"#;

        match output {
            Some(filepath) => {
                let mut f = File::create(filepath).unwrap();
                let _ = f.write_all(template.as_bytes());
            }
            None => println!("{}", template),
        }
}
