use chrono::Utc;
use konto_common::enums::AccountType;
use konto_common::error::AppError;
use konto_db::entities::{account, journal_line};
use konto_db::repository::account_repo::AccountRepo;
use rust_decimal::Decimal;
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use serde::Serialize;
use std::collections::HashMap;
use uuid::Uuid;

pub struct AccountService;

impl AccountService {
    pub async fn list(
        db: &DatabaseConnection,
        page: u64,
        per_page: u64,
        search: Option<&str>,
    ) -> Result<(Vec<account::Model>, u64), AppError> {
        AccountRepo::find_paginated(db, page, per_page, search)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<account::Model>, AppError> {
        AccountRepo::find_all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn get_by_id(db: &DatabaseConnection, id: &str) -> Result<account::Model, AppError> {
        AccountRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Account not found".to_string()))
    }

    pub async fn create(
        db: &DatabaseConnection,
        number: i32,
        name: &str,
        description: Option<String>,
        parent_id: Option<String>,
        currency_id: Option<String>,
    ) -> Result<account::Model, AppError> {
        // Check for duplicate number
        if let Some(_existing) = AccountRepo::find_by_number(db, number)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
        {
            return Err(AppError::Conflict(format!("Account number {number} already exists")));
        }

        let account_type = AccountType::from_account_number(number);
        let now = Utc::now().naive_utc();

        let model = account::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            number: Set(number),
            name: Set(name.to_string()),
            account_type: Set(format!("{account_type:?}").to_lowercase()),
            description: Set(description),
            parent_id: Set(parent_id),
            currency_id: Set(currency_id),
            is_active: Set(true),
            created_at: Set(now),
            updated_at: Set(now),
        };

        AccountRepo::create(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: &str,
        name: Option<String>,
        description: Option<Option<String>>,
        is_active: Option<bool>,
        parent_id: Option<Option<String>>,
        currency_id: Option<Option<String>>,
    ) -> Result<account::Model, AppError> {
        let existing = Self::get_by_id(db, id).await?;
        let mut model: account::ActiveModel = existing.into();

        if let Some(name) = name {
            model.name = Set(name);
        }
        if let Some(desc) = description {
            model.description = Set(desc);
        }
        if let Some(active) = is_active {
            model.is_active = Set(active);
        }
        if let Some(pid) = parent_id {
            model.parent_id = Set(pid);
        }
        if let Some(cid) = currency_id {
            model.currency_id = Set(cid);
        }
        model.updated_at = Set(Utc::now().naive_utc());

        AccountRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn get_tree_with_balances(
        db: &DatabaseConnection,
    ) -> Result<Vec<AccountNodeWithBalance>, AppError> {
        let accounts = AccountRepo::find_all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        // Compute balances from journal_lines: sum(debit) - sum(credit) per account
        let lines = journal_line::Entity::find()
            .all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let mut balance_map: HashMap<String, Decimal> = HashMap::new();
        for line in &lines {
            let entry = balance_map.entry(line.account_id.clone()).or_insert(Decimal::ZERO);
            *entry += line.debit_amount - line.credit_amount;
        }

        // Build tree
        let nodes: Vec<AccountNodeWithBalance> = accounts
            .iter()
            .map(|a| AccountNodeWithBalance {
                id: a.id.clone(),
                number: a.number,
                name: a.name.clone(),
                account_type: a.account_type.clone(),
                parent_id: a.parent_id.clone(),
                balance: *balance_map.get(&a.id).unwrap_or(&Decimal::ZERO),
                is_active: a.is_active,
                children: Vec::new(),
            })
            .collect();

        Ok(build_tree(nodes))
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<(), AppError> {
        Self::get_by_id(db, id).await?;
        AccountRepo::delete(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AccountNodeWithBalance {
    pub id: String,
    pub number: i32,
    pub name: String,
    pub account_type: String,
    pub parent_id: Option<String>,
    #[serde(with = "rust_decimal::serde::str")]
    pub balance: Decimal,
    pub is_active: bool,
    pub children: Vec<AccountNodeWithBalance>,
}

fn build_tree(nodes: Vec<AccountNodeWithBalance>) -> Vec<AccountNodeWithBalance> {
    let mut by_id: HashMap<String, AccountNodeWithBalance> = HashMap::new();
    let mut children_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut root_ids: Vec<String> = Vec::new();

    for node in nodes {
        let id = node.id.clone();
        if let Some(ref pid) = node.parent_id {
            children_map.entry(pid.clone()).or_default().push(id.clone());
        } else {
            root_ids.push(id.clone());
        }
        by_id.insert(id, node);
    }

    root_ids.sort_by_key(|id| by_id.get(id).map(|n| n.number).unwrap_or(0));

    fn collect(
        id: &str,
        by_id: &mut HashMap<String, AccountNodeWithBalance>,
        children_map: &HashMap<String, Vec<String>>,
    ) -> Option<AccountNodeWithBalance> {
        let mut node = by_id.remove(id)?;
        if let Some(child_ids) = children_map.get(id) {
            let mut sorted_ids = child_ids.clone();
            sorted_ids.sort_by_key(|cid| by_id.get(cid).map(|n| n.number).unwrap_or(0));
            for cid in &sorted_ids {
                if let Some(child) = collect(cid, by_id, children_map) {
                    node.children.push(child);
                }
            }
        }
        Some(node)
    }

    let mut result = Vec::new();
    for id in &root_ids {
        if let Some(node) = collect(id, &mut by_id, &children_map) {
            result.push(node);
        }
    }
    result
}
