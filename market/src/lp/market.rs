use good_lp::{Constraint, Expression, ProblemVariables, Solution, SolverModel, Variable, constraint, default_solver, variable, variables};
use std::collections::HashMap;
use starfoundry_lib_types::{StructureId, TypeId};

use crate::market::MarketEntry;

pub struct MarketProblem {
    vars:        ProblemVariables,
    variables:   Vec<Variable>,
    prices:      Expression,

    want:        Expression,
    constraints: Vec<Constraint>,

    mapping_order_id_market: HashMap<i64, MarketEntry>,

    /// max recorded price per unit of the material
    max_per_unit_price: f64,
    /// total units that were recorded
    total_units:        f64,
}

impl MarketProblem {
    pub fn new() -> Self {
        Self {
            vars: variables!(),
            variables: Vec::new(),
            prices: 0.into(),
            want: 0.into(),
            constraints: Vec::new(),

            mapping_order_id_market: HashMap::new(),

            max_per_unit_price: 0f64,
            total_units: 0f64,
        }
    }

    pub fn calculate_market(
        &mut self,
        market_entries: Vec<MarketEntry>,
    ) {
        for entry in market_entries {
            let var_definition = variable()
                .name(entry.name())
                .min(0);
            let variable = self.vars.add(var_definition.clone());
            self.variables.push(variable);
            self.mapping_order_id_market.insert(entry.order_id, entry.clone());

            if entry.price > self.max_per_unit_price {
                self.max_per_unit_price = entry.price;
            }

            // TODO: make hauling optional
            self.prices += self.max_per_unit_price * (self.total_units + variable) + self.hauling_cost(entry.structure_id, variable, entry.item_volume);
            self.total_units += entry.quantity as f64;

            self.want += variable;
            self.constraints.push(constraint!(variable <= entry.quantity));
        }
    }

    fn hauling_cost(
        &self,
        structure_id: StructureId,
        quantity: Variable,
        volume: f64,
    ) -> Expression {
        match *structure_id {
            // UALX
            1046664001931i64 => {
                let hauling_full = 370_000f64;
                let hauling_fuel = 19_816_099f64;
                let hauling_per_m3 = hauling_fuel / hauling_full;
                (quantity * volume) * hauling_per_m3
            },
            // C-J
            1049588174021i64 => {
                let hauling_full = 370_000f64;
                let hauling_fuel = 113_886_795f64;
                let hauling_per_m3 = hauling_fuel / hauling_full;
                (quantity * volume) * hauling_per_m3
            },
            // Jita
            60003760i64 => {
                (quantity * volume) * 475f64
            },
            // Amarr
            60008494i64 => {
                let hauling_full = 370_000f64;
                let hauling_fuel = 173_566_003f64;
                let hauling_per_m3 = hauling_fuel / hauling_full;
                (quantity * volume) * hauling_per_m3
            },
            _ => unimplemented!(),
        }
    }

    pub fn solve(
        mut self,
        want: i32,
    ) -> HashMap<StructureId, MarketProblemResult> {
        self.constraints.push(constraint!(self.want == want));

        let mapping = self.variables
            .into_iter()
            .map(|var| (var, self.vars.display(&var).to_string()))
            .collect::<HashMap<_, _>>();

        let problem_result = self.vars
            .minimise(self.prices)
            .using(default_solver)
            .with_all(self.constraints)
            .solve()
            .unwrap();

        let mut result = HashMap::new();
        for (var, definition) in mapping.iter() {
            let buy_quantity = problem_result.value(*var);
            if buy_quantity > 0f64 {
                let name = format!("{}", definition);

                let order_id = name
                    .split_once("_")
                    .iter()
                    .last()
                    .unwrap().1
                    .parse::<i64>()
                    .unwrap();
                let entry = self.mapping_order_id_market.get(&order_id).unwrap();
                result
                    .entry(entry.structure_id)
                    .and_modify(|x: &mut MarketProblemResult| {
                        x.quantity += buy_quantity as i32;

                        if entry.price > x.price {
                            x.price = entry.price
                        }
                    })
                    .or_insert(MarketProblemResult {
                        quantity: buy_quantity as i32,
                        price: entry.price,
                        type_id: entry.type_id,
                    });
            }
        }

        result
    }
}

#[derive(Debug)]
pub struct MarketProblemResult {
    pub quantity: i32,
    pub price: f64,
    pub type_id: TypeId,
}

impl MarketEntry {
    pub fn name(&self) -> String {
        format!(
            "{}_{}",
            self.structure_id,
            self.order_id,
        )
    }
}
