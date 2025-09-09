use serde::Serialize;
use starfoundry_lib_structures::{BonusVariations, StructureType};
use starfoundry_lib_types::TypeId;
use std::collections::{VecDeque, HashMap};
use std::fs::File;

use super::{Bonus, Dependency, DependencyBuildCost, DependencyTreeEntry, EngineResult, ProjectConfig};
use crate::{BlueprintTyp, StockMinimal};

/// Group of dependencies.
/// 
#[derive(Debug, Default, Serialize)]
pub struct CalculationEngine {
    tree:   HashMap<TypeId, DependencyTreeEntry>,

    config: ProjectConfig,
    stocks: HashMap<TypeId, StockMinimal>,
}

impl CalculationEngine {
    /// Creates a new Dependency tree, without any dependencies.
    /// Use [CalculationEngine::add] to add dependencies.
    /// 
    pub fn new(
        config: ProjectConfig,
    ) -> Self {
        Self {
            config: config,
            tree:   HashMap::new(),
            stocks: HashMap::new(),
        }
    }

    /// Adds the given [Dependency] to the tree.
    /// 
    /// # Params
    /// 
    /// * `dependency` > Dependency to add to the tree
    /// 
    pub fn add(
        &mut self,
        dependency: Dependency,
    ) -> &mut Self {
        self.add_to_tree(dependency.clone(), true);
        let mut queue: VecDeque<Dependency> = if self.config.skip_children {
            VecDeque::new()
        } else {
            dependency.components.into()
        };

        while let Some(dep) = queue.pop_front() {
            self.add_to_tree(dep.clone(), false);

            // Skip if the ptype_id is in the ignore list
            if self.config.blacklist.contains(&dep.ptype_id) {
                continue;
            }

            for component in dep.components.into_iter() {
                let mut component = component;
                if component.typ == BlueprintTyp::Material {
                    component.needed = 0f32;
                }

                queue.push_back(component.clone());
            }
        }

        // Set all blacklisted items to be materials and set their required
        // amount to 0
        // The correct required number will be determined in a later step
        for ptype_id in self.config.blacklist.iter() {
            if let Some(x) = self.tree.get_mut(&ptype_id) {
                x.needed = 0f32;
                x.typ    = BlueprintTyp::Material;
            }
        }

        self.full_calculation();
        self
    }

    pub fn add_stocks(
        &mut self,
        stocks: &Vec<StockMinimal>,
    ) -> &mut Self {
        for stock in stocks {
            let mut has_update = false;
            self
                .tree
                .entry(stock.type_id)
                .and_modify(|x: &mut DependencyTreeEntry| {
                    has_update = true;
                    x.stock += stock.quantity;

                    if x.is_product {
                        x.needed = (x.needed as u32).saturating_sub(stock.quantity as u32) as f32;

                        let mut quantity = stock.quantity as u32;
                        for run in x.runs.iter_mut() {
                            if quantity == 0 {
                                break;
                            }

                            let runs = *run;
                            *run = run.saturating_sub(quantity);
                            quantity = quantity.saturating_sub(runs);
                        }

                        self.stocks.insert(x.product_type_id, stock.clone());
                    }
                });

            if has_update {
                self.partial_calculation(stock.type_id);
            }
        }

        self
    }

    pub fn stocks(&self) -> Vec<StockMinimal> {
        self.stocks
            .clone()
            .into_iter()
            .map(|(_, stock)| stock)
            .filter(|x| x.quantity > 0)
            .collect::<Vec<_>>()
    }

    /// Recalculates the whole tree
    /// 
    pub fn full_calculation(
        &mut self,
    ) {
        let queue = self.tree
            .keys()
            .cloned()
            .collect::<VecDeque<_>>();
        self.calculate_runs(queue);
    }

    /// Only recalculates everything that was changed by updating the given
    /// [TypeId]
    /// 
    pub fn partial_calculation(
        &mut self,
        ptype_id: TypeId,
    ) {
        self.calculate_runs(vec![ptype_id].into())
    }

    /// Gets a list of all product [TypeId]s that are required for the current
    /// tree
    /// 
    pub fn product_type_ids(
        &mut self,
    ) -> Vec<TypeId> {
        self.tree
            .iter()
            .filter(|(_, x)| x.typ == BlueprintTyp::Blueprint)
            .map(|(_, x)| x.product_type_id)
            .collect::<Vec<_>>()
    }

    /// Applies blueprint and structure bonuses to the tree.
    /// Unless overriden, it is assumed that every blueprint has a ME of 10
    /// 
    pub fn apply_bonus(
        &mut self,
    ) -> &mut Self {
        for ptype_id in self.product_type_ids().iter() {
            let me_bonus = match self.config.blueprint_overwrite.get(ptype_id) {
                Some(x) => x.material,
                None => 10f32,
            };
            let te_bonus = match self.config.blueprint_overwrite.get(ptype_id) {
                Some(x) => x.time,
                None    => 20f32
            };

            self.apply_me_bonus(
                *ptype_id,
                me_bonus,
            );
            self.apply_te_bonus(
                *ptype_id,
                te_bonus,
            );
            self.partial_calculation(*ptype_id);
        }

        self.apply_by_bonus_type(BlueprintTyp::Blueprint);
        self.apply_by_bonus_type(BlueprintTyp::Reaction);

        self
    }

    pub fn finalize(
        &mut self,
    ) -> EngineResult {
        for ptype_id in self.tree
                            .iter()
                            .filter(|(_, x)|
                                x.typ == BlueprintTyp::Blueprint ||
                                x.typ == BlueprintTyp::Reaction
                            )
                            .map(|(_, x)| x.product_type_id)
                            .collect::<Vec<_>>() {

            if self.config.is_blacklisted(ptype_id.0) {
                continue;
            }

            if let Some(x) = self.tree.get_mut(&ptype_id) {
                let total_runs: u32 = x.runs.iter().map(|x| *x).sum();
                if x.is_product && total_runs as f32 == x.needed {
                    continue;
                }

                // total number of runs required
                let runs = std::cmp::max(
                    (x.needed as f32 / x.produces as f32).ceil() as u32,
                    1u32
                );

                let max_bp_runs = if let Some(x) = self.config.max_runs.get(&x.product_type_id) {
                    *x
                } else {
                    u32::MAX
                };

                if x.needed == 0f32 {
                    continue;
                } else if x.is_product {
                    x.runs = vec![runs];
                } else if runs == 1 {
                    x.runs.push(runs);
                } else if (runs as f32 * x.time) < self.config.max_time as f32 && runs <= max_bp_runs {
                    x.runs.push(runs);
                } else {
                    // note:
                    // if there was a run limit set, it will use a run based splitting strategy
                    // per default it will go by the max time that were set and split by it

                    // run based splitting, activated when the number of runs is limited
                    if max_bp_runs < u32::MAX {
                        let mut needed_total = (x.needed as f32 / x.produces as f32).ceil() as u32;

                        while needed_total > 0 {
                            if needed_total < max_bp_runs {
                                x.runs.push(needed_total);
                                needed_total -= needed_total;
                            } else {
                                x.runs.push(max_bp_runs);
                                needed_total -= max_bp_runs;
                            }
                        }
                    } else {
                        // time based splitting, default method used
                        let splits = std::cmp::max(
                            ((runs as f32 * x.time) / self.config.max_time as f32).ceil() as u32,
                            1u32
                        );

                        let needed_total = (x.needed as f32 / x.produces as f32).ceil();
                        let max = (needed_total / splits as f32).floor() as u32;
                        let rest = (needed_total % (max as f32 * splits as f32)) as u32;

                        if max > 0 {
                            x.runs = vec![max as u32; splits as usize];
                        }

                        if rest > 0 {
                            x.runs.push(rest as u32);
                        }

                        // balance the number of runs
                        let sum: u32 = x.runs.iter().sum();
                        let overhead = (sum as f32 % splits as f32) as u32;

                        let runs = (sum as f32 / splits as f32).floor() as u32;
                        x.runs = vec![runs; splits as usize];
                        if overhead > 0 {
                            for (index, _) in vec![0; overhead as usize].iter().enumerate() {
                                x.runs[index] += 1;
                            }
                        }
                    }
                }
            } else {
                continue;
            }
        }

        self.calculate_materials();
        self.calculate_cost();
        self.cleanup_stock();

        EngineResult {
            tree:   std::mem::take(&mut self.tree),
            stocks: self.stocks(),
        }
    }

