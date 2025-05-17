use crate::config::Language;

pub fn service_function(
    lang:         &Language,
    service_name: String,
    functions:    Vec<String>,
) -> String {
    match lang {
        Language::TypeScript => typescript(
            service_name,
            functions,
        ),
    }
}

fn typescript(
    service_name: String,
    functions:    Vec<String>,
) -> String {
    format!(r#"import axios from 'axios';

export class {service_name}Service {{
    {functions}
}}"#,
        service_name = service_name,
        functions = functions.join("\n"),
    )
}

