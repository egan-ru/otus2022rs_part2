//! Module for Smarthome Thermometer
//! show status
use std::fmt;

/// Smarthome thermometer
/// 
/// *'text'  - device description
/// *'temp'  - current temperature in °K
pub struct SmartThermometer {
    pub text: String,       // device description
    pub temp: u16,          // device temperature in °K
}

impl SmartThermometer {
    /*** interface ***/

    /// Thermometer ctor
    ///
    /// *'thermometer_text'  - thermometer description 
    ///
    /// *'return'     - new socket instance
    pub fn new(thermometer_text: &str) ->Self {
        let thermometer : SmartThermometer = SmartThermometer {
            text : thermometer_text.to_string(),
            temp : 273,
        };        
        return thermometer;
    }

    /// Update thermometer status
    pub fn update(&mut self) {
        /* need to update the current state of thermometer */
        self.temp = 273 + 20;
    }
}

impl fmt::Display for SmartThermometer {
    /// Thermometer print implementation
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let out_s = &format!("Name = {}\tTemp = {}",
                             self.text,
                             self.temp);
        fmt.write_str(out_s)?;
        Ok(())
    }
}