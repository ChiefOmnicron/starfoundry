use crate::config::Language;
use oas3::spec::Parameter;
use oas3::OpenApiV3Spec;

pub fn interface_function(
    spec:           &OpenApiV3Spec,
    lang:           &Language,
    interface_name: String,
    fields:         Vec<Parameter>,
) -> String {
    match lang {
        Language::TypeScript => typescript(
            spec,
            interface_name,
            fields,
        ),
    }
}

fn typescript(
    spec:           &OpenApiV3Spec,
    interface_name: String,
    fields:         Vec<Parameter>,
) -> String {
    let fields = fields
        .into_iter()
        .map(|x| {
            let required = if let Some(false) = x.required {
                "?"
            } else {
                ""
            };

            let typ = x.schema.unwrap();
            let typ = typ.resolve(spec).unwrap();
            if let Some(x) = typ.schema_type {
                dbg!(x);
            }

            format!(
                "\t{name}{required}",
                name = x.name,
                required = required,
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(r#"export interface I{interface_name} {{
    }}"#,
        interface_name = interface_name,
    )
}

