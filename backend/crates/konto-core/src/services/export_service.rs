use csv::Writer;
use konto_common::error::AppError;

pub struct ExportService;

impl ExportService {
    pub fn to_csv<T: serde::Serialize>(records: &[T]) -> Result<Vec<u8>, AppError> {
        let mut wtr = Writer::from_writer(vec![]);
        for record in records {
            wtr.serialize(record)
                .map_err(|e| AppError::Internal(e.to_string()))?;
        }
        wtr.into_inner()
            .map_err(|e| AppError::Internal(e.to_string()))
    }
}
