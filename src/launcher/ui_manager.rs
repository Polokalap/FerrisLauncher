use std::rc::Rc;
use slint::{ComponentHandle, ModelRc, VecModel};

use crate::launcher::player_manager::set_player;
use crate::logger::info;
use crate::{Window, InstanceData};

pub type InstanceList = Rc<VecModel<InstanceData>>;

pub async fn init() -> Result<(), Box<dyn std::error::Error>> {

    info("Loading main window").await;

    let ui = Window::new()?;
    let ui_weak = ui.as_weak();

    tokio::spawn(async move {

        set_player(ui_weak, "Polokalap").await;

    });

    let instances: Rc<VecModel<InstanceData>> = Rc::new(VecModel::default());
    ui.set_instances(ModelRc::from(instances.clone()));

    add_instance(
        "Meow",
        "Quilt 1.21.11",
        "Box",
        instances.clone(),
    ).await?;
    add_instance(
        "Meow",
        "Quilt 1.21.11",
        "Box",
        instances.clone(),
    ).await?;
    add_instance(
        "Meow",
        "Quilt 1.21.11",
        "Box",
        instances.clone(),
    ).await?;
    add_instance(
        "Meow",
        "Quilt 1.21.11",
        "Box",
        instances.clone(),
    ).await?;
    add_instance(
        "Meow",
        "Quilt 1.21.11",
        "Box",
        instances.clone(),
    ).await?;

    info("Window loaded!").await;
    ui.run()?;

    Ok(())

}

pub async fn add_instance(name: &str, version: &str, icon: &str, instances: InstanceList) -> Result<(), Box<dyn std::error::Error>> {

    info(format!("Registering instance {}", name).as_str()).await;

    instances.push(InstanceData {
        name: name.into(),
        version: version.into(),
        icon: icon.to_lowercase().into(),
    });

    Ok(())

}