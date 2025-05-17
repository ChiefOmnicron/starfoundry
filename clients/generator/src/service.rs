use oas3::spec::{Operation, Parameter, ParameterIn};
use crate::code::{convert_operation_id_to_func_name, get_function, interface_function, post_function, request_body_model_name, response_model_name_200, response_model_name_201, service_function, uppercase_first_char};
use crate::config::{ClientPath, Language};
use oas3::OpenApiV3Spec;

pub struct Service {
    spec:        OpenApiV3Spec,
    client_path: ClientPath,
    tag:         String,

    functions:   Vec<ServiceFunction>,
    models:      Vec<String>,
}

impl Service {
    pub fn new(
        spec:        OpenApiV3Spec,
        client_path: ClientPath,
        tag:         String,
    ) -> Self {
        Self {
            spec:        spec,
            client_path: client_path,
            tag:         tag,

            functions:   Vec::new(),
            models:      Vec::new(),
        }
    }

    pub fn add_service(
        &mut self,
        service: ServiceFunction,
    ) {
        self.functions.push(service);
    }

    pub fn add_model(
        &mut self,
        model: String,
    ) {
        self.models.push(model);
    }

    pub fn generate(
        self,
        lang: &Language,
    ) {
        let functions = self
            .functions
            .into_iter()
            .map(|x| ServiceFunction::generate(x, &lang))
            .collect::<Vec<_>>();

        let service_name = uppercase_first_char(self.tag);

        let service = service_function(
            lang,
            service_name,
            functions,
        );

        self.client_path.save("test.ts", service).unwrap();
    }
}

pub struct ServiceFunction {
    spec:           OpenApiV3Spec,
    path:           String,
    operation:      Operation,
    tag:            String,
    operation_type: OperationType,
}

impl ServiceFunction {
    pub fn new(
        spec:           OpenApiV3Spec,
        path:           String,
        operation:      Operation,
        tag:            String,
        operation_type: OperationType,
    ) -> Self {
        Self {
            spec,
            path,
            operation,
            tag,
            operation_type,
        }
    }

    pub fn method_name(&self) -> String {
        if let Some(x) = &self.operation.operation_id {
            convert_operation_id_to_func_name(&self.tag, x.clone())
        } else {
            tracing::error!("{} tried to call post, but has not post entry", self.path);
            panic!("invalid input");
        }
    }

    pub fn query_params(&self) -> Vec<Parameter> {
        self.operation
            .parameters
            .iter()
            .map(|x| x.resolve(&self.spec).unwrap())
            .filter(|x| x.location == ParameterIn::Query)
            .collect::<Vec<_>>()
    }

    pub fn request_model_name(&self) -> String {
        let body = if let Some(x) = request_body_model_name(
            self.spec.clone(),
            self.operation.clone()
        ) {
            x
        } else {
            tracing::error!("{} has no request_body", self.operation.operation_id.clone().unwrap());
            panic!("invalid input");
        };

        let name = if let Some(x) = body.title {
            x
        } else {
            tracing::error!("{} has no title in a model", self.operation.operation_id.clone().unwrap());
            panic!("invalid input");
        };

        format!("I{name}")
    }

    pub fn response_model_name_200(&self) -> String {
        let body = if let Some(x) = response_model_name_200(
            self.spec.clone(),
            self.operation.clone()
        ) {
            x
        } else {
            tracing::error!("{} has no request_body", self.operation.operation_id.clone().unwrap());
            panic!("invalid input");
        };

        let name = if let Some(x) = body.title {
            x
        } else {
            tracing::error!("{} has no title in a model", self.operation.operation_id.clone().unwrap());
            panic!("invalid input");
        };

        format!("I{name}")
    }

    pub fn response_model_name_201(&self) -> String {
        let body = if let Some(x) = response_model_name_201(
            self.spec.clone(),
            self.operation.clone()
        ) {
            x
        } else {
            tracing::error!("{} has no request_body", self.operation.operation_id.clone().unwrap());
            panic!("invalid input");
        };

        let name = if let Some(x) = body.title {
            x
        } else {
            tracing::error!("{} has no title in a model", self.operation.operation_id.clone().unwrap());
            panic!("invalid input");
        };

        format!("I{name}")
    }

    pub fn generate(
        self,
        lang: &Language,
    ) -> String {
        //let mut interfaces = Vec::new();

        match self.operation_type {
            OperationType::Get => {
                let mut params = Vec::new();
                if !self.query_params().is_empty() {
                    let interface_name = format!("I{}Filter", uppercase_first_char(self.tag.clone()));

                    params.push(format!("filter: {}", interface_name.clone()));
                    interface_function(&self.spec, lang, interface_name, self.query_params());
                }

                get_function(
                    lang,
                    self.path.clone(),
                    params,
                    self.method_name(),
                    self.response_model_name_200(),
                )
            },
            OperationType::Post => post_function(
                lang,
                self.path.clone(),
                self.method_name(),
                self.request_model_name(),
                self.response_model_name_201(),
            ),
            _ => panic!("Unsupported operation type {:?}", self.operation_type)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum OperationType {
    Get,
    Post,
    Put,
    Delete,
}
