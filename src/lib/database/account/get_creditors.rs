use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};

use crate::lib::database;
use crate::lib::database::models::Creditor;
use crate::lib::database::schema::creditor;
use crate::lib::database::schema::creditor::business_name;

pub type Creditors = Vec<Creditor>;

pub fn get_creditors() -> Creditors {
    let mut conn = database::establish_connection();

    let all_names = creditor::table
        .distinct()
        .order(business_name)
        .select(Creditor::as_select())
        .load::<Creditor>(&mut conn);

    all_names.unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_creditors() {
        let creditors = get_creditors();
        assert!(creditors.len() >= 0);
    }
}