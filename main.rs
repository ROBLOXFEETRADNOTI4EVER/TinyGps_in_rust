// Need to add to it so it refreshes itself the esp32 in case of not recing data for 5 secounds it will just start redoing 


#![no_std]
#![no_main]



use core::ptr::null;

use defmt::{info, println};
use embedded_io_async::{Read, Write};
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::{clock::CpuClock, peripheral::Peripheral};
use esp_hal::timer::timg::TimerGroup;
use esp_println::{self as _, print};
use esp_hal::uart::{Uart, Config};
use heapless::{String, Vec};
use esp_hal::system::software_reset;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
fn trigger_reset() -> ! {
    software_reset();
}
const _GPS_VERSION : i32 = 13;
const _GPS_MPH_PER_KNOT :f32 = 1.15077945;

const _GPS_MPS_PER_KNOT :f32 =  0.51444444;
const _GPS_KMPH_PER_KNOT : f32 = 1.852;
const _GPS_MILES_PER_METER : f32 = 0.00062137112;
const _GPS_KM_PER_METER: f32 =  0.001;


struct TinyGPS<UART> {
    uart: UART,
     bufff : Vec<char,128>,
}

impl<UART> TinyGPS<UART>
where 
    UART: Read + Write,
{
    
   async  fn encode(&mut self, c: u8) -> bool {

    // call every single one and return with option true or false with the f64 and then match statment for each
        let mut  valid_sentence = false;
        // valid_sentence = true;
        // valid_sentence
        // println!("{}",c);
        if c == b'\n'{

            // info!("COMPLETED GPS SENTENCE");
            valid_sentence = true;
            // info!("BUFFER IS {}", self.bufff);
            // info!(" Latitude ->{}",self.latitude().await);
            // info!(" Longitude -> {}",self.longitude().await);
            // // info!("{}",self.altitude().await);
            // info!(" LAtitude + Longitude {}", self.latitude_longitude().await);
        }
        
    if c == b'$'{
        if self.bufff.len() > 5 {
            Timer::after(Duration::from_millis(5)).await;
            if self.bufff[1] == 'G' && self.bufff[2] == 'Q' && self.bufff[3] == 'G' && self.bufff[4] == 'S' && self.bufff[5] == 'V' {
                // info!(" \n 0 QZSS SATALITES VISIBLE\n");
                // info!("{}",self.latitude().await);

                self.bufff.clear();

                return  false;
            } else if self.bufff[1] == 'G' && self.bufff[2] == 'A' && self.bufff[3] == 'G' && self.bufff[4] == 'S' && self.bufff[5] == 'V'  {
                // info!(" \n 0 Galileo SATALITES VISIBLE\n");
                // info!("{}",self.latitude().await);

                self.bufff.clear();

                return  false;
            } else if self.bufff[1] == 'G' && self.bufff[2] == 'P' && self.bufff[3] == 'G' && self.bufff[4] == 'S' && self.bufff[5] == 'V'  {
                // info!(" \n 1 GPS  SATALITE VISIBLE\n");
                // info!("{}",self.latitude().await);

                // return  false;
            }else if self.bufff[1] == 'G' && self.bufff[2] == 'N' && self.bufff[3] == 'G' && self.bufff[4] == 'L' && self.bufff[5] == 'L'  {

                
                // info!(" \n Empty Lat/Lon \n");
                // info!("{}",self.latitude().await);
                        // self.bufff.clear();

                // return  false;
            }
        }
      
        self.bufff.clear();
    }

   
    
    match self.bufff.push(c as char){
        Ok(()) =>{
            // info!("whole buffer is {}" ,self.bufff);
        }
        Err(e) =>{
            // info!("AN error accoured with this char ->  {} buffer might be full", e);
            // info!(" buffer size -> {}",self.bufff.len());
            self.bufff.clear();

        }
    }
  


  

        valid_sentence
     }
    
     async fn latitude(&self) -> Option<f64> {
        if self.bufff.len() < 4 {
            return None;
        }
        
        let mut star_pos = None;
        for (i, &ch) in self.bufff.iter().enumerate() {
            if ch == '*' {
                star_pos = Some(i);
                break;
            }
        }
        
        let star_idx = match star_pos {
            Some(idx) => idx,
            None => return None,
        };
        
        if star_idx + 3 > self.bufff.len() {
            return None;
        }
        
        let mut calculated_checksum: u8 = 0;
        for i in 1..star_idx {
            if let Some(&ch) = self.bufff.get(i) {
                calculated_checksum ^= ch as u8;
            }
        }
        
        let high_char = self.bufff[star_idx + 1];
        let low_char = self.bufff[star_idx + 2];
        
        let high_val = match high_char {
            '0'..='9' => high_char as u8 - b'0',
            'A'..='F' => high_char as u8 - b'A' + 10,
            'a'..='f' => high_char as u8 - b'a' + 10,
            _ => return None,
        };
        
        let low_val = match low_char {
            '0'..='9' => low_char as u8 - b'0',
            'A'..='F' => low_char as u8 - b'A' + 10,
            'a'..='f' => low_char as u8 - b'a' + 10,
            _ => return None,
        };
        
        let transmitted_checksum = (high_val << 4) | low_val;
        
        if calculated_checksum != transmitted_checksum {
            return None;
        }
        
        let mut comma_positions: Vec<usize, 10> = Vec::new();
        
        for (i, &char_) in self.bufff.iter().enumerate() {
            if char_ == ',' {
                if comma_positions.push(i).is_err() {
                    return None;
                }
            }
        }
        
        if comma_positions.len() < 2 {
            return None;
        }
        
        let lat_start = comma_positions[0] + 1;
        let lat_end = comma_positions[1];
        
        if lat_start >= lat_end {
            return None; 
        }
        
        let mut lat_str: String<16> = String::new();
        for i in lat_start..lat_end {
            if let Some(&ch) = self.bufff.get(i) {
                if lat_str.push(ch).is_err() {
                    return None; 
                }
            }
        }
        
        if lat_str.len() < 7 {
            return None;
        }
        
        let degrees_str = &lat_str[0..2];
        let minutes_str = &lat_str[2..];
        
        let degrees: f64 = degrees_str.parse().ok()?;
        let minutes: f64 = minutes_str.parse().ok()?;
        
        let mut decimal_lat = degrees + (minutes / 60.0);
        
        let dir_start = comma_positions[1] + 1;
        if let Some(&direction) = self.bufff.get(dir_start) {
            if direction == 'S' {
                decimal_lat = -decimal_lat; 
            }
        }
        
        Some(decimal_lat)
    }
    
    async fn longitude(&self) -> Option<f64> {
        if self.bufff.len() < 4 {
            return None;
        }
        
        let mut star_pos = None;
        for (i, &ch) in self.bufff.iter().enumerate() {
            if ch == '*' {
                star_pos = Some(i);
                break;
            }
        }
        
        let star_idx = match star_pos {
            Some(idx) => idx,
            None => return None,
        };
        
        if star_idx + 3 > self.bufff.len() {
            return None;
        }
        
        let mut calculated_checksum: u8 = 0;
        for i in 1..star_idx {
            if let Some(&ch) = self.bufff.get(i) {
                calculated_checksum ^= ch as u8;
            }
        }
        
        let high_char = self.bufff[star_idx + 1];
        let low_char = self.bufff[star_idx + 2];
        
        let high_val = match high_char {
            '0'..='9' => high_char as u8 - b'0',
            'A'..='F' => high_char as u8 - b'A' + 10,
            'a'..='f' => high_char as u8 - b'a' + 10,
            _ => return None,
        };
        
        let low_val = match low_char {
            '0'..='9' => low_char as u8 - b'0',
            'A'..='F' => low_char as u8 - b'A' + 10,
            'a'..='f' => low_char as u8 - b'a' + 10,
            _ => return None,
        };
        
        let transmitted_checksum = (high_val << 4) | low_val;
        
        if calculated_checksum != transmitted_checksum {
            return None;
        }
    
        let mut comma_positions: Vec<usize, 10> = Vec::new();
        
        for (i, &char_) in self.bufff.iter().enumerate() {
            if char_ == ',' {
                if comma_positions.push(i).is_err() {
                    return None;
                }
            }
        }
        
        if comma_positions.len() < 4 {
            return None;
        }
        
        let lon_start = comma_positions[2] + 1;
        let lon_end = comma_positions[3];
        
        if lon_start >= lon_end {
            return None;
        }
        
        let mut lon_str: String<16> = String::new();
        for i in lon_start..lon_end {
            if let Some(&ch) = self.bufff.get(i) {
                if lon_str.push(ch).is_err() {
                    return None;
                }
            }
        }
        
        if lon_str.len() < 8 {
            return None;
        }
        
        let degrees_str = &lon_str[0..3];
        let minutes_str = &lon_str[3..];
        
        let degrees: f64 = degrees_str.parse().ok()?;
        let minutes: f64 = minutes_str.parse().ok()?;
        
        let mut decimal_lon = degrees + (minutes / 60.0);
        
        let dir_start = comma_positions[3] + 1;
        if let Some(&direction) = self.bufff.get(dir_start) {
            if direction == 'W' {
                decimal_lon = -decimal_lon;
            }
        }
        
        Some(decimal_lon)
    }
    
   async fn altitude(&self) -> Option<f64> {
    if self.bufff.len() < 6 {
        return None;
    }
    
    if !(self.bufff[1] == 'G' && self.bufff[2] == 'P' && self.bufff[3] == 'G' && self.bufff[4] == 'G' && self.bufff[5] == 'A') {
        return None;
    }
    
    let mut comma_positions: Vec<usize, 20> = Vec::new();
    
    for (i, &char_) in self.bufff.iter().enumerate() {
        if char_ == ',' {
            if comma_positions.push(i).is_err() {
                return None;
            }
        }
    }
    
    if comma_positions.len() < 10 {
        return None;
    }
    
    let alt_start = comma_positions[8] + 1;
    let alt_end = comma_positions[9];
    
    if alt_start >= alt_end {
        return None;
    }
    
    let mut alt_str: String<16> = String::new();
    for i in alt_start..alt_end {
        if let Some(&ch) = self.bufff.get(i) {
            if alt_str.push(ch).is_err() {
                return None;
            }
        }
    }
    
    let altitude: f64 = alt_str.parse().ok()?;
    Some(altitude)
}

async fn speed(&self) -> Option<f64> {
    if self.bufff.len() < 6 {
        return None;
    }
    
    if !(self.bufff[1] == 'G' && self.bufff[2] == 'N' && self.bufff[3] == 'R' && self.bufff[4] == 'M' && self.bufff[5] == 'C') {
        return None;
    }
    
    let mut comma_positions: Vec<usize, 20> = Vec::new();
    
    for (i, &char_) in self.bufff.iter().enumerate() {
        if char_ == ',' {
            if comma_positions.push(i).is_err() {
                return None;
            }
        }
    }
    
    if comma_positions.len() < 8 {
        return None;
    }
    
    let speed_start = comma_positions[6] + 1;
    let speed_end = comma_positions[7];
    
    if speed_start >= speed_end {
        return None;
    }
    
    let mut speed_str: String<16> = String::new();
    for i in speed_start..speed_end {
        if let Some(&ch) = self.bufff.get(i) {
            if speed_str.push(ch).is_err() {
                return None;
            }
        }
    }
    
    let speed_knots: f64 = speed_str.parse().ok()?;
    let speed_kmph = speed_knots * 1.852;
    if speed_kmph < 50.0  || speed_kmph > 180.{
        return Some(0.0);  
    }
    Some(speed_kmph)
}

async fn latitude_longitude(&self) -> Option<(f64, f64)> {
    let lat = self.latitude().await?;
    let lon = self.longitude().await?;
    Some((lat, lon))
}

    async fn update(&mut self) -> Result<bool, UART::Error> {
        let mut buf = [0u8; 1];
        match self.uart.read(&mut buf).await {
            Ok(1) => Ok(self.encode(buf[0]).await),
            Ok(_) => Ok(false),
            Err(e) => Err(e),
        }
    }
}
extern crate alloc;

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 72 * 1024);

    let timer0 = TimerGroup::new(peripherals.TIMG1);
    esp_hal_embassy::init(timer0.timer0);

    info!("Embassy initialized!");

    let timer1 = TimerGroup::new(peripherals.TIMG0);
    let _init = esp_wifi::init(
        timer1.timer0,
        esp_hal::rng::Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
    )
    .unwrap();
let uart_config: Config = Config::default()
.with_baudrate(38400);

let uart = Uart::new(peripherals.UART2, uart_config).unwrap()
    .with_rx(peripherals.GPIO32)
    .with_tx(peripherals.GPIO33)
    .into_async();

let mut bufff: Vec<_, 128> = Vec::new();
let mut gps = TinyGPS { uart, bufff };
    // TODO: Spawn some tasks
    let _ = spawner;

    loop {
        match gps.update().await {
            Ok(true) => {
                match gps.latitude_longitude().await {
                    Some(bob ) =>{
                        info!("Longitude and latitude is {}  longitude {} ",bob.0, bob.1);
                    }None =>{
                    }
                    
                }
                match gps.altitude().await {
                    Some(bob ) =>{
                        info!("altitude is ->{} ",bob);
                    }None =>{
                    }
                    
                }

                match gps.speed().await {
                    Some(bob ) =>{
                        info!("Speed is ->{} MIGHT NOT BE ACCAURATE INDOORS ",bob);
                    }None =>{
                    }
                    
                }
            }
            Ok(false) => {
            
            }
            Err(e) => {
                
            }
        }
        Timer::after_millis(5).await; 
    }

}
