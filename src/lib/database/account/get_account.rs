use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection};
use crate::lib::database::models::{Account, Person};
use crate::lib::database::schema::{account, person};
use crate::lib::database::schema::person::id as personId;

pub fn get_account(person_id: Option<&String>, conn: &mut SqliteConnection) -> Option<(Person, Account)> {
    if person_id.is_none() {
        return None;
    };

    let this_id = person_id.unwrap();

    if this_id.len() <= 0 {
        return None;
    }

    let selected_person = account::table
        .inner_join(person::table)
        .filter(personId.eq(this_id))
        .select((Person::as_select(), Account::as_select()))
        .load::<(Person, Account)>(conn);

    selected_person.unwrap().into_iter().nth(0)
}
