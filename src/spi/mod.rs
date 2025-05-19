use embedded_hal::spi::Operation;

use crate::{DataRate, FifoMode, Register, FIFO_SIZE_BYTES};

pub struct ADXL375<SPI, DELAY>
where
    SPI: embedded_hal::spi::SpiDevice,
    DELAY: embedded_hal_async::delay::DelayNs,
{
    spi: SPI,
    delay: DELAY,
}

#[derive(Debug, Clone)]
pub enum Error<T> {
    InvalidBuffer,
    SpiError(T),
}

impl<T> From<T> for Error<T> {
    fn from(err: T) -> Self {
        Error::SpiError(err)
    }
}


impl<SPI, DELAY> ADXL375<SPI, DELAY>
where
    SPI: embedded_hal::spi::SpiDevice,
    DELAY: embedded_hal_async::delay::DelayNs,
{
    pub fn new(spi: SPI, delay: DELAY) -> Self {
        Self { spi, delay }
    }

    pub async fn read<'a>(
        &mut self,
        reg: Register,
        buf: &'a mut [u8],
    ) -> Result<&'a mut [u8], Error<SPI::Error>> {
        let mut cmd = reg as u8 | 0x80; // Set the read bit
        if buf.len() > 1 {
            cmd |= 0x40; // Set multi-byte read bit
        }
        self.spi
            .transaction(&mut [Operation::Write(&[cmd]), Operation::Read(buf)])?;
        Ok(buf)
    }

    pub async fn read_single(&mut self, reg: Register) -> Result<u8, Error<SPI::Error>> {
        let mut buf = [0];
        self.read(reg, &mut buf).await?;
        Ok(buf[0])
    }

    pub async fn write(&mut self, reg: Register, buf: &[u8]) -> Result<(), Error<SPI::Error>> {
        let mut cmd = reg as u8 & 0x7F; // Clear the read bit
        if buf.len() > 1 {
            cmd |= 0x40; // Set multi-byte write bit
        }
        self.spi
            .transaction(&mut [Operation::Write(&[cmd]), Operation::Write(buf)])?;

        Ok(())
    }

    pub async fn set_data_rate(&mut self, rate: DataRate) -> Result<(), Error<SPI::Error>> {
        let mut bw_rate = self.read_single(Register::BwRate).await?;
        bw_rate &= 0xF0; // Clear the data rate bits
        bw_rate |= rate as u8; // Set the new data rate
        self.write(Register::BwRate, &[bw_rate]).await?;
        Ok(())
    }

    pub async fn set_fifo_mode(&mut self, mode: FifoMode) -> Result<(), Error<SPI::Error>> {
        let mut fifo_ctl = self.read_single(Register::FifoCtl).await?;
        fifo_ctl &= 0x3F; // Clear the FIFO mode bits
        fifo_ctl |= mode as u8; // Set the new FIFO mode

        self.write(Register::FifoCtl, &[fifo_ctl]).await?;
        Ok(())
    }

    pub async fn read_fifo(
        &mut self,
    ) -> Result<heapless::Vec<u8, FIFO_SIZE_BYTES>, Error<SPI::Error>> {
        let bw_rate = self.read_single(Register::FifoCtl).await?;
        let samples = bw_rate & 0x1F;
        let mut buf = heapless::Vec::new();
        buf.resize(6 * samples as usize, 0).unwrap();

        for sample in buf.chunks_exact_mut(6) {
            self.read(Register::DataX0, sample).await?;
            // Datasheet requests a 5us delay between samples
            self.delay.delay_us(5).await;
        }

        Ok(buf)
    }

    pub fn release(self) -> SPI {
        self.spi
    }
}
