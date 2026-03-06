use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Employees::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Employees::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Employees::UserId).string().null())
                    .col(ColumnDef::new(Employees::FirstName).string().not_null())
                    .col(ColumnDef::new(Employees::LastName).string().not_null())
                    .col(ColumnDef::new(Employees::AhvNumber).string().not_null())
                    .col(ColumnDef::new(Employees::DateOfBirth).date().not_null())
                    .col(ColumnDef::new(Employees::Street).string().not_null())
                    .col(ColumnDef::new(Employees::PostalCode).string().not_null())
                    .col(ColumnDef::new(Employees::City).string().not_null())
                    .col(
                        ColumnDef::new(Employees::Country)
                            .string()
                            .not_null()
                            .default("CH"),
                    )
                    .col(ColumnDef::new(Employees::Iban).string().not_null())
                    .col(ColumnDef::new(Employees::Bic).string().null())
                    .col(ColumnDef::new(Employees::BankName).string().null())
                    .col(ColumnDef::new(Employees::EmploymentStart).date().not_null())
                    .col(ColumnDef::new(Employees::EmploymentEnd).date().null())
                    .col(
                        ColumnDef::new(Employees::EmploymentPercentage)
                            .decimal_len(5, 2)
                            .not_null()
                            .default("100.00"),
                    )
                    .col(
                        ColumnDef::new(Employees::GrossMonthlySalary)
                            .decimal_len(15, 2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Employees::HasChildren)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Employees::NumberOfChildren)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Employees::ChildAllowanceAmount)
                            .decimal_len(10, 2)
                            .not_null()
                            .default("215.00"),
                    )
                    .col(
                        ColumnDef::new(Employees::EducationAllowanceAmount)
                            .decimal_len(10, 2)
                            .not_null()
                            .default("268.00"),
                    )
                    .col(
                        ColumnDef::new(Employees::IsQuellensteuer)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Employees::QuellensteuerTariff).string().null())
                    .col(
                        ColumnDef::new(Employees::QuellensteuerRate)
                            .decimal_len(5, 4)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Employees::MaritalStatus)
                            .string()
                            .not_null()
                            .default("single"),
                    )
                    .col(
                        ColumnDef::new(Employees::Canton)
                            .string()
                            .not_null()
                            .default("BS"),
                    )
                    .col(
                        ColumnDef::new(Employees::Status)
                            .string()
                            .not_null()
                            .default("active"),
                    )
                    .col(ColumnDef::new(Employees::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Employees::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Employees::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Employees {
    Table,
    Id,
    UserId,
    FirstName,
    LastName,
    AhvNumber,
    DateOfBirth,
    Street,
    PostalCode,
    City,
    Country,
    Iban,
    Bic,
    BankName,
    EmploymentStart,
    EmploymentEnd,
    EmploymentPercentage,
    GrossMonthlySalary,
    HasChildren,
    NumberOfChildren,
    ChildAllowanceAmount,
    EducationAllowanceAmount,
    IsQuellensteuer,
    QuellensteuerTariff,
    QuellensteuerRate,
    MaritalStatus,
    Canton,
    Status,
    CreatedAt,
    UpdatedAt,
}
