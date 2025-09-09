use starfoundry_lib_structures::{BonusVariations, StructureType};
use starfoundry_lib_types::TypeId;
use std::collections::{VecDeque, HashMap};
use std::fs::File;

use super::{Bonus, Dependency, DependencyBuildCost, DependencyTreeEntry, EngineResult, ProjectConfig};
use crate::engine::StockMinimal;
use crate::BlueprintTyp;

/// Group of dependencies.
/// 
#[derive(Debug, Default)]
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
