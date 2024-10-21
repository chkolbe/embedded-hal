//! Async OneWire API

pub use embedded_hal::onewire::{Operation, Error, ErrorKind, ErrorType, Command, RomId};

/// Async OneWire
pub trait OneWire: ErrorType {

    async fn bus_reset<'a>(&'a mut self) -> Result<(), Self::Error>;

    /// Reads enough bytes from device with `rom_id` to fill `buffer`
    async fn read<'a>(&'a mut self, rom_id: RomId, read: &'a mut [u8]) -> Result<(), Self::Error>;

    /// Writes bytes to device with RomId `rom_id`
    async fn write<'a>(&'a mut self, rom_id: RomId, write: &'a [u8]) -> Result<(), Self::Error>;

    /// Writes bytes to device with RomId `rom_id` and then reads enough bytes to fill `read` *in a
    async fn write_read<'a>(
        &'a mut self,
        rom_id: RomId,
        write: &'a [u8],
        read: &'a mut [u8],
    ) -> Result<(), Self::Error>;

    /// Execute the provided operations on the OneWire bus as a single transaction.
    async fn transaction<'a, 'b>(
        &'a mut self,
        operations: &'a mut [Operation<'b>],
    ) -> Result<(), Self::Error>;
}

impl<T: OneWire> OneWire for &mut T {

    async fn bus_reset<'a>(&'a mut self) -> Result<(), Self::Error> {
        T::bus_reset(self).await
    }

    async fn read<'a>(&'a mut self, rom_id: RomId, buffer: &'a mut [u8]) -> Result<(), Self::Error> {
        T::read(self, rom_id, buffer).await
    }

    async fn write<'a>(&'a mut self, rom_id: RomId, bytes: &'a [u8]) -> Result<(), Self::Error> {
        T::write(self, rom_id, bytes).await
    }

    async fn write_read<'a>(
        &'a mut self,
        rom_id: RomId,
        bytes: &'a [u8],
        buffer: &'a mut [u8],
    ) -> Result<(), Self::Error> {
        T::write_read(self, rom_id, bytes, buffer).await
    }

    async fn transaction<'a, 'b>(
        &'a mut self,
        operations: &'a mut [Operation<'b>],
    ) -> Result<(), Self::Error> {
        T::transaction(self, operations).await
    }
}
