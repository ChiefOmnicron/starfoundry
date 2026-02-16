use sqlx::PgPool;
use starfoundry_lib_eve_gateway::{Category, Group, Item};
use starfoundry_lib_types::{CategoryId, GroupId, TypeId};
use std::collections::HashMap;
use std::time::Instant;

use crate::Error;
use crate::parser::groups::GroupIdEntry;
use crate::parser::type_ids::TypeIdEntry;
use crate::parser::categories::CategoryIdEntry;

pub async fn run(
    pool:         &PgPool,
    category_ids: &HashMap<CategoryId, CategoryIdEntry>,
    group_ids:    &HashMap<GroupId, GroupIdEntry>,
    type_ids:     &HashMap<TypeId, TypeIdEntry>,
    repackaged:   &HashMap<TypeId, i32>,
) -> Result<(), Error> {
    tracing::info!("Processing items");
    let start = Instant::now();

    let items = prepare_data(
        &category_ids,
        &group_ids,
        &type_ids,
        &repackaged,
    ).await?;

    let mut transaction = pool
        .begin()
        .await
        .map_err(Error::TransactionError)?;

    tracing::debug!("Clearing database");
    sqlx::query!("
            DELETE FROM item
        ")
        .execute(&mut *transaction)
        .await
        .map_err(Error::DeleteItems)?;
    sqlx::query!("
            DELETE FROM category
        ")
        .execute(&mut *transaction)
        .await
        .map_err(Error::DeleteItems)?;
    sqlx::query!("
            DELETE FROM groups
        ")
        .execute(&mut *transaction)
        .await
        .map_err(Error::DeleteItems)?;
    tracing::debug!("Clearing database done");

    let mut type_id       = Vec::new();
    let mut category_id   = Vec::new();
    let mut group_id      = Vec::new();
    let mut meta_group_id = Vec::new();
    let mut volume        = Vec::new();
    let mut packaged      = Vec::new();
    let mut name          = Vec::new();

    for item in items {
        type_id.push(*item.type_id);
        name.push(item.name);
        category_id.push(*item.category.category_id);
        group_id.push(*item.group.group_id);
        meta_group_id.push(item.meta_group);
        volume.push(item.volume);
        packaged.push(repackaged.get(&item.type_id));
    }

    tracing::debug!("Inserting data");
    sqlx::query!("
            INSERT INTO item
            (
                category_id,
                group_id,
                meta_group_id,
                name,
                type_id,
                volume,
                repackaged
            )
            SELECT * FROM UNNEST(
                $1::INTEGER[],
                $2::INTEGER[],
                $3::INTEGER[],
                $4::VARCHAR[],
                $5::INTEGER[],
                $6::REAL[],
                $7::INTEGER[]
            )
        ",
            &category_id,
            &group_id,
            &meta_group_id as _,
            &name,
            &type_id,
            &volume,
            &packaged as _,
        )
        .execute(&mut *transaction)
        .await
        .map_err(Error::InsertItems)?;

    sqlx::query!("
            INSERT INTO category
            (
                category_id,
                name
            )
            SELECT * FROM UNNEST(
                $1::INTEGER[],
                $2::VARCHAR[]
            )
        ",
            &category_ids.into_iter().map(|(x, _)| **x).collect::<Vec<_>>(),
            &category_ids.into_iter().map(|(_, entry)| entry.name().unwrap_or_default()).collect::<Vec<_>>(),
        )
        .execute(&mut *transaction)
        .await
        .map_err(Error::InsertItems)?;

    sqlx::query!("
            INSERT INTO groups
            (
                group_id,
                category_id,
                name
            )
            SELECT * FROM UNNEST(
                $1::INTEGER[],
                $2::INTEGER[],
                $3::VARCHAR[]
            )
        ",
            &group_ids.into_iter().map(|(x, _)| **x).collect::<Vec<_>>(),
            &group_ids.into_iter().map(|(_, entry)| *entry.category_id).collect::<Vec<_>>(),
            &group_ids.into_iter().map(|(_, entry)| entry.name().unwrap_or_default()).collect::<Vec<_>>(),
        )
        .execute(&mut *transaction)
        .await
        .map_err(Error::InsertItems)?;

    tracing::debug!("Inserting data done");

    transaction
        .commit()
        .await
        .map_err(Error::TransactionError)?;
    tracing::debug!("Transaction committed");

    tracing::info!(
        "Finished processing items, task took {:.2}s",
        start.elapsed().as_secs_f64()
    );

    Ok(())
}

async fn prepare_data(
    category_ids: &HashMap<CategoryId, CategoryIdEntry>,
    group_ids:    &HashMap<GroupId, GroupIdEntry>,
    type_ids:     &HashMap<TypeId, TypeIdEntry>,
    repackaged:   &HashMap<TypeId, i32>,
) -> Result<Vec<Item>, Error> {
    let mut items = Vec::new();

    for (type_id, entry) in type_ids {
        let type_id = *type_id;
        let group_id = entry.group_id.into();
        let category_id = group_ids
            .get(&group_id)
            .map(|x| x.category_id.into())
            .expect("Every entry should have a category id");
        let volume = entry.volume.unwrap_or(0f32);
        let meta_group_id = entry.meta_group_id.map(Into::into);
        let name = entry.name().unwrap_or(format!("Unknown name {}", type_id));
        let repackaged = repackaged.get(&type_id).cloned();

        let group_fetched = group_ids.get(&group_id).unwrap();
        let group = Group {
            category_id: group_fetched.category_id,
            group_id: group_id,
            name: group_fetched.name().unwrap(),
        };

        let category_fetched = category_ids.get(&category_id).unwrap();
        let category = Category {
            category_id: category_id,
            name: category_fetched.name().unwrap(),
        };

        let item = Item {
            type_id,
            group,
            meta_group: meta_group_id,
            category,
            volume,
            name,
            repackaged,
        };
        items.push(item);
    }

    Ok(items)
}

pub fn get_item(
    type_id:      TypeId,
    category_ids: &HashMap<CategoryId, CategoryIdEntry>,
    group_ids:    &HashMap<GroupId, GroupIdEntry>,
    type_ids:     &HashMap<TypeId, TypeIdEntry>,
    repackaged:   &HashMap<TypeId, i32>,
) -> Item {
    let entry = type_ids
        .get(&type_id)
        .unwrap();

    let group_id = entry.group_id.into();
    let category_id = group_ids
        .get(&group_id)
        .map(|x| x.category_id.into())
        .expect("Every entry should have a category id");
    let volume = entry.volume.unwrap_or(0f32);
    let meta_group_id = entry.meta_group_id.map(Into::into);
    let name = entry.name().unwrap_or(format!("Unknown name {}", type_id));
    let repackaged = repackaged.get(&type_id).cloned();

    let group_fetched = group_ids.get(&group_id).unwrap();
    let group = Group {
        category_id: group_fetched.category_id,
        group_id: group_id,
        name: group_fetched.name().unwrap(),
    };

    let category_fetched = category_ids.get(&category_id).unwrap();
    let category = Category {
        category_id: category_id,
        name: category_fetched.name().unwrap(),
    };

    Item {
        type_id,
        group,
        meta_group: meta_group_id,
        category,
        volume,
        name,
        repackaged,
    }
}
