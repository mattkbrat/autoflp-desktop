use crate::lib::database::models::{Account, Person};
use crate::lib::database::schema::account::id as account_id;
use crate::lib::database::schema::person::id as personId;
use crate::lib::database::schema::{account, person};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection};

pub fn get_account(
    person_id: Option<&String>,
    conn: &mut SqliteConnection,
) -> Option<(Person, Account)> {
    person_id?;

    let this_id = person_id.unwrap();

    if this_id.is_empty() {
        return None;
    }

    let mut selected_person = account::table
        .inner_join(person::table)
        .filter(personId.eq(this_id))
        .or_filter(account_id.eq(this_id))
        .select((Person::as_select(), Account::as_select()))
        .load::<(Person, Account)>(conn)
        .expect("Failed")
        .into_iter();

    selected_person.next()
}
