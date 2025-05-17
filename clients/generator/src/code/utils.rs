use oas3::spec::{ObjectSchema, Operation};
use oas3::OpenApiV3Spec;

pub fn convert_operation_id_to_func_name(
    tag:          &str,
    operation_id: String,
) -> String {
    let operation_id = operation_id.replace(&tag, "");
    let mut operation_id: Vec<String> = operation_id
        .split("_")
        .filter(|x| !x.is_empty())
        .map(Into::into)
        .collect::<Vec<_>>();
    operation_id
        .iter_mut()
        .skip(1)
        .for_each(|x: &mut String| *x = uppercase_first_char(x.clone()));
    operation_id.join("")
}

pub fn uppercase_first_char(
    input: String,
) -> String {
    (input[0..1].to_uppercase() + &input[1..]).to_string()
}

pub fn request_body_model_name(
    spec:      OpenApiV3Spec,
    operation: Operation,
) -> Option<ObjectSchema> {
    let operation_id = operation.operation_id.unwrap();

    let body = if let Some(x) = operation.request_body {
        x
    } else {
        tracing::error!("{} operation has no request body, but it was requested", operation_id);
        return None;
    };

    let model = if let Ok(x) = body.resolve(&spec) {
        x
    } else {
        tracing::error!("{} operation has no valid model", operation_id);
        return None;
    };

    let model = if let Some(x) = model.content.get("application/json") {
        x
    } else {
        tracing::error!("{} operation has no valid model type application/json", operation_id);
        return None;
    };

    let schema = if let Ok(x) = model.schema(&spec) {
        x
    } else {
        tracing::error!("{} operation has no valid model schema", operation_id);
        return None;
    };

    Some(schema)
}

pub fn response_model_name_200(
    spec:      OpenApiV3Spec,
    operation: Operation,
) -> Option<ObjectSchema> {
    let operation_id = operation.operation_id.unwrap();

    let body = if let Some(x) = operation.responses {
        x
    } else {
        tracing::error!("{} operation has no response", operation_id);
        return None;
    };

    let body = if let Some(x) = body.get("200") {
        x
    } else {
        tracing::error!("{} operation has no status 200", operation_id);
        return None;
    };

    let model = if let Ok(x) = body.resolve(&spec) {
        x
    } else {
        tracing::error!("{} operation has no valid model", operation_id);
        return None;
    };

    let model = if let Some(x) = model.content.get("application/json") {
        x
    } else {
        tracing::error!("{} operation has no valid model type application/json", operation_id);
        return None;
    };

    let schema = if let Ok(x) = model.schema(&spec) {
        x
    } else {
        tracing::error!("{} operation has no valid model schema", operation_id);
        return None;
    };

    Some(schema)
}

pub fn response_model_name_201(
    spec:      OpenApiV3Spec,
    operation: Operation,
) -> Option<ObjectSchema> {
    let operation_id = operation.operation_id.unwrap();

    let body = if let Some(x) = operation.responses {
        x
    } else {
        tracing::error!("{} operation has no response", operation_id);
        return None;
    };

    let body = if let Some(x) = body.get("201") {
        x
    } else {
        tracing::error!("{} operation has no status 201", operation_id);
        return None;
    };

    let model = if let Ok(x) = body.resolve(&spec) {
        x
    } else {
        tracing::error!("{} operation has no valid model", operation_id);
        return None;
    };

    let model = if let Some(x) = model.content.get("application/json") {
        x
    } else {
        tracing::error!("{} operation has no valid model type application/json", operation_id);
        return None;
    };

    let schema = if let Ok(x) = model.schema(&spec) {
        x
    } else {
        tracing::error!("{} operation has no valid model schema", operation_id);
        return None;
    };

    Some(schema)
}
