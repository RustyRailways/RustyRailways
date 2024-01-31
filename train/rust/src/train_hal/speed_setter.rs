use esp_idf_svc::hal;
use hal::gpio::{Gpio27,Gpio26,Gpio25,PinDriver,Output};
use hal::ledc::{config::TimerConfig, LedcDriver, LedcTimerDriver, Resolution,TIMER0, CHANNEL0};
use hal::prelude::FromValueType;
use anyhow::Result;
use crate::SPEED_OFFSET;

enum Direction{
    Forward,
    Backward,
}

impl From<i8> for Direction {
    fn from(value: i8) -> Self {
        if value > 0{
            return Direction::Forward;
        }else{
            return  Direction::Backward;
        }
    }
}

pub struct MotorDriver<'a>{
    pwm_driver: LedcDriver<'a>,
    forward_pin: PinDriver<'a,Gpio26,Output>,
    backward_pin: PinDriver<'a,Gpio27,Output>
}

const MAX_PWM: u32 = 2u32.pow(14);

impl MotorDriver<'_> {
    pub fn new(timer: TIMER0, pwm_pin: Gpio25, forward_pin: Gpio26, backward_pin: Gpio27, channel: CHANNEL0) -> Result<Self>{

        let timer_config: TimerConfig = TimerConfig::default()
            .frequency(50.Hz())
            .resolution(Resolution::Bits14);

        let mut forward_pin =  PinDriver::output(forward_pin)?;
        let mut backward_pin =  PinDriver::output(backward_pin)?;
        let timer = LedcTimerDriver::new(timer,&timer_config)?;
        let mut pwm_driver = LedcDriver::new(channel,timer,pwm_pin)?;
        pwm_driver.set_duty(0)?;
        forward_pin.set_low()?;
        backward_pin.set_low()?;
        return Ok(Self{forward_pin,backward_pin,pwm_driver});
    }

    pub fn set_speed(&mut self, mut speed: i8) -> Result<()>{
        let direction: Direction = speed.into();
        // max(-127) is to avoid overflow
        // for locomotive 2
        speed=((speed as f32)*SPEED_OFFSET)as i8;

        let mut speed_abs = speed.max(-127).abs() as u32;
        speed_abs = speed_abs*MAX_PWM/127;

        match direction {
            Direction::Backward => {
                self.forward_pin.set_low()?;
                self.backward_pin.set_high()?;
            }
            Direction::Forward =>{
                self.backward_pin.set_low()?;
                self.forward_pin.set_high()?;
            }
        };

        self.pwm_driver.set_duty(speed_abs)?;

        return Ok(());
    }

    /*
    pub fn emergency_stop(&mut self){
        let _ = self.backward_pin.set_low();
        let _ = self.forward_pin.set_low();
        let _ =  self.pwm_driver.set_duty(0);
    }*/
}