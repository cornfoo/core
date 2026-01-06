#[derive(Debug, Clone)]
pub struct InsertBuilder {
    table: String,
    columns: Vec<String>,
    returning: Option<String>,
    on_conflict: Option<String>,
}

impl InsertBuilder {
    pub fn new(table: &str) -> Self {
        Self {
            table: table.to_string(),
            columns: Vec::new(),
            returning: None,
            on_conflict: None,
        }
    }

    pub fn columns(mut self, cols: &[&str]) -> Self {
        self.columns = cols.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn returning(mut self, cols: &str) -> Self {
        self.returning = Some(cols.to_string());
        self
    }

    pub fn on_conflict_do_nothing(mut self) -> Self {
        self.on_conflict = Some("ON CONFLICT DO NOTHING".to_string());
        self
    }

    pub fn on_conflict_update(mut self, constraint: &str, update_cols: &[&str]) -> Self {
        let updates = update_cols
            .iter()
            .map(|col| format!("{} = EXCLUDED.{}", col, col))
            .collect::<Vec<_>>()
            .join(", ");
        self.on_conflict = Some(format!(
            "ON CONFLICT ({}) DO UPDATE SET {}",
            constraint, updates
        ));
        self
    }

    pub fn build(&self) -> String {
        assert!(
            !self.columns.is_empty(),
            "InsertBuilder error: no columns provided"
        );

        let placeholders = (1..=self.columns.len())
            .map(|i| format!("${}", i))
            .collect::<Vec<_>>()
            .join(", ");

        let mut query = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            self.table,
            self.columns.join(", "),
            placeholders
        );

        if let Some(ref conflict) = self.on_conflict {
            query.push_str(&format!(" {}", conflict));
        }

        if let Some(ref ret) = self.returning {
            query.push_str(&format!(" RETURNING {}", ret));
        }

        query
    }
}
