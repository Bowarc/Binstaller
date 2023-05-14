#[macro_use]
extern crate log;

pub mod error;
pub mod frame;

pub struct GraphicalInstaller<Data> {
    data: Option<Data>,
    frames: Vec<frame::GraphicalInstallerFrame<Data>>,
}

impl<Data> GraphicalInstaller<Data> {
    pub fn add_frame(
        &mut self,
        frame: frame::GraphicalInstallerFrame<Data>,
    ) -> Result<(), error::Error> {
        self.frames.push(frame);
        Ok(())
    }

    pub fn register_data(&mut self, data: impl Into<Data>) {
        self.data = Some(data.into());
    }

    pub fn retreive_data(&mut self) -> Option<&mut Data> {
        // match &mut self.data {
        //     None => None,
        //     Some(data) => Some(data),
        // }
        self.data.as_mut()
    }

    pub fn run(&mut self) -> Result<(), error::Error> {
        // This function in meant to be ran in the main thread (eframes needs it)

        for frame in &mut self.frames {
            frame.run(self.data.as_mut()).unwrap();
        }

        Ok(())
    }
}

impl<T> Default for GraphicalInstaller<T> {
    fn default() -> Self {
        Self {
            data: None,
            frames: Vec::new(),
        }
    }
}
