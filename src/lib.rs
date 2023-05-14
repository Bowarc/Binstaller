#[macro_use]
extern crate log;

pub mod error;
pub mod frame;

pub struct GraphicalInstaller<Data> {
    data: Option<Data>,
    frames: Vec<frame::GraphicalInstallerFrame>,
}

impl<Data> GraphicalInstaller<Data> {
    pub fn add_frame(&mut self, frame: frame::GraphicalInstallerFrame) -> Result<(), error::Error> {
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

    // pub fn register_data<T: ?Sized + std::any::Any + 'static>(
    //     &mut self,
    //     id: impl Into<String>,
    //     data: &'static T,
    // ) -> Result<(), error::Error> {
    //     let id = id.into();
    //     if self.data.get(&id).is_some() {
    //         error!("Spot is taken, return an error or replace data")
    //     }
    //     let typeid = std::any::TypeId::of::<T>();

    //     self.data.insert(id, (typeid, Box::new(data)));

    //     Ok(())
    // }

    // pub fn retreive_data<T>(&self, id: impl Into<String>) -> Option<&T> {
    //     let opt = self.data.get(&(id.into()));

    //     if let Some((typeid, value)) = opt {
    //         let v = value.try_into::<T>;
    //     }

    //     None
    // }
}

impl<T> Default for GraphicalInstaller<T> {
    fn default() -> Self {
        Self {
            data: None,
            frames: Vec::new(),
        }
    }
}
