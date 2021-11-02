enum TrafficLight{
    Red,
    Green,
    Yellow,
}

pub trait TimeRemaining{
    fn time(&self) -> u8 ;
}

impl TimeRemaining for TrafficLight{
    fn time(&self) -> u8 {
        let temp_self= &self;
        match temp_self{
            TrafficLight::Red => 6,
            TrafficLight::Green => 9,
            TrafficLight::Yellow => 2,
        }
    }
}


extern crate timer;
extern crate chrono;
use std::sync::mpsc::channel;
use colored::Colorize;

fn main() {
  loop{
    let red_light=TrafficLight::Red;
    let yellow_light=TrafficLight::Yellow;
    let green_light=TrafficLight::Green;

    let mut red_light_counter = red_light.time();
    let mut yellow_light_counter = yellow_light.time();
    let mut green_light_counter = green_light.time();

    while red_light_counter > 0 {
        println!("{} {}","Red light on".red(), red_light_counter);
        red_light_counter -= 1;
        delay_red();
        }
    while yellow_light_counter > 0 {
        println!("{}", "Yellow light on".yellow()); 
        yellow_light_counter -= 1;
        delay_yellow();
        } 
    while green_light_counter > 0 {
        println!("{} {}","Green light on".green(), green_light_counter);
        green_light_counter -= 1;
        delay_green();
        }
    
    let mut yellow_light_counter = yellow_light.time();
    while yellow_light_counter > 0 {
        println!("{}", "Yellow light on".yellow()); 
        yellow_light_counter -= 1;
        delay_yellow();
        }
  }// end of loop
}// end of fn main()

fn delay_red(){
        let timer = timer::Timer::new();
        let (tx, rx) = channel();
        let _guard = timer.schedule_with_delay(chrono::Duration::seconds(2), move || {
            tx.send(()).unwrap();
        });
        rx.recv().unwrap();
}

fn delay_yellow(){
    let timer = timer::Timer::new();
        let (tx, rx) = channel();
        let _guard = timer.schedule_with_delay(chrono::Duration::seconds(3), move || {
            tx.send(()).unwrap();
        });
        rx.recv().unwrap();
    
}

fn delay_green(){
        let timer = timer::Timer::new();
        let (tx, rx) = channel();
        let _guard = timer.schedule_with_delay(chrono::Duration::seconds(2), move || {
            tx.send(()).unwrap();
        });
        rx.recv().unwrap();
}

