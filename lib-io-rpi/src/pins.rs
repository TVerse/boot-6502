use gpio_cdev::{Line, LineHandle, LineRequestFlags};
use lib_io::IoError;
use std::marker::PhantomData;

use lib_io::Result;
use std::convert::TryFrom;

pub struct Input;
pub struct Output;

pub struct IncomingHandshake {
    pub line: Line,
    pub handle: LineHandle,
}
pub struct OutgoingHandshake {
    pub line: Line,
    pub handle: LineHandle,
}
pub struct Reset {
    pub line: Line,
    pub handle: LineHandle,
}

pub struct P0<A> {
    pub line: Line,
    pub handle: LineHandle,
    pub _pd: PhantomData<A>,
}
pub struct P1<A> {
    pub line: Line,
    pub handle: LineHandle,
    pub _pd: PhantomData<A>,
}
pub struct P2<A> {
    pub line: Line,
    pub handle: LineHandle,
    pub _pd: PhantomData<A>,
}
pub struct P3<A> {
    pub line: Line,
    pub handle: LineHandle,
    pub _pd: PhantomData<A>,
}
pub struct P4<A> {
    pub line: Line,
    pub handle: LineHandle,
    pub _pd: PhantomData<A>,
}
pub struct P5<A> {
    pub line: Line,
    pub handle: LineHandle,
    pub _pd: PhantomData<A>,
}
pub struct P6<A> {
    pub line: Line,
    pub handle: LineHandle,
    pub _pd: PhantomData<A>,
}
pub struct P7<A> {
    pub line: Line,
    pub handle: LineHandle,
    pub _pd: PhantomData<A>,
}

pub trait InputPin {
    fn is_high(&self) -> Result<bool>;
    fn is_low(&self) -> Result<bool> {
        self.is_high().map(|b| !b)
    }
}

pub trait OutputPin {
    fn set_high(&mut self) -> Result<()>;

    fn set_low(&mut self) -> Result<()>;
}

impl TryFrom<P0<Input>> for P0<Output> {
    type Error = gpio_cdev::Error;

    fn try_from(value: P0<Input>) -> std::result::Result<Self, Self::Error> {
        drop(value.handle);
        let output_handle = value.line.request(LineRequestFlags::OUTPUT, 0, "P0_out")?;
        Ok(Self {
            line: value.line,
            handle: output_handle,
            _pd: PhantomData,
        })
    }
}
impl TryFrom<P1<Input>> for P1<Output> {
    type Error = gpio_cdev::Error;

    fn try_from(value: P1<Input>) -> std::result::Result<Self, Self::Error> {
        drop(value.handle);
        let output_handle = value.line.request(LineRequestFlags::OUTPUT, 0, "P0_out")?;
        Ok(Self {
            line: value.line,
            handle: output_handle,
            _pd: PhantomData,
        })
    }
}
impl TryFrom<P2<Input>> for P2<Output> {
    type Error = gpio_cdev::Error;

    fn try_from(value: P2<Input>) -> std::result::Result<Self, Self::Error> {
        drop(value.handle);
        let output_handle = value.line.request(LineRequestFlags::OUTPUT, 0, "P0_out")?;
        Ok(Self {
            line: value.line,
            handle: output_handle,
            _pd: PhantomData,
        })
    }
}
impl TryFrom<P3<Input>> for P3<Output> {
    type Error = gpio_cdev::Error;

    fn try_from(value: P3<Input>) -> std::result::Result<Self, Self::Error> {
        drop(value.handle);
        let output_handle = value.line.request(LineRequestFlags::OUTPUT, 0, "P0_out")?;
        Ok(Self {
            line: value.line,
            handle: output_handle,
            _pd: PhantomData,
        })
    }
}
impl TryFrom<P4<Input>> for P4<Output> {
    type Error = gpio_cdev::Error;

    fn try_from(value: P4<Input>) -> std::result::Result<Self, Self::Error> {
        drop(value.handle);
        let output_handle = value.line.request(LineRequestFlags::OUTPUT, 0, "P0_out")?;
        Ok(Self {
            line: value.line,
            handle: output_handle,
            _pd: PhantomData,
        })
    }
}
impl TryFrom<P5<Input>> for P5<Output> {
    type Error = gpio_cdev::Error;

    fn try_from(value: P5<Input>) -> std::result::Result<Self, Self::Error> {
        drop(value.handle);
        let output_handle = value.line.request(LineRequestFlags::OUTPUT, 0, "P0_out")?;
        Ok(Self {
            line: value.line,
            handle: output_handle,
            _pd: PhantomData,
        })
    }
}
impl TryFrom<P6<Input>> for P6<Output> {
    type Error = gpio_cdev::Error;

