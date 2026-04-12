use good_lp::{Constraint, Expression, ProblemVariables, Solution, SolverModel, Variable, WithTimeLimit, constraint, default_solver, variable, variables};
use starfoundry_lib_market::{Asteroid, Mineral};
use starfoundry_lib_types::{StructureId, TypeId};
use std::collections::HashMap;

use crate::lp::{LpError, MarketProblemResult, Result};
use crate::market::MarketEntry;

pub struct AsteroidCompressionProblem {
    vars:        ProblemVariables,
    variables:   Vec<Variable>,
    constraints: Vec<Constraint>,

    total_price: Expression,
    total_units: HashMap<TypeId, Expression>,

    mapping_order_id_market:    HashMap<i64, MarketEntry>,
    max_price_per_entry:        HashMap<Asteroid, f64>,
    total_units_per_entry:      HashMap<Asteroid, f64>,
    minerals:                   HashMap<Mineral, Expression>,
}

impl AsteroidCompressionProblem {
    pub fn new() -> Self {
        Self {
            vars: variables!(),
            variables: Vec::new(),
            constraints: Vec::new(),

            total_price: 0f64.into(),
            total_units: HashMap::new(),

            mapping_order_id_market: HashMap::new(),
            max_price_per_entry: HashMap::new(),
            total_units_per_entry: HashMap::new(),
            minerals: HashMap::new(),
        }
    }

    pub fn define_problem(
        &mut self,
        entries: Vec<MarketEntry>,
    ) {
        for entry in entries {
            let definition = variable().name(entry.name());
            let var = self.vars.add(definition);
            self.variables.push(var);
            self.mapping_order_id_market.insert(entry.order_id, entry.clone());

            // get the max price that was recorded for the market
            let asteroid = Asteroid::from_type_id(entry.type_id);
            let mut max_price = self.max_price_per_entry.get(&asteroid).unwrap_or(&0f64);

            if entry.price > *max_price {
                max_price = &entry.price;
                self.max_price_per_entry.insert(asteroid.clone(), entry.price);
            }

            self.total_price += *max_price * var + self.hauling_cost(entry.structure_id, var, entry.item_volume);
            self.constraints.push(constraint!(var >= 0));

            if asteroid.is_any_asteroid() {
                let quantity = ((entry.quantity / 100) as f64).floor();
                self.constraints.push(constraint!(var <= quantity));
            } else {
                self.constraints.push(constraint!(var <= entry.quantity as f64));
            }

            for (mineral, quantity) in asteroid.minerals() {
                let quantity = if asteroid.is_any_asteroid() {
                    (var * quantity) * 0.9063
                } else {
                    var * quantity
                };


                self.minerals
                    .entry(mineral)
                    .and_modify(|x: &mut Expression| *x += quantity.clone())
                    .or_insert(quantity);
            }
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
        minerals: HashMap<Mineral, f64>,
    ) -> Result<HashMap<(StructureId, TypeId), MarketProblemResult>> {
        for (mineral, quantity) in minerals {
            if let Some(x) = self.minerals.get(&mineral) {
                self.constraints.push(constraint!(x.clone() >= quantity));
            }
        }

        let mapping = self.variables
            .into_iter()
            .map(|var| (var, self.vars.display(&var).to_string()))
            .collect::<HashMap<_, _>>();

        let problem_result = self.vars
            .minimise(self.total_price)
            .using(default_solver)
            .with_all(self.constraints)
            .with_time_limit(10)
            .solve()
            .map_err(|_| LpError::NoSolution)?;

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

                let quantity = if Asteroid::from_type_id(entry.type_id).is_any_asteroid() {
                    buy_quantity.ceil() * 100f64
                } else {
                    buy_quantity
                } as i32;

                result
                    .entry((entry.structure_id, entry.type_id))
                    .and_modify(|x: &mut MarketProblemResult| {
                        //x.push(MarketProblemResult {
                        //    quantity: buy_quantity as i32,
                        //    price: entry.price,
                        //    type_id: entry.type_id,
                        //});

                        x.quantity += quantity;

                        if entry.price > x.price {
                            x.price = entry.price
                        }
                    })
                    .or_insert(MarketProblemResult {
                        quantity: quantity,
                        price: entry.price,
                        type_id: entry.type_id,
                    });
            }
        }

        Ok(result)
    }
}


