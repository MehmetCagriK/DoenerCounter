use chrono;
use rand::Rng;
use std::io;

#[derive(Debug)]
struct DoenerLog{
    worker_id: u64,
    doener_wgt: u64,
    date: chrono::DateTime<chrono::Local>,
}

impl DoenerLog{
    fn create_log(worker_id: u64) -> DoenerLog{
        let cur_date = chrono::Local::now();
        let doener_wgt = rand::thread_rng().gen_range(80..101);
        DoenerLog{
            worker_id,
            doener_wgt,
            date: cur_date,
        }
    }

    fn print_log(&self){
        println!("Worker ID: {}, Doener Weight: {}, Date: {}", self.worker_id, self.doener_wgt, self.date.format("%c").to_string());
    }
}

fn main() {
    println!("Hello to Doener Counter!");

    loop{

        let mut worker_id_str = String::new();

        io::stdin()
            .read_line(&mut worker_id_str)
            .expect("Failed to read line");

        let worker_id: u64 = match worker_id_str.trim().parse() {
                Ok(num) => num,
                Err(_) => continue
            };

        let log = DoenerLog::create_log(worker_id);
        log.print_log();
    }

}