    fn try_from(value: P6<Input>) -> std::result::Result<Self, Self::Error> {
        drop(value.handle);
        let output_handle = value.line.request(LineRequestFlags::OUTPUT, 0, "P0_out")?;
        Ok(Self {
            line: value.line,
            handle: output_handle,
            _pd: PhantomData,
        })
    }
}
impl TryFrom<P7<Input>> for P7<Output> {
    type Error = gpio_cdev::Error;

    fn try_from(value: P7<Input>) -> std::result::Result<Self, Self::Error> {
        drop(value.handle);
        let output_handle = value.line.request(LineRequestFlags::OUTPUT, 0, "P0_out")?;
        Ok(Self {
            line: value.line,
            handle: output_handle,
            _pd: PhantomData,
        })
    }
}

impl TryFrom<P0<Output>> for P0<Input> {
    type Error = gpio_cdev::Error;

    fn try_from(value: P0<Output>) -> std::result::Result<Self, Self::Error> {
        drop(value.handle);
        let output_handle = value.line.request(LineRequestFlags::INPUT, 0, "P0_in")?;
        Ok(Self {
            line: value.line,
            handle: output_handle,
            _pd: PhantomData,
        })
    }
}
impl TryFrom<P1<Output>> for P1<Input> {
    type Error = gpio_cdev::Error;

    fn try_from(value: P1<Output>) -> std::result::Result<Self, Self::Error> {
        drop(value.handle);
        let output_handle = value.line.request(LineRequestFlags::INPUT, 0, "P0_in")?;
        Ok(Self {
            line: value.line,
            handle: output_handle,
            _pd: PhantomData,
        })
    }
}
impl TryFrom<P2<Output>> for P2<Input> {
    type Error = gpio_cdev::Error;

    fn try_from(value: P2<Output>) -> std::result::Result<Self, Self::Error> {
        drop(value.handle);
        let output_handle = value.line.request(LineRequestFlags::INPUT, 0, "P0_in")?;
        Ok(Self {
            line: value.line,
            handle: output_handle,
            _pd: PhantomData,
        })
    }
}
impl TryFrom<P3<Output>> for P3<Input> {
    type Error = gpio_cdev::Error;

    fn try_from(value: P3<Output>) -> std::result::Result<Self, Self::Error> {
        drop(value.handle);
        let output_handle = value.line.request(LineRequestFlags::INPUT, 0, "P0_in")?;
        Ok(Self {
            line: value.line,
            handle: output_handle,
            _pd: PhantomData,
        })
    }
}
impl TryFrom<P4<Output>> for P4<Input> {
    type Error = gpio_cdev::Error;

    fn try_from(value: P4<Output>) -> std::result::Result<Self, Self::Error> {
        drop(value.handle);
        let output_handle = value.line.request(LineRequestFlags::INPUT, 0, "P0_in")?;
        Ok(Self {
            line: value.line,
            handle: output_handle,
            _pd: PhantomData,
        })
    }
}
impl TryFrom<P5<Output>> for P5<Input> {
    type Error = gpio_cdev::Error;

    fn try_from(value: P5<Output>) -> std::result::Result<Self, Self::Error> {
        drop(value.handle);
        let output_handle = value.line.request(LineRequestFlags::INPUT, 0, "P0_in")?;
        Ok(Self {
            line: value.line,
            handle: output_handle,
            _pd: PhantomData,
        })
    }
}
impl TryFrom<P6<Output>> for P6<Input> {
    type Error = gpio_cdev::Error;

    fn try_from(value: P6<Output>) -> std::result::Result<Self, Self::Error> {
        drop(value.handle);
        let output_handle = value.line.request(LineRequestFlags::INPUT, 0, "P0_in")?;
        Ok(Self {
            line: value.line,
            handle: output_handle,
            _pd: PhantomData,
        })
    }
}
impl TryFrom<P7<Output>> for P7<Input> {
    type Error = gpio_cdev::Error;

    fn try_from(value: P7<Output>) -> std::result::Result<Self, Self::Error> {
        drop(value.handle);
        let output_handle = value.line.request(LineRequestFlags::INPUT, 0, "P0_in")?;
        Ok(Self {
            line: value.line,
            handle: output_handle,
            _pd: PhantomData,
        })
    }
}

