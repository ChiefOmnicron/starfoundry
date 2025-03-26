use async_trait::async_trait;
use sqlx::PgPool;
use starfoundry_libs_eve_api::{BlueprintInfo, Credentials};
use starfoundry_libs_types::TypeId;
use std::collections::HashMap;
use uuid::Uuid;

use crate::{Discord, DiscordAddField, DiscordEmbedBuilder, Notification};

mod check;
mod thresholds;

use self::check::*;
use self::thresholds::*;
use crate::Error;
use crate::DiscordEmbed;
use crate::NotificationTarget;
use serde::Serialize;

pub struct StockBlueprint {
    id: Uuid,
}

impl StockBlueprint {
    pub fn new(
        id: Uuid,
    ) -> Self {
        Self {
            id
        }
    }

    pub async fn send(
        &self,
        pool:        &PgPool,
        credentials: &Credentials,
    ) -> Result<(), Error> {
        let stock = BlueprintThresholds::load(&pool, self.id).await;
        let mut findings = Self::run_check(
                stock.clone(),
                credentials,
            )
            .await?;
        findings.sort_by_key(|x| x.want - x.has);
        findings.reverse();

        let notifications = self.notifications(&pool).await?;
        for (url, target) in notifications {
            match target {
                NotificationTarget::Discord => {
                    let discord = self.discord_message(&mut findings);
                    self.discord(url, discord)
                },
                NotificationTarget::Json => {
                    let value = self.json_message(&mut findings);
                    self.json(url, value)
                },
            }
            .await
            .map(drop)?
        }

        Ok(())
    }

    fn discord_message(
        &self,
        findings: &mut Vec<Finding>,
    ) -> Vec<Discord> {
        let critical_embed = DiscordEmbed::new(
            "Critical",
            "",
            crate::DiscordColor::DarkRed,
        );
        let critical_filtered = findings
            .iter()
            .filter(|x| x.action == FindingAction::Critical)
            .cloned()
            .collect::<Vec<_>>();
        let critical_embeds = StockBlueprint::add_entries(critical_embed, critical_filtered);

        let non_critical_embed = DiscordEmbed::new(
            "Non-Critical",
            "",
            crate::DiscordColor::DarkOrange,
        );
        let non_critical_filtered = findings
            .iter()
            .filter(|x| x.action == FindingAction::NonCritical)
            .cloned()
            .collect::<Vec<_>>();
        let non_critical_embeds = StockBlueprint::add_entries(non_critical_embed, non_critical_filtered);

        let mut messages: Vec<Discord> = Vec::new();
        if critical_embeds.is_empty() && non_critical_embeds.is_empty() {
            let all_good = DiscordEmbed::new(
                "Nothing todo",
                "",
                crate::DiscordColor::DarkGreen,
            );

            messages.push(Discord {
                content: "".into(),
                embeds: vec![all_good],
            })
        }

        messages
            .extend(
                critical_embeds
                    .into_iter()
                    .map(|x| Discord {
                        content: "".into(),
                        embeds: vec![x]
                    })
            );
        messages
            .extend(
                non_critical_embeds
                    .into_iter()
                    .map(|x| Discord {
                        content: "".into(),
                        embeds: vec![x]
                    })
            );

        /*if !critical_embed.fields.is_empty() {
            discord.embeds.push(critical_embed);
        }
        if !non_critical_embed.fields.is_empty() {
            discord.embeds.push(non_critical_embed);
        }*/

        messages
    }

    fn json_message(
        &self,
        findings: &mut Vec<Finding>,
    ) -> serde_json::Value {
        #[derive(Serialize)]
        struct TmpEntry {
            blueprint: String,
            has: usize,
            missing: usize,
            want: usize,
        }

        #[derive(Serialize)]
        struct TmpResponse {
            event:        String,
            critical:     Vec<TmpEntry>,
            non_critical: Vec<TmpEntry>,
        }

        let mut critical = Vec::new();
        let mut non_critical = Vec::new();

        for finding in findings {
            match finding.action {
                FindingAction::Critical => {
                    critical.push(TmpEntry {
                        blueprint: finding.blueprint.clone(),
                        has: finding.has,
                        missing: finding.want - finding.has,
                        want: finding.want,
                    });
                },
                FindingAction::NonCritical => {
                    non_critical.push(TmpEntry {
                        blueprint: finding.blueprint.clone(),
                        has: finding.has,
                        missing: finding.want - finding.has,
                        want: finding.want,
                    });
                },
                _ => continue
            }
        }

        let response = TmpResponse {
            event: "STOCK_BLUEPRINT".into(),
            critical,
            non_critical
        };
        serde_json::to_value(&response).unwrap_or_default()
    }

