use mfrc522::{Mfrc522,Uid, Initialized};
use mfrc522::comm::eh02::spi::{SpiInterface,DummyNSS,DummyDelay};

use log::error;
use esp_idf_svc::hal;

use hal::spi::{SPI2,SpiDriver,SpiDeviceDriver};
use hal::gpio::{Gpio15,Gpio16,Gpio17,Gpio18};
use hal::units::Hertz;

use anyhow::Result;

pub struct NfcSpiDriver<'a>{
    driver: SpiDriver<'a>,
}

impl<'a> NfcSpiDriver<'a>{

    pub fn new(spi: SPI2,sclk: Gpio15, serial_in: Gpio16, serial_out: Gpio17)->Result<Self>{
        
        let driver = SpiDriver::new::<SPI2>(
            spi,
            sclk,
            serial_out,
            Some(serial_in),
            &hal::spi::SpiDriverConfig::new(),
        )?;

        Ok(Self{driver})
    }
}

pub struct NfcReader<'a>{
    reader: Mfrc522<SpiInterface<SpiDeviceDriver<'a, &'a SpiDriver<'a>>, DummyNSS, DummyDelay>, Initialized>
}

impl<'a> NfcReader<'a>{

    pub fn new(cs_1: Gpio18, driver: &'a NfcSpiDriver)->Result<Self>{
        
        let reader = hal::spi::config::Config::new().baudrate(Hertz::from(10_000));
        let reader = SpiDeviceDriver::new(&driver.driver, Some(cs_1), &reader)?;
        let reader = SpiInterface::new(reader);
        let reader = Mfrc522::new(reader).init();

        let reader = match reader {
            Ok(r) => r,
            Err(e) => return Err(anyhow::anyhow!("Initialization of nfc reader failed: {:?}",e))
        };

        return Ok(NfcReader{reader})
    }


    pub fn try_get_tag(&mut self) -> Option<[u8;4]>{
        let atqa = match self.reader.reqa(){
            Result::Ok(v) => v,
            Err(_) => return None
        };
        let uid = match self.reader.select(&atqa){
            Result::Ok(v) => v,
            Err(_) => return None
        };
        let ids: &[u8] = match &uid {
            Uid::Single(v) => v.as_bytes(),
            _ => {
                error!("Red tag with more that 4 IDs: {:?}",uid.as_bytes());
                return None;
            }
        };
        let ids: &[u8;4] = ids[0..4].try_into().unwrap();
        return Some(*ids);
    }
    
}