    /// Writes the current state of the tree to disk
    /// 
    #[allow(unused)]
    #[cfg(debug_assertions)]
    pub fn write_debug_file(&mut self) -> &mut Self {
        self.write_debug_file_named("DependencyTreeDebug.json");
        self
    }

    #[allow(unused)]
    #[cfg(debug_assertions)]
    pub fn write_debug_file_named(
        &mut self,
        name: &str,
    ) -> &mut Self {
        let mut file = File::create(name).unwrap();
        serde_json::to_writer_pretty(&mut file, &self).unwrap();
        self
    }

    /// Adds the given [Dependency] to the tree.
    /// 
    /// # Params
    /// 
    /// * `dependency` > Dependency to add to the tree
    /// 
    fn add_to_tree(
        &mut self,
        dependency: Dependency,
        is_product: bool,
    ) {
        // Collect all children and the require amount
        let children = dependency
            .components
            .into_iter()
            .map(|x| (x.ptype_id, x.needed))
            .collect::<HashMap<_, _>>();

        let runs = if is_product {
            vec![dependency.needed as u32]
        } else {
            Vec::new()
        };

        // Either insert or edit the ptype_id entry
        self.tree
            .entry(dependency.ptype_id.clone())
            .and_modify(|x: &mut DependencyTreeEntry| {
                x.needed += dependency.needed;

                if is_product {
                    x.runs.push(dependency.needed as u32);
                }
            })
            .or_insert(DependencyTreeEntry {
                blueprint_type_id:  dependency.btype_id,
                product_type_id:    dependency.ptype_id,
                needed:             dependency.needed,
                time:               dependency.time,
                produces:           dependency.produces,
                children_unbonused: children.clone(),
                children:           children,
                typ:                dependency.typ,
                item:               dependency.item,
                runs:               runs,
                stock:              0,
                is_product:         is_product,
                bonus:              Vec::new(),
                build_cost:         DependencyBuildCost::default(),
                structure:          None,
            });
    }

    /// Calculates how many runs are required for [BlueprintTyp::Blueprint] and
    /// [BlueprintTyp::Reaction]
    /// 
    /// Values are updated in place
    /// 
    /// # Params
    /// 
    /// * `queue` > Only updates entries that have the given ids as dependency
    ///             Allows for partial calculations
    /// 
    fn calculate_runs(
        &mut self,
        queue: VecDeque<TypeId>,
    ) {
        let mut queue = queue.clone();

        while let Some(ptype_id) = queue.pop_front() {
            // Add all children to the queue
            if let Some(x) = self.tree.get(&ptype_id) {
                let entries = x.children.keys().collect::<Vec<_>>();
                queue.extend(entries);
            }

            let mut updated_runs = std::collections::HashMap::new();

            // Get all items that have the ptype_id as a children, calculate the
            // number of runs and collect them together
            self.tree
                .iter()
                .filter(|(_, e)|
                    e.typ == BlueprintTyp::Blueprint ||
                    e.typ == BlueprintTyp::Reaction
                )
                .filter(|(_, e)| e.children.contains_key(&ptype_id))
                .for_each(|(p, e)| {
                    if e.is_product {
                        let per_run = e.children.get(&ptype_id).unwrap_or(&0f32);
                        let total_runs = e.runs
                            .iter()
                            .map(|x| (*x as f32 * per_run).ceil())
                            .sum();

                        updated_runs
                            .entry(p.clone())
                            .and_modify(|x: &mut f32| *x += total_runs)
                            .or_insert(total_runs);
                    } else if !self.config.is_blacklisted(*p) {
                        let runs = (e.needed / e.produces as f32).ceil();
                        let per_run = e.children.get(&ptype_id).unwrap_or(&0f32);
                        let per_run = runs as f32 * per_run;

                        updated_runs
                            .entry(p.clone())
                            .and_modify(|x: &mut f32| *x += per_run)
                            .or_insert(per_run);
                    }
                });

            // the product will not be in any children
            if updated_runs.is_empty() {
                continue;
            }

            if let Some(x) = self.tree.get_mut(&ptype_id) {
                if x.typ != BlueprintTyp::Material &&
                   !self.config.is_blacklisted(*x.product_type_id) {

                    if !x.is_product {
                        x.needed = updated_runs
                            .into_iter()
                            .map(|(_, e)| e.ceil())
                            .sum();
                    }

                    let old_needed = x.needed;
                    x.needed -= x.stock as f32;

                    if x.needed <= 0f32 {
                        self.stocks
                            .entry(x.product_type_id)
                            .and_modify(|x| {
                                if old_needed.ceil() as i32 > x.quantity {
                                    x.quantity = old_needed.ceil() as i32;
                                }
                            })
                            .or_insert(StockMinimal {
                                type_id:  x.product_type_id,
                                quantity: old_needed.ceil() as i32,
                            });

                        let before = x.needed;
                        x.needed = 0f32;
                        x.runs = Vec::new();
                        x.bonus.push(Bonus {
                            before: before,
                            after: x.needed,
                            percent: 0f32,
                            typ: "misc".into(),
                            additional: "calculate_runs".into()
                        });
                    } else {
                        self.stocks
                            .entry(x.product_type_id)
                            .and_modify(|y| {
                                if x.stock as i32 > y.quantity {
                                    y.quantity = x.stock as i32;
                                }
                            })
                            .or_insert(StockMinimal {
                                type_id:  x.product_type_id,
                                quantity: x.stock as i32,
                            });
                    }
                }
            }
        }
    }

