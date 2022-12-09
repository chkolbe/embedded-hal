//! Blocking OneWire API

use onewire::{Command, RomId};

/// Blocking read
pub trait Read {
    /// Error type
    type Error;

    /// Reads enough bytes from slave with `address` to fill `buffer`
    ///
    /// # OneWire Events (contract)
    ///
    /// ``` text
    /// Master: RST    SKR/ROM      CMD                   ...    
    /// Slave:     SAK          SAK     SAK B0 SAK B1 SAK ... BN SAK
    /// ```
    ///
    /// Where
    ///
    /// - `RST` = start condition
    /// - `SKR/ROM` = Skip Rom (Slave Identification) OR Send ROM-ID on the Bus
    /// - 'CMD' = Command to execute for the Slave (like Update Buffer)
    /// - `SAK` = slave acknowledge
    /// - 'NSAK' = slave no acknowledge
    /// - `Bi` = ith byte of data
    /// - `MAK` = master acknowledge
    /// - `NMAK` = master no acknowledge
    fn read(&mut self, rom_id: RomId, buffer: &mut [u8]) -> Result<(), Self::Error>;
}

/// Blocking write
pub trait Write {
    /// Error type
    type Error;

    /// Writes bytes to slave with address `address`
    ///
    /// # OneWire Events (contract)
    ///
    /// ``` text
    /// Master: RST    SKR/ROM      CMD     B0     B1     ... BN    
    /// Slave:     SAK          SAK     SAK    SAK    SAK ...    SAK
    /// ```
    ///
    /// Where
    ///
    /// - `RST` = start condition
    /// - `SKR/ROM` = Skip Rom (Slave Identification) OR Send ROM-ID on the Bus
    /// - 'CMD' = Command to execute for the Slave (like Update Buffer)
    /// - `SAK` = slave acknowledge
    /// - 'NSAK' = slave no acknowledge
    /// - `Bi` = ith byte of data
    /// - `MAK` = master acknowledge
    /// - `NMAK` = master no acknowledge
    fn write(&mut self, rom_id: RomId, command: Command) -> Result<(), Self::Error>;
}

/// Blocking write
pub mod read {
    use blocking::onewire::Read;
    use onewire::{OneMaster, RomId};

    /// Default implementation of `blocking::onewire::Write<Payload>` for implementers of `onewire::OneMaster<Payload>`
    pub trait Default: OneMaster {}

    impl<S> Read for S
    where
        S: Default,
    {
        type Error = S::Error;

        fn read(&mut self, rom_id: RomId, buffer: &mut [u8]) -> Result<(), S::Error> {
            nb::block!(self.read(rom_id, buffer))
        }
    }
}

/// Blocking write
pub mod write {
    use blocking::onewire::Write;
    use onewire::{Command, OneMaster, RomId};

    /// Default implementation of `blocking::onewire::Write<Payload>` for implementers of `onewire::OneMaster<Payload>`
    pub trait Default: OneMaster {}

    impl<S> Write for S
    where
        S: Default,
    {
        type Error = S::Error;

        fn write(&mut self, rom_id: RomId, command: Command) -> Result<(), S::Error> {
            nb::block!(self.write(rom_id, command.clone()))?;

            Ok(())
        }
    }
}
