use futures::future::join_all;
use prettytable::Table;
use std::future::Future;
use std::pin::Pin;
use thiserror::Error;

pub mod prelude;

#[derive(Error, Debug)]
pub enum ModuleError {
    #[error("Module failed to execute")]
    ExecutionError,
}

pub type ModuleResult = Result<Table, ModuleError>;
pub type AsyncModuleResult = Pin<Box<dyn Future<Output = ModuleResult> + Send>>;

pub enum ModuleKind {
    Sync(fn() -> ModuleResult),
    Async(fn() -> AsyncModuleResult),
}

pub struct Module {
    pub name: &'static str,
    pub kind: ModuleKind,
}

impl Module {
    pub async fn execute(&self) -> ModuleResult {
        match &self.kind {
            ModuleKind::Sync(func) => func(),
            ModuleKind::Async(func) => func().await,
        }
    }
}

inventory::collect!(Module);

pub async fn get(name: Vec<&str>) -> Vec<Option<&'static Module>> {
    inventory::iter::<Module>
        .into_iter()
        .filter(|m| name.contains(&m.name))
        .map(Some)
        .collect()
}

pub async fn get_names() -> Vec<String> {
    inventory::iter::<Module>
        .into_iter()
        .map(|m| m.name.to_string())
        .collect()
}

pub async fn get_all() -> Vec<&'static Module> {
    inventory::iter::<Module>.into_iter().collect()
}

pub async fn execute_all() -> Vec<ModuleResult> {
    let modules = get_all().await;
    let futures: Vec<_> = modules.into_iter().map(|m| m.execute()).collect();
    join_all(futures).await
}

pub async fn execute(name: Vec<&str>) -> Vec<ModuleResult> {
    let modules = get_all().await;
    let futures: Vec<_> = modules
        .into_iter()
        .filter(|m| name.contains(&m.name))
        .map(|m| m.execute())
        .collect();
    join_all(futures).await.into_iter().collect()
}
