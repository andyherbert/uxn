use std::{
    error::Error,
    time::{Instant, SystemTime, UNIX_EPOCH},
};
use tz::{error::DateTimeError, DateTime, TimeZone, TzError, UtcDateTime};

pub enum DeviceSystemTime {
    Local,
    Utc,
    Custom {
        date_time: DeviceDateTime,
        from: Instant,
    },
    Static(DeviceDateTime),
}

impl DeviceSystemTime {
    pub fn new(date_time: DeviceDateTime) -> DeviceSystemTime {
        DeviceSystemTime::Custom {
            date_time,
            from: Instant::now(),
        }
    }
}

#[derive(Clone)]
pub struct DeviceDateTime {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    weekday: u8,
    day_of_the_year: u16,
}

impl Default for DeviceDateTime {
    fn default() -> DeviceDateTime {
        DeviceDateTime::unix_epoch()
    }
}

impl DeviceDateTime {
    pub fn new(unix_time: i64) -> Result<DeviceDateTime, Box<dyn Error>> {
        let date_time = UtcDateTime::from_timespec(unix_time, 0)?;
        Ok(DeviceDateTime {
            year: u16::try_from(date_time.year())?,
            month: date_time.month() - 1,
            day: date_time.month_day(),
            hour: date_time.hour(),
            minute: date_time.minute(),
            second: date_time.second(),
            weekday: date_time.week_day(),
            day_of_the_year: date_time.year_day(),
        })
    }

    pub fn unix_epoch() -> DeviceDateTime {
        DeviceDateTime::new(0).expect("valid unix time")
    }

    pub fn utc() -> Result<DeviceDateTime, Box<dyn Error>> {
        let now = SystemTime::now();
        let unix_time = now.duration_since(UNIX_EPOCH)?.as_secs();
        DeviceDateTime::new(unix_time as i64)
    }

    pub fn local() -> Result<DeviceDateTime, Box<dyn Error>> {
        let time = TimeZone::local()?;
        let local_time_type = time.find_current_local_time_type()?;
        let now = SystemTime::now();
        let unix_time = now.duration_since(UNIX_EPOCH)?.as_secs();
        let date_time = DateTime::from_timespec_and_local(unix_time as i64, 0, *local_time_type)?;
        DeviceDateTime::new(date_time.unix_time())
    }

    pub fn unix_time(&self) -> Result<i64, DateTimeError> {
        let time = UtcDateTime::new(
            self.year as i32,
            self.month + 1,
            self.day,
            self.hour,
            self.minute,
            self.second,
            0,
        )?;
        Ok(time.unix_time())
    }

    fn from_then(self: &DeviceDateTime, from: &Instant) -> Result<DeviceDateTime, Box<dyn Error>> {
        let unix_time = self.unix_time()?;
        let since_epoch = (Instant::now() - *from).as_secs() as i64;
        DeviceDateTime::new(unix_time + since_epoch)
    }

    fn is_dst() -> Result<bool, TzError> {
        let time_zone = TimeZone::local()?;
        let local_time_type = time_zone.find_current_local_time_type()?;
        Ok(local_time_type.is_dst())
    }
}

pub fn device_input_u8(port: u8, date_time_type: &DeviceSystemTime) -> u8 {
    let date_time = match date_time_type {
        DeviceSystemTime::Local => DeviceDateTime::local().unwrap_or_default(),
        DeviceSystemTime::Utc => DeviceDateTime::utc().unwrap_or_default(),
        DeviceSystemTime::Custom { date_time, from } => {
            date_time.from_then(from).unwrap_or_default()
        }
        DeviceSystemTime::Static(date_time) => date_time.clone(),
    };
    match port {
        // Year
        0x00 => (date_time.year >> 8) as u8,
        0x01 => (date_time.year & 0xff) as u8,
        // Month
        0x02 => date_time.month,
        //  Day
        0x03 => date_time.day,
        // Hour
        0x04 => date_time.hour,
        // Minute
        0x05 => date_time.minute,
        // Second
        0x06 => date_time.second,
        // Day of the week
        0x07 => date_time.weekday,
        // Day of the year
        0x08 => (date_time.day_of_the_year >> 8) as u8,
        0x09 => (date_time.day_of_the_year & 0xff) as u8,
        // Is daylight savings
        0x0a => match date_time_type {
            DeviceSystemTime::Local => {
                if DeviceDateTime::is_dst().unwrap_or(false) {
                    1
                } else {
                    0
                }
            }
            _ => 0,
        },
        _ => 0,
    }
}