    fn calculate_materials(
        &mut self,
    ) {
        let entries = self.tree
            .clone()
            .into_iter()
            .filter(|(_, x)|
                x.typ == BlueprintTyp::Blueprint ||
                x.typ == BlueprintTyp::Reaction
            )
            .map(|(_, x)| x)
            .collect::<Vec<_>>();

        // First collect all required materials together in the map
        // This will be our total required materials over all jobs
        let mut materials = HashMap::new();
        for entry in entries {
            for (child_ptype, child_val) in entry.children.iter() {
                for run in entry.runs.iter() {
                    materials
                        .entry(child_ptype.clone())
                        .and_modify(|x: &mut f32| *x += (child_val * *run as f32).ceil())
                        .or_insert((child_val * *run as f32).ceil());
                }
            }
        }

        // Apply the number of needed materials
        for (ptype_id, value) in materials {
            if let Some(x) = self.tree.get_mut(&ptype_id) {
                x.needed = value;

                let old_needed = x.needed;
                x.needed -= x.stock as f32;

                if x.needed <= 0f32 {
                    self.stocks
                        .entry(x.product_type_id)
                        .and_modify(|x| {
                            if old_needed.ceil() as i32 > x.quantity {
                                x.quantity = old_needed.ceil() as i32;
                            }
                        })
                        .or_insert(StockMinimal {
                            type_id:  x.product_type_id,
                            quantity: old_needed.ceil() as i32,
                        });

                    let before = x.needed;
                    x.needed = 0f32;
                    x.runs = Vec::new();

                    x.bonus.push(Bonus {
                        before: before,
                        after: x.needed,
                        percent: 0f32,
                        typ: "misc".into(),
                        additional: "calculate_materials".into()
                    });
                } else {
                    self.stocks
                        .entry(x.product_type_id)
                        .and_modify(|y| {
                            if x.stock as i32 > y.quantity {
                                y.quantity = x.stock as i32;
                            }
                        })
                        .or_insert(StockMinimal {
                            type_id:  x.product_type_id,
                            quantity: x.stock as i32,
                        });
                }
            }
        }
    }

    fn calculate_cost(
        &mut self,
    ) {
        let entries = self.tree
            .clone()
            .into_iter()
            .filter(|(_, x)|
                x.typ == BlueprintTyp::Blueprint ||
                x.typ == BlueprintTyp::Reaction
            )
            .collect::<Vec<_>>();

        for (_, entry) in entries {
            let mut material_adjusted_price = HashMap::new();

            let structure = if let Some(x) = &entry.structure {
                x
            } else {
                continue;
            };

            let total_runs: u32 = entry.runs.iter().sum();
            let materials_cost_total = entry
                .children_unbonused
                .iter()
                .map(|(type_id, quantity)| {
                    let adjusted_price = self.config
                        .material_cost
                        .get(type_id)
                        .unwrap_or(&0f64) * *quantity as f64 * total_runs as f64;

                    material_adjusted_price.insert(*type_id, adjusted_price as f32);
                    adjusted_price as f32
                })
                .sum::<f32>()
                .ceil();
            let system_cost: &(f32, f32) = self.config
                .system_index
                .get(&structure.system_id)
                .unwrap_or(&(1f32, 1f32));

            let system_cost = if structure.structure_type == StructureType::Raitaru ||
                structure.structure_type == StructureType::Azbel ||
                structure.structure_type == StructureType::Sotiyo {
                100f32 * (materials_cost_total * (system_cost.0 / 100f32))
            } else if structure.structure_type == StructureType::Athanor ||
                structure.structure_type == StructureType::Tatara {
                100f32 * (materials_cost_total * (system_cost.1  / 100f32))
            } else {
                f32::INFINITY
            };

            let facility_bonus: f32 = if structure.structure_type == StructureType::Sotiyo {
                system_cost * 0.05
            } else if structure.structure_type == StructureType::Azbel {
                system_cost * 0.04
            } else if structure.structure_type == StructureType::Azbel {
                system_cost * 0.03
            } else {
                0f32
            };
            let total_job_gross = system_cost - facility_bonus;

            // TODO: add option to adjust facility costs
            let facility = materials_cost_total * 0.01;
            let scc = materials_cost_total * 0.04;

            let total_job_cost = (total_job_gross + facility + scc).ceil();

            self.tree
                .get_mut(&entry.product_type_id)
                .map(|x: &mut DependencyTreeEntry| {
                    x.build_cost.material_adjusted_price = material_adjusted_price;
                    x.build_cost.total_job_gross = total_job_gross;
                    x.build_cost.material_cost_total = materials_cost_total;
                    x.build_cost.facility = facility;
                    x.build_cost.scc = scc;
                    x.build_cost.total_job_cost = total_job_cost;
                });
        }
    }

    fn cleanup_stock(
        &mut self,
    ) {
        for (type_id, _) in self.stocks.clone().iter() {
            if let Some(x) = self.tree.get(type_id) {
                if x.is_product {
                    continue;
                }
            }

            if self
                .tree
                .iter()
                .filter(|(_, parent)| parent.children.contains_key(type_id))
                .find(|(_, parent)| parent.needed > 0f32)
                .is_none() {

                self.stocks.remove(type_id);
            }
        }
    }

