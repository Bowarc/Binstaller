#[macro_use]
extern crate log;

pub mod error;
pub mod frame;

#[derive(Default)]
pub struct GraphicalInstaller {
    data: Option<Box<dyn std::any::Any>>,
    frames: Vec<frame::GraphicalInstallerFrame>,
}

impl GraphicalInstaller {
    pub fn add_frame(&mut self, frame: frame::GraphicalInstallerFrame) -> Result<(), error::Error> {
        self.frames.push(frame);
        Ok(())
    }

    pub fn register_data(&mut self, data: impl std::any::Any) {
        self.data = Some(Box::new(data));
    }

    pub fn retreive_data<T: 'static>(&mut self) -> Option<T> {
        Some(*self.data.take()?.downcast::<T>().unwrap())
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
