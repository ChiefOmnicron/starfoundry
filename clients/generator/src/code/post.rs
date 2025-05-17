use crate::config::Language;

pub fn post_function(
    lang:           &Language,
    path:           String,
    method_name:    String,
    request_model:  String,
    response_model: String,
) -> String {
    match lang {
        Language::TypeScript => typescript(
            path,
            method_name,
            request_model,
            response_model,
        ),
    }
}

fn typescript(
    path:           String,
    method_name:    String,
    request_model:  String,
    response_model: String,
) -> String {
    format!(r#"public static async {method_name} (
        data: {request_model},
        cancel: AbortSignal,
    ): Promise<{response_model}> {{
        return axios
            .post<{response_model}>(
                `{path}`,
                data,
                {{
                    headers: {{
                        'Content-Type': 'application/json',
                    }},
                    signal: cancel,
                }},
            )
            .then((x) => x.data);
    }}"#,
        path =           path,
        method_name =    method_name,
        request_model =  request_model,
        response_model = response_model,
    )
}

