#[macro_use]
extern crate log;

pub mod error;
pub mod frame;

#[derive(Default)]
pub struct GraphicalInstaller<Data: Default + std::fmt::Debug> {
    data: Data,
    frames: Vec<frame::GraphicalInstallerFrame<Data>>,
}

impl<Data: Default + std::fmt::Debug> GraphicalInstaller<Data> {
    pub fn add_frame(
        &mut self,
        frame: frame::GraphicalInstallerFrame<Data>,
    ) -> Result<(), error::Error> {
        self.frames.push(frame);
        Ok(())
    }

    pub fn register_data(&mut self, data: impl Into<Data>) {
        self.data = data.into();
    }

    pub fn retreive_data(&mut self) -> &mut Data {
        // match &mut self.data {
        //     None => None,
        //     Some(data) => Some(data),
        // }
        &mut self.data
    }

    pub fn run(&mut self) -> Result<(), error::Error> {
        trace!(
            "Running with {num_frames} frames and with data: {data:?}",
            num_frames = self.frames.len(),
            data = self.data
        );
        // This function in meant to be ran in the main thread (eframes needs it)

        for frame in &mut self.frames {
            frame.run(&mut self.data).unwrap();
        }

        Ok(())
    }
}

// impl<T> Default for GraphicalInstaller<T> {
//     fn default() -> Self {
//         Self {
//             data: None,
//             frames: Vec::new(),
//         }
//     }
// }
