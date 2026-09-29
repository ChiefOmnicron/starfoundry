pub fn group_id_to_job_type(
    group_id: i32,
) -> String {
    match group_id {
        428     => "INTERMEDIATE_REACTIONS",
        429     => "COMPOSITE_REACTIONS",
        4096    => "MOLECULAR_FORGED_REACTIONS",
        974     => "HYBRID_REACTIONS",
        334     => "CONSTRUCTION_COMPONENTS",
        913     => "ADVANCED_CAPITAL_CONSTRUCTION_COMPONENTS",
        873     => "CAPITAL_CONSTRUCTION_COMPONENTS",
        536     => "STRUCTURE_COMPONENTS",
        332     => "TOOLS",
        1       => "T1_MODULE",
        2       => "T2_MODULE",
        1308    => "RIGS",
        66      => "STRUCTURE_RIGS",
        8       => "CHARGES",
        6       => "SHIPS",
        65      => "STRUCTURE",
        22      => "DEPLOYABLE",
        _       => "UNKNOWN",
    }.into()
}