    fn apply_by_bonus_type(
        &mut self,
        bonus_type: BlueprintTyp,
    ) -> &mut Self {
        let tree_clone = self.tree.clone();
        let structures_clone = self.config.structures.clone();

        let blueprints = tree_clone
            .iter()
            .filter(|(_, e)| e.typ == bonus_type)
            .collect::<Vec<_>>();

        for (_, blueprint) in blueprints {
            // Find all structures that have bonuses for the blueprint
            let structure_ids = self.config.structure_mappings
                .iter()
                .filter(|x|
                    x.category_group.contains(&blueprint.item.category_id) ||
                    x.category_group.contains(&blueprint.item.group_id)
                )
                .map(|x| x.structure_uuid)
                .collect::<Vec<_>>();

            // Grab the actual structures
            let structures = if structure_ids.len() > 0 {
                structures_clone
                    .iter()
                    .filter(|x| structure_ids.contains(&x.id))
                    .collect::<Vec<_>>()
            } else {
                continue;
            };

            // Find the structure with the best ME bonus and the lowest system bonus
            let mut selected_structure = None;
            let mut total_me = 0f32;
            let mut selected_system_cost = f32::INFINITY;
            for structure in structures {
                /*if blueprint.typ == BlueprintTyp::Reaction {
                    // TODO: more precise decision
                    if !(structure.services.contains(&45539.into()) ||
                        structure.services.contains(&45537.into()) ||
                        structure.services.contains(&45538.into()))
                     {
                        continue;
                    }
                } else if blueprint.typ == BlueprintTyp::Blueprint {
                    // TODO: do not skip here, make an option that structures can only
                    // be used for capital or super capital stuffs
                    if blueprint.item.group_id == 485.into() ||
                        blueprint.item.group_id == 547.into() ||
                        blueprint.item.group_id == 1538.into() ||
                        blueprint.item.group_id == 659.into() ||
                        blueprint.item.group_id == 30.into() {

                        // ignore
                    } else if !structure.services.contains(&35878.into()) {
                        continue;
                    }
                }*/

                if blueprint.item.group_id == 485.into() ||
                    blueprint.item.group_id == 547.into() ||
                    blueprint.item.group_id == 1538.into() {
                    if !structure.services.contains(&35881.into()) {
                        continue;
                    }
                }
                if blueprint.item.group_id == 659.into() ||
                    blueprint.item.group_id == 30.into() {
                    if !structure.services.contains(&35877.into()) {
                        continue;
                    }
                }

                let rig_me = if let Some(x) = structure
                    .rigs()
                    .iter()
                    .find(|x| {
                        x.has_category_or_group(*blueprint.item.category_id) ||
                        x.has_category_or_group(*blueprint.item.group_id)
                    })
                    .map(|x| x.material) {

                    x.unwrap_or_default()
                } else {
                    0f32
                };

                let structure_me = if let Some(BonusVariations::Material(me)) = structure
                    .structure_type
                    .bonus()
                    .iter()
                    .find(|x| {
                        match x {
                            BonusVariations::Material(_) => true,
                            _                            => false
                        }
                    }) {

                    *me
                } else {
                    0f32
                };

                let (manufacturing, reaction): &(f32, f32) = self.config
                    .system_index
                    .get(&structure.system_id)
                    .unwrap_or(&(1f32, 1f32));

                let system_cost = if structure.structure_type == StructureType::Raitaru ||
                    structure.structure_type == StructureType::Azbel ||
                    structure.structure_type == StructureType::Sotiyo {
                    *manufacturing
                } else if  structure.structure_type == StructureType::Athanor ||
                    structure.structure_type == StructureType::Tatara {
                    *reaction
                } else {
                    f32::INFINITY
                };

                if rig_me + structure_me > total_me &&
                    system_cost <= selected_system_cost {

                    selected_structure = Some(structure);
                    total_me = rig_me + structure_me;
                    selected_system_cost = system_cost;
                }
            }

            let structure = if let Some(x) = selected_structure {
                self.tree
                    .get_mut(&blueprint.product_type_id)
                    .map(|y| y.structure = Some(x.clone()));
                x
            } else {
                continue;
            };

            let rig = structure
                .rigs()
                .iter()
                .find(|x|
                    x.has_category_or_group(*blueprint.item.category_id) ||
                    x.has_category_or_group(*blueprint.item.group_id)
                )
                .map(|x| (x.material, x.time));

            if let Some((me, te)) = rig {
                if let Some(me) = me {
                    self.apply_me_bonus(
                        blueprint.product_type_id,
                        me,
                    );
                }
                if let Some(te) = te {
                    self.apply_te_bonus(
                        blueprint.product_type_id,
                        te,
                    );
                }
                self.partial_calculation(blueprint.product_type_id);
            }

            if let Some(BonusVariations::Material(me)) = structure
                .structure_type
                .bonus()
                .iter()
                .find(|x| {
                    match x {
                        BonusVariations::Material(_) => true,
                        _                            => false
                    }
                }) {
                    self.apply_me_bonus(
                        blueprint.product_type_id,
                        *me,
                    );
                self.partial_calculation(blueprint.product_type_id);
            }

            if let Some(BonusVariations::Time(te)) = structure
                .structure_type
                .bonus()
                .iter()
                .find(|x| {
                    match x {
                        BonusVariations::Time(_) => true,
                        _                        => false
                    }
                }) {
                    self.apply_te_bonus(
                        blueprint.product_type_id,
                        *te,
                    );
                self.partial_calculation(blueprint.product_type_id);
            }
        }
        self
    }

    /// Applies a material bonus to the given ptype_id
    /// 
    /// # Params
    /// 
    /// * `ptype_id` > [TypeId] of the product that gets a bonus
    /// * `bonus`    > Bonus in percent that should be applied
    /// 
    fn apply_me_bonus(
        &mut self,
        ptype_id: TypeId,
        bonus:    f32,
    ) {
        let entry = if let Some(x) = self.tree.get_mut(&ptype_id) {
            x
        } else {
            return;
        };

        for (tid, base_val) in entry.children.iter_mut() {
            // Don´t apply bonuses if only one item is required
            if *base_val == 1f32 {
                continue;
            }

            let mut new_bonus = Bonus::default();
            new_bonus.before = *base_val;
            new_bonus.percent = bonus;
            new_bonus.additional = tid.to_string();
            new_bonus.typ = String::from("material");

            let modifier = *base_val * (bonus as f32 / 100f32);
            *base_val = *base_val - modifier;

            new_bonus.after = *base_val;
            entry.bonus.push(new_bonus);
        }
    }

    /// Applies a time bonus to the given ptype_id
    /// 
    /// # Params
    /// 
    /// * `ptype_id` > [TypeId] of the product that gets a bonus
    /// * `bonus`    > Bonus in percent that should be applied
    /// 
    fn apply_te_bonus(
        &mut self,
        ptype_id: TypeId,
        bonus:    f32,
    ) {
        let entry = if let Some(x) = self.tree.get_mut(&ptype_id) {
            x
        } else {
            return;
        };

        if entry.typ == BlueprintTyp::Material {
            return;
        }

        let mut new_bonus = Bonus::default();
        new_bonus.before = entry.time;
        new_bonus.percent = bonus;
        new_bonus.typ = String::from("time");

        let modifier = entry.time * (bonus as f32 / 100f32);
        entry.time = entry.time - modifier;

        new_bonus.after = entry.time;
        entry.bonus.push(new_bonus);
    }
}

#[cfg(test)]
mod calculation_tests {
    use sqlx::PgPool;
    use sqlx::postgres::PgPoolOptions;
    use starfoundry_lib_structures::{Security, Structure};
    use uuid::Uuid;
    use std::str::FromStr;

    use crate::{ProjectConfigBuilder, StructureMapping};

    use super::*;

