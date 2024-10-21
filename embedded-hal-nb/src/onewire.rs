//! OneWire traits using `nb`.

pub use embedded_hal::onewire::{Operation, Error, ErrorKind, ErrorType, Command, RomId};

/// Blocking Onewire
pub trait OneWire: ErrorType {
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

    /// Writes bytes to device with RomId `rom_id`
    ///
    /// # OneWire Events (contract)
    ///
    /// Same as the `write` method
    fn write_iter<B>(&mut self, rom_id: RomId, bytes: B) -> nb::Result<(), Self::Error>
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
    ) -> nb::Result<(), Self::Error>;

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
    ) -> nb::Result<(), Self::Error>
    where
        B: IntoIterator<Item = u8>;

    /// Execute the provided operations on the OneWire bus.
    fn transaction<'a>(
        &mut self,
        operations: &mut [Operation<'a>],
    ) -> nb::Result<(), Self::Error>;

    /// Execute the provided operations on the I2C bus (iterator version).
    fn transaction_iter<'a, O>(&mut self, operations: O) -> nb::Result<(), Self::Error>
    where
        O: IntoIterator<Item = Operation<'a>>;
}

//impl<A: AddressMode, T: I2c<A>> I2c<A> for &mut T {
impl<T: OneWire> OneWire for &mut T {

    fn bus_reset(&mut self) -> nb::Result<(), Self::Error> {
        T::bus_reset(self)
    }

    fn read(&mut self, rom_id: RomId, buffer: &mut [u8]) -> nb::Result<(), Self::Error> {
        T::read(self, rom_id, buffer)
    }

    fn write(&mut self, rom_id: RomId, command: Command) -> nb::Result<(), Self::Error> {
        T::write(self, rom_id, command)
    }

    fn write_iter<B>(&mut self, rom_id: RomId, bytes: B) -> nb::Result<(), Self::Error>
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
    ) -> nb::Result<(), Self::Error> {
        T::write_read(self, rom_id, bytes, buffer)
    }

    fn write_iter_read<B>(
        &mut self,
        rom_id: RomId,
        bytes: B,
        buffer: &mut [u8],
    ) -> nb::Result<(), Self::Error>
    where
        B: IntoIterator<Item = u8>,
    {
        T::write_iter_read(self, rom_id, bytes, buffer)
    }

    fn transaction<'a>(
        &mut self,
        operations: &mut [Operation<'a>],
    ) -> nb::Result<(), Self::Error> {
        T::transaction(self, operations)
    }

    fn transaction_iter<'a, O>(&mut self, operations: O) -> nb::Result<(), Self::Error>
    where
        O: IntoIterator<Item = Operation<'a>>,
    {
        T::transaction_iter(self, operations)
    }
}