impl InputPin for IncomingHandshake {
    fn is_high(&self) -> Result<bool> {
        self.handle
            .get_value()
            .map(|c| c != 0)
            .map_err(|e| IoError::Other(Box::new(e)))
    }
}
impl InputPin for P0<Input> {
    fn is_high(&self) -> Result<bool> {
        self.handle
            .get_value()
            .map(|c| c != 0)
            .map_err(|e| IoError::Other(Box::new(e)))
    }
}
impl InputPin for P1<Input> {
    fn is_high(&self) -> Result<bool> {
        self.handle
            .get_value()
            .map(|c| c != 0)
            .map_err(|e| IoError::Other(Box::new(e)))
    }
}
impl InputPin for P2<Input> {
    fn is_high(&self) -> Result<bool> {
        self.handle
            .get_value()
            .map(|c| c != 0)
            .map_err(|e| IoError::Other(Box::new(e)))
    }
}
impl InputPin for P3<Input> {
    fn is_high(&self) -> Result<bool> {
        self.handle
            .get_value()
            .map(|c| c != 0)
            .map_err(|e| IoError::Other(Box::new(e)))
    }
}
impl InputPin for P4<Input> {
    fn is_high(&self) -> Result<bool> {
        self.handle
            .get_value()
            .map(|c| c != 0)
            .map_err(|e| IoError::Other(Box::new(e)))
    }
}
impl InputPin for P5<Input> {
    fn is_high(&self) -> Result<bool> {
        self.handle
            .get_value()
            .map(|c| c != 0)
            .map_err(|e| IoError::Other(Box::new(e)))
    }
}
impl InputPin for P6<Input> {
    fn is_high(&self) -> Result<bool> {
        self.handle
            .get_value()
            .map(|c| c != 0)
            .map_err(|e| IoError::Other(Box::new(e)))
    }
}
impl InputPin for P7<Input> {
    fn is_high(&self) -> Result<bool> {
        self.handle
            .get_value()
            .map(|c| c != 0)
            .map_err(|e| IoError::Other(Box::new(e)))
    }
}

impl OutputPin for OutgoingHandshake {
    fn set_high(&mut self) -> Result<()> {
        self.handle
            .set_value(1)
            .map_err(|e| IoError::Other(Box::new(e)))
    }

    fn set_low(&mut self) -> Result<()> {
        self.handle
            .set_value(0)
            .map_err(|e| IoError::Other(Box::new(e)))
    }
}
impl OutputPin for Reset {
    fn set_high(&mut self) -> Result<()> {
        self.handle
            .set_value(1)
            .map_err(|e| IoError::Other(Box::new(e)))
    }

    fn set_low(&mut self) -> Result<()> {
        self.handle
            .set_value(0)
            .map_err(|e| IoError::Other(Box::new(e)))
    }
}

impl OutputPin for P0<Output> {
    fn set_high(&mut self) -> Result<()> {
        self.handle
            .set_value(1)
            .map_err(|e| IoError::Other(Box::new(e)))
    }

    fn set_low(&mut self) -> Result<()> {
        self.handle
            .set_value(0)
            .map_err(|e| IoError::Other(Box::new(e)))
    }
}
impl OutputPin for P1<Output> {
    fn set_high(&mut self) -> Result<()> {
        self.handle
            .set_value(1)
            .map_err(|e| IoError::Other(Box::new(e)))
    }

    fn set_low(&mut self) -> Result<()> {
        self.handle
            .set_value(0)
            .map_err(|e| IoError::Other(Box::new(e)))
    }
}
impl OutputPin for P2<Output> {
    fn set_high(&mut self) -> Result<()> {
        self.handle
            .set_value(1)
            .map_err(|e| IoError::Other(Box::new(e)))
    }

    fn set_low(&mut self) -> Result<()> {
        self.handle
            .set_value(0)
            .map_err(|e| IoError::Other(Box::new(e)))
    }
}
impl OutputPin for P3<Output> {
    fn set_high(&mut self) -> Result<()> {
        self.handle
            .set_value(1)
            .map_err(|e| IoError::Other(Box::new(e)))
    }

    fn set_low(&mut self) -> Result<()> {
        self.handle
            .set_value(0)
            .map_err(|e| IoError::Other(Box::new(e)))
    }
}
impl OutputPin for P4<Output> {
    fn set_high(&mut self) -> Result<()> {
        self.handle
            .set_value(1)
            .map_err(|e| IoError::Other(Box::new(e)))
    }

    fn set_low(&mut self) -> Result<()> {
        self.handle
            .set_value(0)
            .map_err(|e| IoError::Other(Box::new(e)))
    }
}
impl OutputPin for P5<Output> {
    fn set_high(&mut self) -> Result<()> {
        self.handle
            .set_value(1)
            .map_err(|e| IoError::Other(Box::new(e)))
    }

    fn set_low(&mut self) -> Result<()> {
        self.handle
            .set_value(0)
            .map_err(|e| IoError::Other(Box::new(e)))
    }
}
impl OutputPin for P6<Output> {
    fn set_high(&mut self) -> Result<()> {
        self.handle
            .set_value(1)
            .map_err(|e| IoError::Other(Box::new(e)))
    }

    fn set_low(&mut self) -> Result<()> {
        self.handle
            .set_value(0)
            .map_err(|e| IoError::Other(Box::new(e)))
    }
}
impl OutputPin for P7<Output> {
    fn set_high(&mut self) -> Result<()> {
        self.handle
            .set_value(1)
            .map_err(|e| IoError::Other(Box::new(e)))
    }

    fn set_low(&mut self) -> Result<()> {
        self.handle
            .set_value(0)
            .map_err(|e| IoError::Other(Box::new(e)))
    }
}
