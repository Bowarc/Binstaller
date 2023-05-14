pub struct GraphicalInstallerFrame<Data> {
    pub(crate) ui_executor: Box<dyn Fn(Option<&mut Data>)>,
}

impl<Data> GraphicalInstallerFrame<Data> {
    pub fn set_executor(
        &mut self,
        executor: &'static dyn Fn(Option<&mut Data>),
    ) -> Result<(), crate::error::Error> {
        self.ui_executor = Box::new(executor);
        Ok(())
    }

    pub fn run(&mut self, data: Option<&mut Data>) -> Result<(), crate::error::Error> {
        // self.ui_executor(data);

        let fnc = &self.ui_executor;
        fnc(data);
        Ok(())
    }
}

impl<Data> Default for GraphicalInstallerFrame<Data> {
    fn default() -> Self {
        GraphicalInstallerFrame {
            ui_executor: Box::new(|_| {}),
        }
    }
}
