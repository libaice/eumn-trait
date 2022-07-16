// 定义Enum ， 三种信号灯 Red Green Yellow
enum TrafficLight {
    Red,
    Green,
    Yellow,
}

// 定义trait
trait LightTime {
    fn time(&self) -> u8;
}


// 为TrafficLight 实现trait
impl LightTime for TrafficLight {
    fn time(&self) -> u8 {
        // Match 灯的类型
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Green => 50,
            TrafficLight::Yellow => 10,
        }
    }
}


fn main() {
    let yellow = TrafficLight::Yellow;
    let red = TrafficLight::Red;
    let green = TrafficLight::Green;

    println!("yellow time is {:?}", yellow.time());
    println!("red time is {:?}", red.time());
    println!("green time is {:?}", green.time());
}
