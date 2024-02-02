use crate::{Module, ModuleResult};

use futures::future::join_all;

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

pub async fn get_all_sorted_by_category() -> Vec<&'static Module> {
    let mut modules: Vec<_> = inventory::iter::<Module>.into_iter().collect();
    modules.sort_by(|a, b| a.category.cmp(b.category).then_with(|| a.name.cmp(b.name)));
    modules
}

pub fn format_module_info(module: &Module) -> String {
    format!(
        "{} - {} (v{}) by {}: {}",
        module.name, module.description, module.version, module.author, module.category
    )
}
