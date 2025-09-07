#![no_std]
#![no_main]

use defmt::info;
use embedded_io_async::{Read, Write};
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::{clock::CpuClock, peripheral::Peripheral};
use esp_hal::timer::timg::TimerGroup;
use esp_println as _;
use esp_hal::uart::{Uart, Config};
use heapless::{String, Vec};
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

const _GPS_VERSION : i32 = 13;
const _GPS_MPH_PER_KNOT :f32 = 1.15077945;

const _GPS_MPS_PER_KNOT :f32 =  0.51444444;
const _GPS_KMPH_PER_KNOT : f32 = 1.852;
const _GPS_MILES_PER_METER : f32 = 0.00062137112;
const _GPS_KM_PER_METER: f32 =  0.001;


struct TinyGPS<UART> {
    uart: UART,
}

impl<UART> TinyGPS<UART>
where 
    UART: Read + Write,
{
   async  fn encode(&mut self, c: u8) -> bool {
        
    // call every single one and return with option true or false with the f64 and then match statment for each
    
     }
    
     async  fn latitude(&self) -> Option<f64>{

     }
     async  fn longitude(&self) -> Option<f64>{

     }
     async  fn altitude(&self) -> Option<f64>{

     }
     async  fn speed(&self) -> Option<f64>{

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

let mut gps = TinyGPS { uart };
    // TODO: Spawn some tasks
    let _ = spawner;

    loop {
        match gps.update().await {
            Ok(true) => {
         // reading gps data
            }
            Ok(false) => {
            
            }
            Err(e) => {
                
            }
        }
        Timer::after_millis(10).await; 
    }

}
