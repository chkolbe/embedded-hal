//! OneWire Interface

use nb;

use private;

impl private::Sealed for RomId {}
impl private::Sealed for Command {}

/// 64-bit address mode type
pub type RomId = u64;

/// OneWire Command to OneWire Device
#[derive(Clone)]
#[repr(u8)]
#[warn(dead_code)]
pub enum Command {
    /// Skip Search ROM Function - Only One Device connected is supported!
    SkipRom = 0xcc,
    /// DS18S20 Command to start Temperature Conversion
    ConvertTemperature = 0x44,
    /// DS18S20 Command to read the Scratch Pad from Memory
    ReadScratchPad = 0xbe,
}

impl From<u8> for Command {
    fn from(item: u8) -> Self {
        match item {
            0xcc => Command::SkipRom,
            0x44 => Command::ConvertTemperature,
            0xbe => Command::ReadScratchPad,
            _ => Command::SkipRom,
        }
    }
}

impl Into<u8> for Command {
    fn into(self) -> u8 {
        match self {
            Command::SkipRom => 0xcc,
            Command::ConvertTemperature => 0x44,
            Command::ReadScratchPad => 0xbe,
        }
    }
}

/// OneWire Master Mode
///
/// # Notes
///
/// -
pub trait OneMaster {
    /// An enumeration of OneWire errors
    type Error;

    /// Does Bus Reset and syncs the Slaves
    ///
    /// **NOTE** Bus Reset should be done before any Slave Interaction.
    fn bus_reset(&mut self) -> nb::Result<(), Self::Error>;

    /// Write the OneWire Command on the Bus.
    fn write(&mut self, rom_id: RomId, command: Command) -> nb::Result<(), Self::Error>;

    /// Read the Payload from the Bus.
    ///
    /// **NOTE** A Slave must be select and command must been sent before the Slave response.
    fn read(&mut self, rom_id: RomId, buffer: &mut [u8]) -> nb::Result<(), Self::Error>;
}
