use sea_orm::*;

use crate::entities::email_template::{self, Entity as EmailTemplateEntity};

pub struct EmailTemplateRepo;

impl EmailTemplateRepo {
    pub async fn find_all(
        db: &DatabaseConnection,
    ) -> Result<Vec<email_template::Model>, DbErr> {
        EmailTemplateEntity::find()
            .order_by_asc(email_template::Column::TemplateType)
            .order_by_asc(email_template::Column::Language)
            .all(db)
            .await
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<Option<email_template::Model>, DbErr> {
        EmailTemplateEntity::find_by_id(id).one(db).await
    }

    pub async fn find_by_type_and_language(
        db: &DatabaseConnection,
        template_type: &str,
        language: &str,
    ) -> Result<Option<email_template::Model>, DbErr> {
        EmailTemplateEntity::find()
            .filter(email_template::Column::TemplateType.eq(template_type))
            .filter(email_template::Column::Language.eq(language))
            .one(db)
            .await
    }

    pub async fn update(
        db: &DatabaseConnection,
        model: email_template::ActiveModel,
    ) -> Result<email_template::Model, DbErr> {
        model.update(db).await
    }
}
