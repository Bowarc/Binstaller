type Executor<Data> = dyn Fn(&mut eframe::egui::Ui, &mut Data);

pub struct GraphicalInstallerFrame<Data> {
    pub(crate) ui_executor: Box<Executor<Data>>,
}

impl<Data> GraphicalInstallerFrame<Data> {
    pub fn set_executor(
        &mut self,
        executor: &'static Executor<Data>,
    ) -> Result<(), crate::error::Error> {
        self.ui_executor = Box::new(executor);
        Ok(())
    }
}

impl<Data> Default for GraphicalInstallerFrame<Data> {
    fn default() -> Self {
        GraphicalInstallerFrame {
            ui_executor: Box::new(|_, _| {}),
        }
    }
}
