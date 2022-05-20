#[path = "devices/smartsocket.rs"]
mod smartsocket;
use smartsocket::SmartSocket;
#[path = "devices/smartthermometer.rs"]
mod smartthermometer;
use smartthermometer::SmartThermometer;

/// Main task2 routine
fn main() {
    println!("Task2 start");
    let mut socket0: SmartSocket = SmartSocket::new("Socket in kitchen");
    let mut thermometer0: SmartThermometer = SmartThermometer::new("Thermometer in kitchen");
    socket0.dis();
    socket0.en();
    socket0.update();
    thermometer0.update();
    println!("Socket0: {}", socket0);
    println!("Thermometer0: {}", thermometer0);
    println!("Task2 done");
}
