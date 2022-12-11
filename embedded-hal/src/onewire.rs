//! Blocking Onewire API

/// Onewire error
pub trait Error: core::fmt::Debug {
    /// Convert error to a generic Onewire error kind
    ///
    /// By using this method, Onewire errors freely defined by HAL implementations
    /// can be converted to a set of generic Onewire errors upon which generic
    /// code can act.
    fn kind(&self) -> ErrorKind;
}

impl Error for core::convert::Infallible {
    fn kind(&self) -> ErrorKind {
        match *self {}
    }
}

/// I2C error kind
///
/// This represents a common set of I2C operation errors. HAL implementations are
/// free to define more specific or additional error types. However, by providing
/// a mapping to these common I2C errors, generic code can still react to them.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum ErrorKind {
    /// A bus reset operation was not acknowledged with device present, 
    /// e.g. no Device connected to the Bus.
    NoDevicePresence,
    /// Device with this RomId not found.
    RomNotFound(RomId),
    /// A different error occurred. The original error may contain more information.
    Other,
}

impl Error for ErrorKind {
    fn kind(&self) -> ErrorKind {
        *self
    }
}

impl core::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NoDevicePresence => write!(f, "Bus Reset without Device Present Ack!"),
            Self::RomNotFound(rid) => rid.fmt(f),
            Self::Other => write!(
                f,
                "A different error occurred. The original error may contain more information"
            ),
        }
    }
}

/// Onewire error type trait
///
/// This just defines the error type, to be used by the other traits.
pub trait ErrorType {
    /// Error type
    type Error: Error;
}

impl<T: ErrorType> ErrorType for &mut T {
    type Error = T::Error;
}

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

/// Transactional Onewire operation.
///
/// Several operations can be combined as part of a transaction.
#[derive(Debug, PartialEq, Eq)]
pub enum Operation<'a> {
    /// Search Device with Matchin RomId
    Search(&'a RomId),
    /// Read data into the provided buffer
    Read(&'a mut [u8]),
    /// Write data from the provided buffer
    Write(&'a [u8]),
}

/// Blocking Onewire
pub trait OneWire: ErrorType {
    /// Does Bus Reset and syncs the Slaves
    ///
    /// **NOTE** Bus Reset should be done before any Slave Interaction.
    fn bus_reset(&mut self) -> Result<(), Self::Error>;

    /// Write the OneWire Command on the Bus.
    fn write(&mut self, rom_id: RomId, command: Command) -> Result<(), Self::Error>;

    /// Read the Payload from the Bus.
    ///
    /// **NOTE** A Slave must be select and command must been sent before the Slave response.
    fn read(&mut self, rom_id: RomId, buffer: &mut [u8]) -> Result<(), Self::Error>;

    /// Writes bytes to device with RomId `rom_id`
    ///
    /// # OneWire Events (contract)
    ///
    /// Same as the `write` method
    fn write_iter<B>(&mut self, rom_id: RomId, bytes: B) -> Result<(), Self::Error>
    where
        B: IntoIterator<Item = u8>;

    /// Writes bytes to device with RomId `rom_id` and then reads enough bytes to fill `buffer` *in a
    /// single transaction*
    ///
    /// # OneWire Events (contract)
    fn write_read(
        &mut self,
        rom_id: RomId,
        bytes: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), Self::Error>;

    /// Writes bytes to device with RomId `rom_id` and then reads enough bytes to fill `buffer` *in a
    /// single transaction*
    ///
    /// # OneWire Events (contract)
    ///
    /// Same as the `write_read` method
    fn write_iter_read<B>(
        &mut self,
        rom_id: RomId,
        bytes: B,
        buffer: &mut [u8],
    ) -> Result<(), Self::Error>
    where
        B: IntoIterator<Item = u8>;

    /// Execute the provided operations on the OneWire bus.
    fn transaction<'a>(
        &mut self,
        operations: &mut [Operation<'a>],
    ) -> Result<(), Self::Error>;

    /// Execute the provided operations on the I2C bus (iterator version).
    fn transaction_iter<'a, O>(&mut self, operations: O) -> Result<(), Self::Error>
    where
        O: IntoIterator<Item = Operation<'a>>;
}

//impl<A: AddressMode, T: I2c<A>> I2c<A> for &mut T {
impl<T: OneWire> OneWire for &mut T {
    
    fn bus_reset(&mut self) -> Result<(), Self::Error> {
        T::bus_reset(self)
    }
    
    fn read(&mut self, rom_id: RomId, buffer: &mut [u8]) -> Result<(), Self::Error> {
        T::read(self, rom_id, buffer)
    }

    fn write(&mut self, rom_id: RomId, command: Command) -> Result<(), Self::Error> {
        T::write(self, rom_id, command)
    }

    fn write_iter<B>(&mut self, rom_id: RomId, bytes: B) -> Result<(), Self::Error>
    where
        B: IntoIterator<Item = u8>,
    {
        T::write_iter(self, rom_id, bytes)
    }

    fn write_read(
        &mut self,
        rom_id: RomId,
        bytes: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        T::write_read(self, rom_id, bytes, buffer)
    }

    fn write_iter_read<B>(
        &mut self,
        rom_id: RomId,
        bytes: B,
        buffer: &mut [u8],
    ) -> Result<(), Self::Error>
    where
        B: IntoIterator<Item = u8>,
    {
        T::write_iter_read(self, rom_id, bytes, buffer)
    }

    fn transaction<'a>(
        &mut self,
        operations: &mut [Operation<'a>],
    ) -> Result<(), Self::Error> {
        T::transaction(self, operations)
    }

    fn transaction_iter<'a, O>(&mut self, operations: O) -> Result<(), Self::Error>
    where
        O: IntoIterator<Item = Operation<'a>>,
    {
        T::transaction_iter(self, operations)
    }
}