    async fn pool() -> PgPool {
        dotenvy::dotenv().ok();
        let pg_addr = std::env::var("DATABASE_URL").unwrap();
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&pg_addr)
            .await
            .unwrap();
        pool
    }

    async fn load_dependency(
        quanttiy: u32,
        type_id: TypeId
    ) -> Dependency {
        let pool = pool().await;
        let dependency = sqlx::query!("
                SELECT data
                FROM blueprint_json
                WHERE ptype_id = $1
            ",
                *type_id
            )
            .fetch_one(&pool)
            .await
            .map(|x| x.data)
            .unwrap();
        Dependency::try_from(quanttiy, dependency).unwrap()
    }

    async fn calculation_engine() -> CalculationEngine {
        let pool = pool().await;

        let manufacturing_a = Structure {
            id: Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap().into(),
            name: "Sotiyo manufacturing".into(),
            system_id: 30002019.into(),
            security: Security::Nullsec,
            structure_type: StructureType::Sotiyo,
            structure_id: 1337i64,
            services: vec![
                TypeId(35878),
                TypeId(35881),
            ],
            rigs: vec![
                starfoundry_lib_structures::rig::fetch(&pool, TypeId::from(37180)).await.unwrap(),
                starfoundry_lib_structures::rig::fetch(&pool, TypeId::from(37178)).await.unwrap(),
                starfoundry_lib_structures::rig::fetch(&pool, TypeId::from(43704)).await.unwrap(),
            ],
        };

        let reaction_a = Structure {
            id: Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap().into(),
            name: "Tatara reactions".into(),
            system_id: 30002019.into(),
            security: Security::Nullsec,
            structure_type: StructureType::Tatara,
            structure_id: 1338i64,
            services: vec![
                TypeId(35899), // Reprocessing
                TypeId(45539), // Biochemical Reactor
                TypeId(45537), // Composite Reactor
                TypeId(45538), // Hybrid Reactor
            ],
            rigs: vec![
                starfoundry_lib_structures::rig::fetch(&pool, TypeId::from(46497)).await.unwrap(),
            ],
        };

        let mapping = vec![
            StructureMapping {
                structure_uuid: manufacturing_a.id,
                category_group: manufacturing_a.category_groups(),
            },
            StructureMapping {
                structure_uuid: reaction_a.id,
                category_group: reaction_a.category_groups(),
            },
        ];

        let system_index = sqlx::query!("
                    SELECT
                        system_id,
                        manufacturing,
                        reaction
                    FROM industry_index
                    WHERE timestamp = (
                        SELECT timestamp
                        FROM industry_index
                        WHERE system_id = ANY($1)
                        GROUP BY system_id, timestamp
                        ORDER BY timestamp DESC
                        LIMIT 1
                    )
                    AND system_id = ANY($1)
                ",
                &vec![30003950]
            )
            .fetch_all(&pool)
            .await
            .unwrap()
            .into_iter()
            .map(|x| (x.system_id.into(), (x.manufacturing, x.reaction)))
            .collect::<HashMap<_, _>>();

        let material_cost = sqlx::query!("
                    SELECT
                        type_id,
                        adjusted_price
                    FROM market_price
                ",
            )
            .fetch_all(&pool)
            .await
            .unwrap()
            .into_iter()
            .map(|x| (x.type_id.into(), x.adjusted_price))
            .collect::<HashMap<_, _>>();

        let config = ProjectConfigBuilder::default()
            .add_structures(vec![manufacturing_a, reaction_a])
            .add_structure_mappings(mapping)
            .add_blacklists(vec![4051, 4246, 4247, 4312])
            .set_system_index(system_index)
            .set_material_cost(material_cost)
            .build();
        CalculationEngine::new(config)
    }

    #[tokio::test]
    async fn caracal1() {
        let caracal = load_dependency(1, TypeId(621)).await;
        let calculation_result = calculation_engine()
            .await
            .add(caracal)
            .apply_bonus()
            .finalize();

        assert_eq!(calculation_result.tree.get(&34.into()).unwrap().needed.ceil(), 460933f32);
        assert_eq!(calculation_result.tree.get(&35.into()).unwrap().needed.ceil(), 153645f32);
        assert_eq!(calculation_result.tree.get(&36.into()).unwrap().needed.ceil(),  30729f32);
        assert_eq!(calculation_result.tree.get(&37.into()).unwrap().needed.ceil(),   8536f32);
        assert_eq!(calculation_result.tree.get(&38.into()).unwrap().needed.ceil(),   1281f32);
        assert_eq!(calculation_result.tree.get(&39.into()).unwrap().needed.ceil(),    299f32);
        assert_eq!(calculation_result.tree.get(&40.into()).unwrap().needed.ceil(),    120f32);
    }

    #[tokio::test]
    async fn caracal5() {
        let caracal = load_dependency(5, TypeId(621)).await;
        let calculation_result = calculation_engine()
            .await
            .add(caracal)
            .apply_bonus()
            .finalize();

        assert_eq!(calculation_result.tree.get(&34.into()).unwrap().needed.ceil(), 2304661f32);
        assert_eq!(calculation_result.tree.get(&35.into()).unwrap().needed.ceil(),  768221f32);
        assert_eq!(calculation_result.tree.get(&36.into()).unwrap().needed.ceil(),  153645f32);
        assert_eq!(calculation_result.tree.get(&37.into()).unwrap().needed.ceil(),   42679f32);
        assert_eq!(calculation_result.tree.get(&38.into()).unwrap().needed.ceil(),    6402f32);
        assert_eq!(calculation_result.tree.get(&39.into()).unwrap().needed.ceil(),    1494f32);
        assert_eq!(calculation_result.tree.get(&40.into()).unwrap().needed.ceil(),     598f32);

        assert_eq!(calculation_result.tree.get(&621.into()).unwrap().runs, vec![5]);
    }

    #[tokio::test]
    async fn caracal_warden() {
        let caracal = load_dependency(1, TypeId(621)).await;
        let warden  = load_dependency(10, TypeId(23559)).await;

        let calculation_result = calculation_engine()
            .await
            .add(caracal)
            .add(warden)
            .apply_bonus()
            .finalize();

        assert_eq!(calculation_result.tree.get(&34.into()).unwrap().needed.ceil(), 465338f32);
        assert_eq!(calculation_result.tree.get(&35.into()).unwrap().needed.ceil(), 238397f32);
        assert_eq!(calculation_result.tree.get(&36.into()).unwrap().needed.ceil(),  30729f32);
        assert_eq!(calculation_result.tree.get(&37.into()).unwrap().needed.ceil(),   8536f32);
        assert_eq!(calculation_result.tree.get(&38.into()).unwrap().needed.ceil(),   1316f32);
        assert_eq!(calculation_result.tree.get(&39.into()).unwrap().needed.ceil(),    299f32);
        assert_eq!(calculation_result.tree.get(&40.into()).unwrap().needed.ceil(),    274f32);
    }

    #[tokio::test]
    async fn product_is_in_stock_single() {
        // Caracal
        let stocks = vec![
            StockMinimal {
                type_id:  TypeId(621),
                quantity: 1,
            }
        ];

        let caracal = load_dependency(1, TypeId(621)).await;
        let calculation_result = calculation_engine()
            .await
            .add(caracal)
            .add_stocks(&stocks)
            .apply_bonus()
            .finalize();

        for (_, entry) in calculation_result.tree.iter() {
            assert_eq!(entry.needed, 0f32);
        }

        assert_eq!(calculation_result.stocks.len(), 1);
        assert_eq!(calculation_result.stocks[0].type_id, TypeId(621));
        assert_eq!(calculation_result.tree.get(&621.into()).unwrap().runs, vec![0]);
    }

    #[tokio::test]
    async fn product_is_in_stock_multiple1() {
        // Caracal
        let stocks = vec![
            StockMinimal {
                type_id:  TypeId(621),
                quantity: 1,
            }
        ];

        let caracal = load_dependency(2, TypeId(621)).await;
        let calculation_result = calculation_engine()
            .await
            .add(caracal)
            .add_stocks(&stocks)
            .apply_bonus()
            .finalize();

        assert_eq!(calculation_result.tree.get(&34.into()).unwrap().needed.ceil(), 460933f32);
        assert_eq!(calculation_result.tree.get(&35.into()).unwrap().needed.ceil(), 153645f32);
        assert_eq!(calculation_result.tree.get(&36.into()).unwrap().needed.ceil(),  30729f32);
        assert_eq!(calculation_result.tree.get(&37.into()).unwrap().needed.ceil(),   8536f32);
        assert_eq!(calculation_result.tree.get(&38.into()).unwrap().needed.ceil(),   1281f32);
        assert_eq!(calculation_result.tree.get(&39.into()).unwrap().needed.ceil(),    299f32);
        assert_eq!(calculation_result.tree.get(&40.into()).unwrap().needed.ceil(),    120f32);

        assert_eq!(calculation_result.stocks.len(), 1);
        assert_eq!(calculation_result.stocks[0].type_id, TypeId(621));
        assert_eq!(calculation_result.tree.get(&621.into()).unwrap().runs, vec![1]);
    }

    #[tokio::test]
    async fn product_is_in_stock_multiple2() {
        // Caracal
        let stocks = vec![
            StockMinimal {
                type_id:  TypeId(621),
                quantity: 1,
            }
        ];

        let caracal = load_dependency(1, TypeId(621)).await;
        let calculation_result = calculation_engine()
            .await
            .add(caracal.clone())
            .add(caracal.clone())
            .add(caracal)
            .add_stocks(&stocks)
            .apply_bonus()
            .finalize();

        assert_eq!(calculation_result.tree.get(&34.into()).unwrap().needed.ceil(), 921866f32);
        assert_eq!(calculation_result.tree.get(&35.into()).unwrap().needed.ceil(), 307290f32);
        assert_eq!(calculation_result.tree.get(&36.into()).unwrap().needed.ceil(),  61458f32);
        assert_eq!(calculation_result.tree.get(&37.into()).unwrap().needed.ceil(),  17072f32);
        assert_eq!(calculation_result.tree.get(&38.into()).unwrap().needed.ceil(),   2562f32);
        assert_eq!(calculation_result.tree.get(&39.into()).unwrap().needed.ceil(),    598f32);
        assert_eq!(calculation_result.tree.get(&40.into()).unwrap().needed.ceil(),    240f32);

        assert_eq!(calculation_result.stocks.len(), 1);
        assert_eq!(calculation_result.stocks[0].type_id, TypeId(621));
        assert_eq!(calculation_result.tree.get(&621.into()).unwrap().runs, vec![0, 1, 1]);
    }

    // Regression test - Adding T1 and T2 of the same product, the T1 is ignored
    #[tokio::test]
    async fn t1_and_t2_product() {
        let steel_plates_t1 = load_dependency(1, TypeId(11279)).await;
        let steel_plates_t2 = load_dependency(1, TypeId(20353)).await;
        let calculation_result = calculation_engine()
            .await
            .add(steel_plates_t1)
            .add(steel_plates_t2)
            .apply_bonus()
            .finalize();

        assert_eq!(calculation_result.tree.get(&11279.into()).unwrap().runs, vec![2]);
        assert_eq!(calculation_result.tree.get(&20353.into()).unwrap().runs, vec![1]);

        assert_eq!(calculation_result.tree.get(&34.into()).unwrap().needed.ceil(), 32806f32);
        assert_eq!(calculation_result.tree.get(&35.into()).unwrap().needed.ceil(), 31129f32);
        assert_eq!(calculation_result.tree.get(&36.into()).unwrap().needed.ceil(), 23440f32);
        assert_eq!(calculation_result.tree.get(&37.into()).unwrap().needed.ceil(),  1568f32);
        assert_eq!(calculation_result.tree.get(&38.into()).unwrap().needed.ceil(),    31f32);
        assert_eq!(calculation_result.tree.get(&39.into()).unwrap().needed.ceil(),     4f32);
        assert_eq!(calculation_result.tree.get(&40.into()).unwrap().needed.ceil(),    21f32);
    }

    // Regression test - Tests that splitting something into multiple stacks
    //                   does not merge them into a single stack
    #[tokio::test]
    async fn split_runs() {
        let nanite = load_dependency(20, TypeId(28668)).await;
        let calculation_result = calculation_engine()
            .await
            .add(nanite.clone())
            .add(nanite)
            .apply_bonus()
            .finalize();

        assert_eq!(calculation_result.tree.get(&28668.into()).unwrap().runs, vec![20, 20]);

        assert_eq!(calculation_result.tree.get(&17392.into()).unwrap().needed.ceil(),  40f32);
        assert_eq!(calculation_result.tree.get(&2348.into()).unwrap().needed.ceil(),   40f32);
        assert_eq!(calculation_result.tree.get(&2463.into()).unwrap().needed.ceil(),  138f32);
    }

    #[tokio::test]
    async fn naglfar_1() {
        let testfolder = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let file = std::fs::File::open(format!("{}/testdata/naglfar.json", testfolder)).unwrap();
        let parsed: Dependency = serde_json::from_reader(file).unwrap();

        let mut tree = calculation_engine().await;
        tree.add(parsed);

        let calculation_result = tree
            .apply_bonus()
            .finalize();
        calculation_result.write_debug_file();

        // Booster Gas Clouds
        assert_eq!(calculation_result.tree.get(&25278.into()).unwrap().needed.ceil(),       20f32);
        assert_eq!(calculation_result.tree.get(&25279.into()).unwrap().needed.ceil(),       20f32);
        assert_eq!(calculation_result.tree.get(&28694.into()).unwrap().needed.ceil(),      156f32);
        assert_eq!(calculation_result.tree.get(&28695.into()).unwrap().needed.ceil(),     1091f32);
        assert_eq!(calculation_result.tree.get(&28696.into()).unwrap().needed.ceil(),      156f32);
        assert_eq!(calculation_result.tree.get(&28697.into()).unwrap().needed.ceil(),      156f32);
        assert_eq!(calculation_result.tree.get(&28698.into()).unwrap().needed.ceil(),      156f32);
        assert_eq!(calculation_result.tree.get(&28699.into()).unwrap().needed.ceil(),      156f32);
        assert_eq!(calculation_result.tree.get(&28700.into()).unwrap().needed.ceil(),     1091f32);
        assert_eq!(calculation_result.tree.get(&28701.into()).unwrap().needed.ceil(),      156f32);

        // Fullerenes
        assert_eq!(calculation_result.tree.get(&30370.into()).unwrap().needed.ceil(),    11685f32);
        assert_eq!(calculation_result.tree.get(&30371.into()).unwrap().needed.ceil(),    11879f32);
        assert_eq!(calculation_result.tree.get(&30372.into()).unwrap().needed.ceil(),    12172f32);
        assert_eq!(calculation_result.tree.get(&30373.into()).unwrap().needed.ceil(),    11198f32);
        assert_eq!(calculation_result.tree.get(&30374.into()).unwrap().needed.ceil(),    11685f32);
        assert_eq!(calculation_result.tree.get(&30375.into()).unwrap().needed.ceil(),    11976f32);
        assert_eq!(calculation_result.tree.get(&30376.into()).unwrap().needed.ceil(),     1656f32);
        assert_eq!(calculation_result.tree.get(&30377.into()).unwrap().needed.ceil(),      585f32);
        assert_eq!(calculation_result.tree.get(&30378.into()).unwrap().needed.ceil(),      585f32);

        // Raw Moon Materials
        assert_eq!(calculation_result.tree.get(&16633.into()).unwrap().needed.ceil(),    66403f32);
        assert_eq!(calculation_result.tree.get(&16634.into()).unwrap().needed.ceil(),    66305f32);
        assert_eq!(calculation_result.tree.get(&16635.into()).unwrap().needed.ceil(),    21715f32);
        assert_eq!(calculation_result.tree.get(&16636.into()).unwrap().needed.ceil(),    21813f32);
        assert_eq!(calculation_result.tree.get(&16639.into()).unwrap().needed.ceil(),      293f32);
        assert_eq!(calculation_result.tree.get(&16642.into()).unwrap().needed.ceil(),      391f32);
        assert_eq!(calculation_result.tree.get(&16643.into()).unwrap().needed.ceil(),      196f32);
        assert_eq!(calculation_result.tree.get(&16644.into()).unwrap().needed.ceil(),       98f32);
        assert_eq!(calculation_result.tree.get(&16646.into()).unwrap().needed.ceil(),      391f32);
        assert_eq!(calculation_result.tree.get(&16647.into()).unwrap().needed.ceil(),       98f32);
        assert_eq!(calculation_result.tree.get(&16648.into()).unwrap().needed.ceil(),       98f32);
        assert_eq!(calculation_result.tree.get(&16649.into()).unwrap().needed.ceil(),       98f32);
        assert_eq!(calculation_result.tree.get(&16650.into()).unwrap().needed.ceil(),       98f32);
        assert_eq!(calculation_result.tree.get(&16651.into()).unwrap().needed.ceil(),      391f32);
        assert_eq!(calculation_result.tree.get(&16652.into()).unwrap().needed.ceil(),       98f32);
        assert_eq!(calculation_result.tree.get(&16653.into()).unwrap().needed.ceil(),       98f32);

        // Minerals
        assert_eq!(calculation_result.tree.get(&34.into()).unwrap().needed.ceil(),     3568889f32);
        assert_eq!(calculation_result.tree.get(&35.into()).unwrap().needed.ceil(),    10569714f32);
        assert_eq!(calculation_result.tree.get(&36.into()).unwrap().needed.ceil(),     2976841f32);
        assert_eq!(calculation_result.tree.get(&37.into()).unwrap().needed.ceil(),      816905f32);
        assert_eq!(calculation_result.tree.get(&38.into()).unwrap().needed.ceil(),       88037f32);
        assert_eq!(calculation_result.tree.get(&39.into()).unwrap().needed.ceil(),       41749f32);
        assert_eq!(calculation_result.tree.get(&40.into()).unwrap().needed.ceil(),       20884f32);
        assert_eq!(calculation_result.tree.get(&11399.into()).unwrap().needed.ceil(),     1281f32);

        // Fuel Blocks
        assert_eq!(calculation_result.tree.get(&4051.into()).unwrap().needed.ceil(),       667f32);
        assert_eq!(calculation_result.tree.get(&4246.into()).unwrap().needed.ceil(),      1014f32);
        assert_eq!(calculation_result.tree.get(&4247.into()).unwrap().needed.ceil(),       642f32);
        assert_eq!(calculation_result.tree.get(&4312.into()).unwrap().needed.ceil(),       822f32);

        // PI
        assert_eq!(calculation_result.tree.get(&2312.into()).unwrap().needed.ceil(),       390f32);
        assert_eq!(calculation_result.tree.get(&2319.into()).unwrap().needed.ceil(),       817f32);
        assert_eq!(calculation_result.tree.get(&2401.into()).unwrap().needed.ceil(),      3671f32);
        assert_eq!(calculation_result.tree.get(&2463.into()).unwrap().needed.ceil(),       817f32);
        assert_eq!(calculation_result.tree.get(&2867.into()).unwrap().needed.ceil(),         6f32);
        assert_eq!(calculation_result.tree.get(&2868.into()).unwrap().needed.ceil(),        27f32);
        assert_eq!(calculation_result.tree.get(&2870.into()).unwrap().needed.ceil(),        13f32);
        assert_eq!(calculation_result.tree.get(&2871.into()).unwrap().needed.ceil(),         8f32);
        assert_eq!(calculation_result.tree.get(&2872.into()).unwrap().needed.ceil(),        33f32);
        assert_eq!(calculation_result.tree.get(&2876.into()).unwrap().needed.ceil(),        40f32);
        assert_eq!(calculation_result.tree.get(&3645.into()).unwrap().needed.ceil(),      3671f32);
        assert_eq!(calculation_result.tree.get(&3775.into()).unwrap().needed.ceil(),       390f32);

        // Commodities
        assert_eq!(calculation_result.tree.get(&57443.into()).unwrap().needed.ceil(),        1f32);
        assert_eq!(calculation_result.tree.get(&57445.into()).unwrap().needed.ceil(),        4f32);
        assert_eq!(calculation_result.tree.get(&57446.into()).unwrap().needed.ceil(),        4f32);
        assert_eq!(calculation_result.tree.get(&57447.into()).unwrap().needed.ceil(),        4f32);
        assert_eq!(calculation_result.tree.get(&57448.into()).unwrap().needed.ceil(),       30f32);
        assert_eq!(calculation_result.tree.get(&57450.into()).unwrap().needed.ceil(),        1f32);
        assert_eq!(calculation_result.tree.get(&57452.into()).unwrap().needed.ceil(),       66f32);
    }

    /// Test that includes stock of a item with sub dependencies
    #[tokio::test]
    async fn naglfar_with_stock_1() {
        let testfolder = std::env::var("CARGO_MANIFEST_DIR").unwrap();

        let file = std::fs::File::open(format!("{}/testdata/naglfar.json", testfolder)).unwrap();
        let parsed: Dependency = serde_json::from_reader(file).unwrap();

        // Neurolink Protection Cell
        let stocks = vec![
            StockMinimal {
                type_id:  TypeId(57488),
                quantity: 1,
            }
        ];

        let calculation_result = calculation_engine()
            .await
            .add(parsed)
            .add_stocks(&stocks)
            .apply_bonus()
            .finalize();

        calculation_result.write_debug_file();

        // Materials that are no longer needed because of the stock
        assert_eq!(calculation_result.tree.get(&2329.into()).unwrap().needed.ceil(),         0f32);
        assert_eq!(calculation_result.tree.get(&2346.into()).unwrap().needed.ceil(),         0f32);
        assert_eq!(calculation_result.tree.get(&2348.into()).unwrap().needed.ceil(),         0f32);
        assert_eq!(calculation_result.tree.get(&2361.into()).unwrap().needed.ceil(),         0f32);
        assert_eq!(calculation_result.tree.get(&9842.into()).unwrap().needed.ceil(),         0f32);
        assert_eq!(calculation_result.tree.get(&11399.into()).unwrap().needed.ceil(),        0f32);
        assert_eq!(calculation_result.tree.get(&28694.into()).unwrap().needed.ceil(),        0f32);
        assert_eq!(calculation_result.tree.get(&28696.into()).unwrap().needed.ceil(),        0f32);
        assert_eq!(calculation_result.tree.get(&28697.into()).unwrap().needed.ceil(),        0f32);
        assert_eq!(calculation_result.tree.get(&28698.into()).unwrap().needed.ceil(),        0f32);
        assert_eq!(calculation_result.tree.get(&28699.into()).unwrap().needed.ceil(),        0f32);
        assert_eq!(calculation_result.tree.get(&28701.into()).unwrap().needed.ceil(),        0f32);
        assert_eq!(calculation_result.tree.get(&57443.into()).unwrap().needed.ceil(),        0f32);
        assert_eq!(calculation_result.tree.get(&57445.into()).unwrap().needed.ceil(),        0f32);
        assert_eq!(calculation_result.tree.get(&57446.into()).unwrap().needed.ceil(),        0f32);
        assert_eq!(calculation_result.tree.get(&57447.into()).unwrap().needed.ceil(),        0f32);
        assert_eq!(calculation_result.tree.get(&57450.into()).unwrap().needed.ceil(),        0f32);
        assert_eq!(calculation_result.tree.get(&57452.into()).unwrap().needed.ceil(),        0f32);

        // Booster Gas Clouds
        assert_eq!(calculation_result.tree.get(&25278.into()).unwrap().needed.ceil(),       20f32);
        assert_eq!(calculation_result.tree.get(&25279.into()).unwrap().needed.ceil(),       20f32);
        assert_eq!(calculation_result.tree.get(&28695.into()).unwrap().needed.ceil(),      935f32);
        assert_eq!(calculation_result.tree.get(&28700.into()).unwrap().needed.ceil(),      935f32);

        // Fullerenes
        assert_eq!(calculation_result.tree.get(&30370.into()).unwrap().needed.ceil(),      975f32);
        assert_eq!(calculation_result.tree.get(&30371.into()).unwrap().needed.ceil(),     1169f32);
        assert_eq!(calculation_result.tree.get(&30372.into()).unwrap().needed.ceil(),     1462f32);
        assert_eq!(calculation_result.tree.get(&30373.into()).unwrap().needed.ceil(),      488f32);
        assert_eq!(calculation_result.tree.get(&30374.into()).unwrap().needed.ceil(),      975f32);
        assert_eq!(calculation_result.tree.get(&30375.into()).unwrap().needed.ceil(),     1266f32);
        assert_eq!(calculation_result.tree.get(&30376.into()).unwrap().needed.ceil(),     1656f32);
        assert_eq!(calculation_result.tree.get(&30377.into()).unwrap().needed.ceil(),      585f32);
        assert_eq!(calculation_result.tree.get(&30378.into()).unwrap().needed.ceil(),      585f32);

        // Raw Moon Materials
        assert_eq!(calculation_result.tree.get(&16633.into()).unwrap().needed.ceil(),    66112f32);
        assert_eq!(calculation_result.tree.get(&16634.into()).unwrap().needed.ceil(),    66014f32);
        assert_eq!(calculation_result.tree.get(&16635.into()).unwrap().needed.ceil(),    21424f32);
        assert_eq!(calculation_result.tree.get(&16636.into()).unwrap().needed.ceil(),    21522f32);
        assert_eq!(calculation_result.tree.get(&16639.into()).unwrap().needed.ceil(),      293f32);
        assert_eq!(calculation_result.tree.get(&16642.into()).unwrap().needed.ceil(),      391f32);
        assert_eq!(calculation_result.tree.get(&16643.into()).unwrap().needed.ceil(),      196f32);
        assert_eq!(calculation_result.tree.get(&16644.into()).unwrap().needed.ceil(),       98f32);
        assert_eq!(calculation_result.tree.get(&16646.into()).unwrap().needed.ceil(),      391f32);
        assert_eq!(calculation_result.tree.get(&16647.into()).unwrap().needed.ceil(),       98f32);
        assert_eq!(calculation_result.tree.get(&16648.into()).unwrap().needed.ceil(),       98f32);
        assert_eq!(calculation_result.tree.get(&16649.into()).unwrap().needed.ceil(),       98f32);
        assert_eq!(calculation_result.tree.get(&16650.into()).unwrap().needed.ceil(),       98f32);
        assert_eq!(calculation_result.tree.get(&16651.into()).unwrap().needed.ceil(),      391f32);
        assert_eq!(calculation_result.tree.get(&16652.into()).unwrap().needed.ceil(),       98f32);
        assert_eq!(calculation_result.tree.get(&16653.into()).unwrap().needed.ceil(),       98f32);

        // Minerals
        assert_eq!(calculation_result.tree.get(&34.into()).unwrap().needed.ceil(),     2926313f32);
        assert_eq!(calculation_result.tree.get(&35.into()).unwrap().needed.ceil(),    10569714f32);
        assert_eq!(calculation_result.tree.get(&36.into()).unwrap().needed.ceil(),     2976841f32);
        assert_eq!(calculation_result.tree.get(&37.into()).unwrap().needed.ceil(),      816905f32);
        assert_eq!(calculation_result.tree.get(&38.into()).unwrap().needed.ceil(),       88037f32);
        assert_eq!(calculation_result.tree.get(&39.into()).unwrap().needed.ceil(),       41749f32);
        assert_eq!(calculation_result.tree.get(&40.into()).unwrap().needed.ceil(),       20884f32);

        // Fuel Blocks
        assert_eq!(calculation_result.tree.get(&4051.into()).unwrap().needed.ceil(),       539f32);
        assert_eq!(calculation_result.tree.get(&4246.into()).unwrap().needed.ceil(),       866f32);
        assert_eq!(calculation_result.tree.get(&4247.into()).unwrap().needed.ceil(),       514f32);
        assert_eq!(calculation_result.tree.get(&4312.into()).unwrap().needed.ceil(),       787f32);

        // PI
        assert_eq!(calculation_result.tree.get(&2312.into()).unwrap().needed.ceil(),       390f32);
        assert_eq!(calculation_result.tree.get(&2319.into()).unwrap().needed.ceil(),       390f32);
        assert_eq!(calculation_result.tree.get(&2401.into()).unwrap().needed.ceil(),      3671f32);
        assert_eq!(calculation_result.tree.get(&2463.into()).unwrap().needed.ceil(),       390f32);
        assert_eq!(calculation_result.tree.get(&2867.into()).unwrap().needed.ceil(),         6f32);
        assert_eq!(calculation_result.tree.get(&2868.into()).unwrap().needed.ceil(),        27f32);
        assert_eq!(calculation_result.tree.get(&2870.into()).unwrap().needed.ceil(),        13f32);
        assert_eq!(calculation_result.tree.get(&2871.into()).unwrap().needed.ceil(),         8f32);
        assert_eq!(calculation_result.tree.get(&2872.into()).unwrap().needed.ceil(),        33f32);
        assert_eq!(calculation_result.tree.get(&2876.into()).unwrap().needed.ceil(),        40f32);
        assert_eq!(calculation_result.tree.get(&3645.into()).unwrap().needed.ceil(),      3671f32);
        assert_eq!(calculation_result.tree.get(&3775.into()).unwrap().needed.ceil(),       390f32);

        // Commodities
        assert_eq!(calculation_result.tree.get(&57448.into()).unwrap().needed.ceil(),       26f32);
    }
}
