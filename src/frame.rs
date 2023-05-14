type UiExecutorFunction = dyn Fn();

pub struct GraphicalInstallerFrame {
    ui_executor: Box<UiExecutorFunction>,
}

impl GraphicalInstallerFrame {
    pub fn set_executor(
        &mut self,
        executor: &'static UiExecutorFunction,
    ) -> Result<(), crate::error::Error> {
        self.ui_executor = Box::new(executor);
        Ok(())
    }
}

impl Default for GraphicalInstallerFrame {
    fn default() -> Self {
        GraphicalInstallerFrame {
            ui_executor: Box::new(|| {}),
        }
    }
}