    async fn notifications(
        &self,
        pool: &PgPool,
    ) -> Result<Vec<(String, NotificationTarget)>, Error> {
        sqlx::query!(r#"
                SELECT
                    url,
                    target AS "target!: NotificationTarget"
                FROM stock_blueprints sb
                JOIN notifications n ON n.id = ANY(sb.notifications)
                WHERE sb.id = $1
            "#,
                self.id,
            )
            .fetch_all(pool)
            .await
            .map(|x| {
                x.into_iter()
                    .map(|y| (y.url, y.target))
                    .collect::<Vec<_>>()
            })
            .map_err(|e| Error::FetchStockBlueprintNotifications(e, self.id))
    }

    // TODO: replace with database
    async fn run_check(
        blueprint_thresholds: BlueprintThresholds,
        credential_cache:     &Credentials,
    ) -> Result<Vec<Finding>, Error> {
        let mut blueprints: HashMap<TypeId, Vec<BlueprintEntry>> = HashMap::new();

        // clients
        let rci_api_client = crate::api_client(
                98024275.into(),
                credential_cache.clone(),
            )
            .await
            .unwrap();
        let fis_api_client = crate::api_client(
                98748294.into(),
                credential_cache.clone(),
            )
            .await
            .unwrap();

        // fetch bpcs
        let mut rci_blueprints = rci_api_client
            .corporation_blueprints()
            .await
            .map_err(Error::ConnectError)?;

        let fis_blueprints = fis_api_client
            .corporation_blueprints()
            .await
            .map_err(Error::ConnectError)?;

        rci_blueprints.extend(fis_blueprints);

        let filtered = rci_blueprints
            .iter()
            .filter(|x| blueprint_thresholds.type_ids().contains(&x.type_id));
        for blueprint in filtered {
            let entry = BlueprintEntry::from(blueprint.clone());

            blueprints
                .entry(blueprint.type_id)
                .and_modify(|x| x.push(entry))
                .or_insert(vec![entry]);
        }

        let mut findings = Vec::new();

        for (type_id, entries) in blueprints {
            let threshold_entry = blueprint_thresholds.entry(type_id);
            let full_run_bpc = entries
                .iter()
                .filter(|x| x.typ == BlueprintType::Copy)
                .filter(|x| x.runs >= threshold_entry.min_runs as i32)
                .filter(|x| x.me >= threshold_entry.min_me as i32)
                .filter(|x| x.te >= threshold_entry.min_te as i32)
                .collect::<Vec<_>>();

            let action = if full_run_bpc.len() >= threshold_entry.want as usize {
                FindingAction::Ignore
            } else if full_run_bpc.len() <= threshold_entry.critical as usize {
                FindingAction::Critical
            } else if full_run_bpc.len() > threshold_entry.critical as usize {
                FindingAction::NonCritical
            } else {
                FindingAction::Unknown
            };

            findings.push(Finding {
                blueprint: threshold_entry.name,
                has: full_run_bpc.len(),
                want: threshold_entry.want as usize,
                action,
            });
        }

        Ok(findings)
    }

    fn add_entries(
        embed:    DiscordEmbed,
        findings: Vec<Finding>,
    ) -> Vec<DiscordEmbed> {
        /*let mut blueprints = String::new();
        let mut missing = String::new();

        for finding in findings {
            blueprints += &format!("{}\n", finding.blueprint);
            missing += &format!("{} / {}\n", finding.has, finding.want);
        }

        if !blueprints.is_empty() {
            embed.fields.push(
                DiscordField::new("Blueprint", &blueprints),
            );
            embed.fields.push(
                DiscordField::new("Have / Want", &missing),
            );
        }*/

        let mut blueprints = Vec::new();
        let mut missing = Vec::new();

        for finding in findings {
            blueprints.push(format!("{}", finding.blueprint));
            missing.push(format!("{} / {}", finding.has, finding.want));
        }

        if !blueprints.is_empty() {
            DiscordEmbedBuilder::new(embed)
                .build_two_fields(
                    DiscordAddField {
                        name: "Blueprints".into(),
                        entries: blueprints
                    },
                    DiscordAddField {
                        name: "Has/Want".into(),
                        entries: missing,
                    }
                )
        } else {
            Vec::new()
        }
    }
}

#[async_trait]
impl Notification for StockBlueprint {}

#[derive(Clone, Copy, Debug)]
struct BlueprintEntry {
    pub typ:     BlueprintType,
    pub runs:    i32,
    #[allow(unused)]
    pub type_id: TypeId,
    pub me:      i32,
    pub te:      i32,
}

impl From<BlueprintInfo> for BlueprintEntry {
    fn from(value: BlueprintInfo) -> Self {
        Self {
            typ: BlueprintType::from(value.quantity),
            runs: value.runs,
            type_id: value.type_id,
            me: value.material_efficiency as i32,
            te: value.time_efficiency as i32,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BlueprintType {
    Original,
    Copy,
    Unknown,
}

impl From<i32> for BlueprintType {
    fn from(value: i32) -> Self {
        match value {
            -2 => Self::Copy,
            -1 => Self::Original,
            _  => Self::Unknown,
        }
    }
}
