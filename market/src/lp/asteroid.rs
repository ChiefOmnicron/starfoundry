use good_lp::{Constraint, Expression, ProblemVariables, Solution, SolverModel, Variable, constraint, default_solver, variable, variables};
use std::collections::HashMap;
use starfoundry_lib_types::{StructureId, TypeId};
use crate::lp::{Asteroid, MarketProblemResult, Mineral};
use crate::market::MarketEntry;

pub struct AsteroidCompressionProblem {
    vars:        ProblemVariables,
    variables:   Vec<Variable>,
    constraints: Vec<Constraint>,

    total_price: Expression,

    mapping_order_id_market: HashMap<i64, MarketEntry>,
    max_price_per_entry: HashMap<Asteroid, f64>,
    total_units_per_entry: HashMap<Asteroid, f64>,
    minerals: HashMap<Mineral, Expression>,
}

impl AsteroidCompressionProblem {
    pub fn new() -> Self {
        Self {
            vars: variables!(),
            variables: Vec::new(),
            constraints: Vec::new(),

            total_price: 0f64.into(),

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
            let variable = self.vars.add(definition);
            self.variables.push(variable);
            self.mapping_order_id_market.insert(entry.order_id, entry.clone());

            // get the max price that was recorded for the market
            let asteroid = Asteroid::from_type_id(entry.type_id);
            let mut max_price = self.max_price_per_entry.get(&asteroid).unwrap_or(&0f64);
            let total_units = self.total_units_per_entry.get(&asteroid).unwrap_or(&0f64);

            if entry.price > *max_price {
                max_price = &entry.price;
                self.max_price_per_entry.insert(asteroid.clone(), entry.price);
            }

            self.total_price += *max_price * (variable.clone());
            self.constraints.push(constraint!(variable.clone() >= 0));
            self.constraints.push(constraint!(variable <= total_units + entry.quantity as f64));

            self.total_units_per_entry.insert(asteroid.clone(), total_units + entry.quantity as f64);

            for (mineral, quantity) in asteroid.minerals() {
                self.minerals
                    .entry(mineral)
                    .and_modify(|x: &mut Expression| *x += variable * quantity)
                    .or_insert(variable * quantity);
            }
        }
    }

    pub fn solve(
        mut self,
        minerals: HashMap<Mineral, f64>,
    ) -> HashMap<StructureId, Vec<MarketProblemResult>> {
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
            .solve()
            .unwrap();

        let mut result = HashMap::new();
        for (var, definition) in mapping.iter() {
            let buy_quantity = problem_result.value(*var);
            if buy_quantity > 0f64 {
                dbg!(&buy_quantity);
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
                    .and_modify(|x: &mut Vec<MarketProblemResult>| {
                        x.push(MarketProblemResult {
                            quantity: buy_quantity as i32,
                            price: entry.price,
                            type_id: entry.type_id,
                        });
                    })
                    .or_insert(vec![MarketProblemResult {
                        quantity: buy_quantity as i32,
                        price: entry.price,
                        type_id: entry.type_id,
                    }]);
            }
        }

        result
    }
}

pub struct CompressedOreMarketEntry {
    type_id: TypeId,
    entries: Vec<MarketEntry>,
}

#[cfg(test)]
mod asteroid_lp_test {
    use crate::lp::{Asteroid, AsteroidCompressionProblem, Mineral};
    use sqlx::postgres::PgPoolOptions;
    use starfoundry_lib_eve_gateway::EveGatewayApiClientItem;
    use std::collections::HashMap;

    use crate::eve_gateway_api_client;
    use crate::market::MarketEntry;

    #[tokio::test]
    pub async fn basic() {
        unsafe {
            std::env::set_var("STARFOUNDRY_EVE_GATEWAY_API_URL", "http://localhost:9998");
            std::env::set_var("STARFOUNDRY_USER_AGENT", "StarFoundry Industry (github.com/chiefomnicron/starfoundry/market, 0.0.0)");
        }

        let pool = PgPoolOptions::new()
            .connect("postgresql://postgres:postgres@localhost:5432/dev-sf-market")
            .await
            .unwrap();

        let market_data = sqlx::query!("
                SELECT *
                FROM market_order_latest
                WHERE structure_id = 60003760
                AND type_id = ANY($1)
                AND is_buy = false
                ORDER BY price ASC
            ",
                &Asteroid::type_ids(),
                //&vec![17425]
            )
            .fetch_all(&pool)
            .await
            .unwrap()
            .into_iter()
            .map(|x| MarketEntry {
                order_id: x.order_id,
                price: x.price,
                quantity: (x.remaining as f64 / 100f64).floor() as i32,
                structure_id: x.structure_id.into(),
                type_id: x.type_id.into(),
                item_volume: 1f64,
            })
            .collect::<Vec<_>>();

        let mut problem = AsteroidCompressionProblem::new();
        problem.define_problem(market_data);
        let result = problem.solve(
            vec![
                (Mineral::Isogen, 594847f64),
                (Mineral::Megacyte, 15275f64),
                (Mineral::Mexallon, 2185180f64),
                (Mineral::Nocxium, 62866f64),
                (Mineral::Pyerite, 7775689f64),
                (Mineral::Tritanium, 2306201f64),
                (Mineral::Zydrine, 30460f64),
            ]
            .into_iter()
            .collect::<HashMap<_, _>>()
        );

        let type_ids = result
            .iter()
            .flat_map(|(_, x)|
                x.into_iter().map(|y| y.type_id).collect::<Vec<_>>()
            ).collect::<Vec<_>>();
        let market = result
            .into_iter()
            .flat_map(|(_, x)| x)
            .collect::<Vec<_>>();

        let items = eve_gateway_api_client()
            .unwrap()
            .fetch_item_bulk(type_ids)
            .await
            .unwrap();

        for item in items {
            if let Some(x) = market.iter().find(|x| x.type_id == item.type_id) {
                println!("{}\t{}", item.name, x.quantity * 100);
            }
        }
    }
}
