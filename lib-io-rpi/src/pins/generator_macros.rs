macro_rules! input_pin {
    ($pin:ident) => {
        pub struct $pin {
            pub line: gpio_cdev::Line,
            pub handle: gpio_cdev::LineHandle,
        }

        impl $pin {
            pub fn new(chip: &mut gpio_cdev::Chip, number: u32) -> lib_io::Result<$pin> {
                let line = chip
                    .get_line(number)
                    .map_err(|e| lib_io::IoError::Other(Box::new(e)))?;
                let handle = line
                    .request(Input::flag(), 0, stringify!($pin))
                    .map_err(|e| lib_io::IoError::Other(Box::new(e)))?;
                Ok($pin { line, handle })
            }
        }

        impl InputPin for $pin {
            fn is_high(&self) -> lib_io::Result<bool> {
                self.handle
                    .get_value()
                    .map(|c| c != 0)
                    .map_err(|e| lib_io::IoError::Other(Box::new(e)))
            }
        }
    };
}

macro_rules! switchable_pin {
    ($pin:ident) => {
        pub struct $pin<A: Selector> {
            pub line: gpio_cdev::Line,
            pub handle: gpio_cdev::LineHandle,
            pub _pd: std::marker::PhantomData<A>,
        }

        impl<A: Selector> $pin<A> {
            pub fn new(chip: &mut gpio_cdev::Chip, number: u32) -> lib_io::Result<$pin<A>> {
                let line = chip
                    .get_line(number)
                    .map_err(|e| lib_io::IoError::Other(Box::new(e)))?;
                let handle = line
                    .request(A::flag(), 0, stringify!($pin))
                    .map_err(|e| lib_io::IoError::Other(Box::new(e)))?;
                Ok($pin {
                    line,
                    handle,
                    _pd: std::marker::PhantomData,
                })
            }
        }

        impl std::convert::TryFrom<$pin<Input>> for $pin<Output> {
            type Error = gpio_cdev::Error;

            fn try_from(value: $pin<Input>) -> std::result::Result<Self, Self::Error> {
                drop(value.handle);
                let output_handle = value.line.request(Output::flag(), 0, stringify!($pin))?;
                Ok(Self {
                    line: value.line,
                    handle: output_handle,
                    _pd: std::marker::PhantomData,
                })
            }
        }

        impl std::convert::TryFrom<$pin<Output>> for $pin<Input> {
            type Error = gpio_cdev::Error;

            fn try_from(value: $pin<Output>) -> std::result::Result<Self, Self::Error> {
                drop(value.handle);
                let output_handle = value.line.request(Input::flag(), 0, stringify!($pin))?;
                Ok(Self {
                    line: value.line,
                    handle: output_handle,
                    _pd: std::marker::PhantomData,
                })
            }
        }

        impl InputPin for $pin<Input> {
            fn is_high(&self) -> lib_io::Result<bool> {
                self.handle
                    .get_value()
                    .map(|c| c != 0)
                    .map_err(|e| lib_io::IoError::Other(Box::new(e)))
            }
        }

        impl OutputPin for $pin<Output> {
            fn set_high(&mut self) -> lib_io::Result<()> {
                self.handle
                    .set_value(1)
                    .map_err(|e| lib_io::IoError::Other(Box::new(e)))
            }

            fn set_low(&mut self) -> lib_io::Result<()> {
                self.handle
                    .set_value(0)
                    .map_err(|e| lib_io::IoError::Other(Box::new(e)))
            }
        }
    };
}

macro_rules! output_pin {
    ($pin:ident) => {
        pub struct $pin {
            pub line: gpio_cdev::Line,
            pub handle: gpio_cdev::LineHandle,
        }

        impl $pin {
            pub fn new(chip: &mut gpio_cdev::Chip, number: u32) -> lib_io::Result<$pin> {
                let line = chip
                    .get_line(number)
                    .map_err(|e| lib_io::IoError::Other(Box::new(e)))?;
                let handle = line
                    .request(Output::flag(), 0, stringify!($pin))
                    .map_err(|e| lib_io::IoError::Other(Box::new(e)))?;
                Ok($pin { line, handle })
            }
        }

        impl OutputPin for $pin {
            fn set_high(&mut self) -> lib_io::Result<()> {
                self.handle
                    .set_value(1)
                    .map_err(|e| lib_io::IoError::Other(Box::new(e)))
            }

            fn set_low(&mut self) -> lib_io::Result<()> {
                self.handle
                    .set_value(0)
                    .map_err(|e| lib_io::IoError::Other(Box::new(e)))
            }
        }
    };
}
