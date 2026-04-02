#[macro_export]
macro_rules! sort_by_job_flat {
    ($name:ident, $typ_in:ty) => {
        pub fn $name(entries: Vec<$typ_in>) -> Vec<$typ_in> {
            #[allow(dead_code)]
            struct Tmp {
                header:  String,
                entries: Vec<$typ_in>,
            }

            crate::sort_by_job!(inner_sort, $typ_in, Tmp);

            inner_sort(entries)
                .into_iter()
                .flat_map(|x| x.entries)
                .collect::<Vec<_>>()
        }
    }
}
