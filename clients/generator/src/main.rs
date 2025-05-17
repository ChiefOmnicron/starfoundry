mod config;
mod code;
mod service;

use oas3::OpenApiV3Spec;
use oas3::spec::PathItem;
use std::fs::remove_dir_all;
use std::path::Path;
use std::process::Command;
use tracing_subscriber::EnvFilter;

use crate::config::{config, ClientModule, ClientPath, Language};
use crate::service::{OperationType, Service, ServiceFunction};

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = config();

    // TODO: load definition from server
    let definition = std::fs::read_to_string("definition.json").unwrap();
    let open_api_spec: OpenApiV3Spec = oas3::from_json(definition).unwrap();

    let mut services: Vec<Service> = Vec::new();

    for (key, client) in config.clients {
        copy_template(key, &client.path).unwrap();

        for module in client.modules {
            let routes = collect_routes(&module, open_api_spec.clone());

            let mut service = Service::new(open_api_spec.clone(), client.path.clone(), module.tag.clone());

            for (path, item) in routes {
                if let Some(x) = item.get {
                    let func = ServiceFunction::new(open_api_spec.clone(), path, x, module.tag.clone(), OperationType::Get);
                    service.add_service(func);
                } else if let Some(x) = item.post {
                    let func = ServiceFunction::new(open_api_spec.clone(), path, x, module.tag.clone(), OperationType::Post);
                    service.add_service(func);
                } else if let Some(x) = item.put {
                    let func = ServiceFunction::new(open_api_spec.clone(), path, x, module.tag.clone(), OperationType::Put);
                    service.add_service(func);
                } else if let Some(x) = item.delete {
                    let func = ServiceFunction::new(open_api_spec.clone(), path, x, module.tag.clone(), OperationType::Delete);
                    service.add_service(func);
                }
            }

            services.push(service);
        }
    }

    for service in services {
        service.generate(&Language::TypeScript);
    }
}

fn collect_routes(
    module:        &ClientModule,
    open_api_spec: OpenApiV3Spec,
) -> Vec<(String, PathItem)> {
    open_api_spec
        .paths
        .clone()
        .unwrap_or_default()
        .into_iter()
        .filter(|(_, x)| {
            x
                .methods()
                .into_iter()
                .filter(|(_, y)| y.tags.contains(&module.tag))
                .count() > 0
        })
        .collect::<Vec<_>>()
}

fn copy_template(
    name: String,
    path: &ClientPath,
) -> Result<(), Box<dyn std::error::Error>> {
    if Path::new(&path.0).exists() {
        remove_dir_all(&path.0)?;
    }

    Command::new("cp")
        .arg("-r")
        .arg(format!("src/templates/{name}/"))
        .arg(format!("{}/", path.0))
        .output()?;

    Ok(())
}
