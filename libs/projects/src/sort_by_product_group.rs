#[macro_export]
macro_rules! sort_by_product_group {
    ($name:ident, $typ_in:ty, $typ_out:ident) => {
        pub fn $name(entries: HashMap<String, Vec<$typ_in>>) -> Vec<$typ_out> {
            let mut grouped = Vec::new();

            for (project_name, assignments) in entries.into_iter() {
                grouped.push(
                    $typ_out {
                        header:  project_name.clone(),
                        entries: assignments,
                    }
                );
            }

            grouped.sort_by_key(|x| x.header.clone());

            grouped
        }
    }
}
