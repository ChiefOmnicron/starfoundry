use good_lp::{constraint, default_solver, variable, variables, Constraint, Expression, ProblemVariables, Solution, SolverModel, Variable};
use std::collections::HashMap;

pub struct MarketProblem {
    vars:        ProblemVariables,
    variables:   Vec<Variable>,
    prices:      Expression,

    want:        Expression,
    constraints: Vec<Constraint>,

    mapping_order_id_market: HashMap<i64, MarketLpEntry>,
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
        }
    }

    pub fn calculate_market(
        &mut self,
        market_entries: Vec<MarketLpEntry>,
    ) {
        for entry in market_entries {
            let var_definition = variable().name(format!(
                "{}_{}",
                entry.structure_id,
                entry.order_id,
            ));
            let variable = self.vars.add(var_definition.clone());
            self.variables.push(variable);
            self.mapping_order_id_market.insert(entry.order_id, entry.clone());

            self.prices += variable * entry.price;

            self.want += variable;
            self.constraints.push(constraint!(variable >= 0));
            self.constraints.push(constraint!(variable <= entry.quantity));
        }
    }

    pub fn solve(
        mut self,
        want: i32,
    ) -> HashMap<i64, MarketProblemResult> {
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
                        price: entry.price
                    });

                //result.insert(name, buy_quantity);
            }
        }

        result
    }
}

#[derive(Debug)]
pub struct MarketProblemResult {
    pub quantity: i32,
    pub price: f64,
}

#[derive(Clone, Debug)]
pub struct MarketLpEntry {
    pub order_id:     i64,
    pub structure_id: i64,
    pub price:        f64,
    pub quantity:     i32,
}
