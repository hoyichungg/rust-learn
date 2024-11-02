use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::{self, Sender, Receiver};
use std::time::Duration;

struct Runner {
    lane: usize,
    runner_number: usize,
}

impl Runner {
    fn run(&self, baton_sender: Option<Sender<String>>, baton_receiver: Option<Receiver<String>>) {
        if let Some(receiver) = baton_receiver {
            receiver.recv().expect("接力棒傳遞失敗");
        }
        
        println!("跑道 {} 的跑者 {} 開始跑步！", self.lane, self.runner_number);
        thread::sleep(Duration::from_millis(500));
        
        if let Some(sender) = baton_sender {
            sender.send("接力棒".to_string()).expect("接力棒傳遞失敗");
        }
        
        println!("跑道 {} 的跑者 {} 完成接力！", self.lane, self.runner_number);
    }
}

struct Referee {
    winner: Arc<Mutex<Option<usize>>>,
}

impl Referee {
    fn monitor(&self, rx: Receiver<usize>, total_lanes: usize) {
        let mut completed = 0;
        
        for lane in rx {
            let mut winner = self.winner.lock().unwrap();
            if winner.is_none() {
                *winner = Some(lane);
                println!("🏆 跑道 {} 獲勝！", lane);
            }
            completed += 1;
            
            // 當所有跑道完成後結束監控
            if completed == total_lanes {
                println!("比賽結束！");
                break;
            }
        }
    }
}

fn main() {
    let total_lanes = 10;
    let mut threads = vec![];
    let winner = Arc::new(Mutex::new(None));
    let (tx, rx) = mpsc::channel();
    
    for lane in 1..=total_lanes {
        let (tx1, rx1) = mpsc::channel();
        let (tx2, rx2) = mpsc::channel();
        
        let runner1 = Runner { lane, runner_number: 1 };
        let runner2 = Runner { lane, runner_number: 2 };
        let runner3 = Runner { lane, runner_number: 3 };
        
        let tx_done = tx.clone();
        
        threads.push(thread::spawn(move || {
            runner1.run(Some(tx1.clone()), None);
            runner2.run(Some(tx2.clone()), Some(rx1));
            runner3.run(None, Some(rx2));
            
            // 通知裁判此跑道完成
            tx_done.send(lane).expect("無法通知裁判此跑道完成");
        }));
    }
    
    let referee = Referee {
        winner: Arc::clone(&winner),
    };
    
    // 啟動裁判監控執行緒
    let referee_thread = thread::spawn(move || {
        referee.monitor(rx, total_lanes);
    });
    
    for thread in threads {
        thread.join().expect("執行緒加入失敗");
    }
    
    // 等待裁判執行緒結束
    referee_thread.join().expect("裁判執行緒加入失敗");
}