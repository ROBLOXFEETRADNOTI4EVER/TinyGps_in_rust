# ESP32 GPS Parser

GPS parser for FAKE aliexpress counterfit NEO-M10-10-O modules (the ones that dont actually exist but work anyway)

## What this does

Embeded rust code for esp32 using Embassy framework to read gps data from uart and parse nmea sentences

Gets latitude longitude altitude speed from gps module connected to esp32

## Hardware stuff

- ESP32 board 
- Fake GPS module from aliexpress (any nmea one works)
- Connect gps tx to gpio32 
- Connect gps rx to gpio33
- Ground connection

Baud rate is 38400 but you can change it if your module is different

## Features that work

- reads nmea sentences over uart
- parses lat/lon from GNGLL sentences  
- gets altitude from GPGGA sentences
- gets speed from GPRMC sentences
- checksum validation so you know data isnt corrupted
- async uart reading with embassy framework

## Building

Need esp32 rust setup first then just:

