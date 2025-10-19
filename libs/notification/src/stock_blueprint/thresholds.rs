use sqlx::PgPool;
use starfoundry_lib_types::TypeId;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct BlueprintThreshold {
    pub type_id: TypeId,
    pub name: String,
    pub want: i32,
    pub critical: i32,
    pub min_runs: i32,
    pub min_me: i32,
    pub min_te: i32,
}

#[derive(Clone, Debug)]
pub struct BlueprintThresholds(Vec<BlueprintThreshold>);

impl BlueprintThresholds {
    pub async fn load(
        pool:     &PgPool,
        stock_id: Uuid,
    ) -> Self {
        let mut thresholds = Vec::new();

        sqlx::query!("
                SELECT
                    sbt.blueprint_stock_id,
                    sbt.want,
                    sbt.critical,
                    sbt.min_runs,
                    sbt.min_me,
                    sbt.min_te,
                    i.type_id,
                    i.name
                FROM stock_blueprint_threshold sbt
                JOIN item i ON i.type_id = sbt.type_id
                JOIN blueprints_temp bt ON bt.type_id = sbt.type_id
                WHERE blueprint_stock_id = $1
            ",
                stock_id,
            )
            .fetch_all(pool)
            .await
            .unwrap()
            .into_iter()
            .for_each(|x| {
                let stock = BlueprintThreshold {
                    type_id: x.type_id.into(),
                    name: x.name,
                    want: x.want,
                    critical: x.critical,
                    min_runs: x.min_runs,
                    min_me: x.min_me,
                    min_te: x.min_te,
                };

                thresholds.push(stock);
            });

        Self(thresholds)
    }

    pub fn entry(
        &self,
        type_id: TypeId,
    ) -> BlueprintThreshold {
        self.0
            .iter()
            .find(|x| x.type_id == type_id)
            .unwrap()
            .clone()
    }

    pub fn type_ids(&self) -> Vec<TypeId> {
        self.0
            .iter()
            .map(|x| x.type_id)
            .collect()
    }
}
