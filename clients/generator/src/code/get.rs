use crate::config::Language;

pub fn get_function(
    lang:           &Language,
    path:           String,
    params:         Vec<String>,
    method_name:    String,
    response_model: String,
) -> String {
    match lang {
        Language::TypeScript => typescript(
            path,
            params,
            method_name,
            response_model,
        ),
    }
}

fn typescript(
    path:           String,
    params:         Vec<String>,
    method_name:    String,
    response_model: String,
) -> String {
    format!(r#"public static async {method_name} (
        {params}cancel: AbortSignal,
    ): Promise<{response_model}> {{
        return axios
            .get<{response_model}>(
                `{path}`
            )
            .then((x) => x.data);
    }}"#,
        path =           path,
        params =         params.join("\n,"),
        method_name =    method_name,
        response_model = response_model,
    )
}

