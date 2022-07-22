pub use chrono::{DateTime, NaiveDate, Utc};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct AccessToken {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub access_token: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub refresh_token: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub token_type: String,
    #[serde(default)]
    pub expires_in: i64,
    #[serde(default)]
    pub x_refresh_token_expires_in: i64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct AccountBasedExpenseLineDetail {
    pub account_ref: NtRef,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub billable_status: String,
    pub tax_code_ref: NtRef,
}

// update `impl std::fmt::Display for Addr` below if any fields are added
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Addr {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub city: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub country: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub country_sub_division_code: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub line1: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub postal_code: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct AttachableRef {
    pub entity_ref: NtRef,
    pub include_on_send: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Attachment {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attachable_ref: Vec<AttachableRef>,
    #[serde(default, skip_serializing_if = "String::is_empty", rename = "domain")]
    pub domain: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub file_access_uri: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub file_name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,

    pub meta_data: MetaData,
    #[serde(default)]
    pub size: i64,
    #[serde(default, rename = "sparse")]
    pub sparse: bool,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub sync_token: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub temp_download_uri: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Any {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        rename = "declaredType"
    )]
    pub declared_type: String,
    #[serde(default, rename = "globalScope")]
    pub global_scope: bool,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(default)]
    pub nil: bool,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub scope: String,
    #[serde(default, rename = "typeSubstituted")]
    pub type_substituted: bool,
    #[serde(default)]
    pub value: NtRef,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BillPayment {
    #[serde(default)]
    pub check_payment: Payment,
    #[serde(default)]
    pub credit_card_payment: Payment,
    #[serde(default)]
    pub currency_ref: NtRef,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub doc_number: String,
    #[serde(default, skip_serializing_if = "String::is_empty", rename = "domain")]
    pub domain: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub line: Vec<Line>,

    pub meta_data: MetaData,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub pay_type: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub private_note: String,
    #[serde(default, rename = "sparse")]
    pub sparse: bool,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub sync_token: String,
    #[serde(default)]
    pub total_amt: f32,

    pub txn_date: NaiveDate,
    #[serde(default)]
    pub vendor_ref: NtRef,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CompanyInfo {
    #[serde(default)]
    pub company_addr: Addr,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub company_name: String,

    pub company_start_date: NaiveDate,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub country: String,
    #[serde(default)]
    pub customer_communication_addr: Addr,
    #[serde(default, skip_serializing_if = "String::is_empty", rename = "domain")]
    pub domain: String,
    #[serde(default)]
    pub email: Email,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fiscal_year_start_month: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(default)]
    pub legal_addr: Addr,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub legal_name: String,

    pub meta_data: MetaData,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub name_value: Vec<NtRef>,
    #[serde(default)]
    pub primary_phone: PrimaryPhone,
    #[serde(default, rename = "sparse")]
    pub sparse: bool,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub supported_languages: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub sync_token: String,
    #[serde(default)]
    pub web_addr: WebAddr,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Email {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub address: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Item {
    /// If true, the object is currently enabled for use by QuickBooks.
    pub active: Option<bool>,

    /// Reference to the Inventory Asset account that tracks the current value of the inventory.
    /// If the same account is used for all inventory items, the current balance of this account will represent the current total value of the inventory.
    /// Query the Account name list resource to determine the appropriate Account object for this reference.
    /// Use `Account.id` and `Account.name` from that object for `AssetAccountRef.value` and `AssetAccountRef.name`, respectively.
    ///
    /// Required for Inventory item types.
    pub asset_account_ref: Option<NtRef>,

    /// Description of the item.
    ///
    /// * max character: maximum of 4000 chars
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Documentation unavailable.
    #[serde(default, skip_serializing_if = "String::is_empty", rename = "domain")]
    pub domain: String,

    /// Reference to the expense account used to pay the vendor for this item.
    /// Must be an account with account type of Cost of Goods Sold.
    /// Query the Account name list resource to determine the appropriate Account object for this reference.
    /// Use `Account.id` and `Account.name` from that object for `ExpenseAccountRef.value` and `ExpenseAccountRef.name`, respectively.
    ///
    /// For France locales:
    /// * This is an optional field.
    /// * This is the purchase account id, If not provided it defaults to the default purchase account: 605100 and 601100 are the default expense accounts used for Service and Product type of item, respectively.
    ///
    /// Required for Inventory, NonInventory, and Service item types
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expense_account_ref: Option<NtRef>,

    /// Fully qualified name of the entity.
    /// The fully qualified name prepends the topmost parent, followed by each sub element separated by colons.
    /// Takes the form of Item:SubItem.
    /// Returned from an existing object and not input on a new object.
    /// Limited to 5 levels.
    ///
    /// * filterable
    /// * read only
    /// * system defined
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fully_qualified_name: String,

    /// Unique Identifier for an Intuit entity (object).
    ///
    /// Required for the update operation.
    ///
    /// * filterable
    /// * read only
    /// * sortable
    /// * system defined
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,

    #[serde(default)]
    pub income_account_ref: NtRef,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub inv_start_date: String,

    /// Classification that specifies the use of this item.
    /// Available when endpoint is evoked with the minorversion=3 query parameter.
    /// Read-only after object is created.
    /// Valid values include: Product and Service.
    ///
    /// Applicable for France companies only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_category_type: Option<String>,

    #[serde(default, skip_serializing_if = "String::is_empty", rename = "Type")]
    pub item_type: String,

    #[serde(default)]
    pub level: i64,

    pub meta_data: MetaData,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(default)]
    pub parent_ref: NtRef,

    #[serde(default)]
    pub purchase_cost: f32,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub purchase_desc: String,

    #[serde(default)]
    pub qty_on_hand: i64,

    pub sku: Option<String>,

    #[serde(default, rename = "sparse")]
    pub sparse: bool,

    #[serde(default)]
    pub sub_item: bool,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub sync_token: String,

    #[serde(default)]
    pub taxable: bool,

    #[serde(default)]
    pub track_qty_on_hand: bool,

    #[serde(default)]
    pub unit_price: f32,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Line {
    #[serde(default)]
    pub account_based_expense_line_detail: AccountBasedExpenseLineDetail,
    #[serde(default)]
    pub amount: f32,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub detail_type: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub linked_txn: Vec<LinkedTxn>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LinkedTxn {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub txn_id: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub txn_type: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct MetaData {
    pub create_time: DateTime<Utc>,
    pub last_updated_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Serialize)]
pub struct NtRef {
    #[serde(default, skip_serializing_if = "String::is_empty", rename = "type")]
    pub entity_ref_type: String,
    #[serde(default, skip_serializing_if = "String::is_empty", alias = "Name")]
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty", alias = "Value")]
    pub value: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Payment {
    #[serde(default)]
    pub bank_account_ref: NtRef,
    #[serde(default, rename = "CCAccountRef")]
    pub cc_account_ref: NtRef,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub print_status: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PrimaryPhone {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub free_form_number: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Purchase {
    #[serde(default)]
    pub account_ref: NtRef,
    #[serde(default)]
    pub credit: bool,
    #[serde(default)]
    pub currency_ref: NtRef,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub doc_number: String,
    #[serde(default, skip_serializing_if = "String::is_empty", rename = "domain")]
    pub domain: String,
    #[serde(default)]
    pub entity_ref: NtRef,

    pub id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub line: Vec<Line>,

    pub meta_data: MetaData,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub payment_type: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub private_note: String,
    #[serde(default)]
    pub purchase_ex: PurchaseEx,
    #[serde(default, rename = "sparse")]
    pub sparse: bool,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub sync_token: String,
    #[serde(default)]
    pub total_amt: f32,

    pub txn_date: NaiveDate,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PurchaseEx {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub any: Vec<Any>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct QueryResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attachable: Vec<Attachment>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bill_payment: Vec<BillPayment>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub company_info: Vec<CompanyInfo>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub item: Vec<Item>,
    #[serde(default, rename = "maxResults")]
    pub max_results: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub purchase: Vec<Purchase>,
    #[serde(default, rename = "startPosition")]
    pub start_position: i64,
    #[serde(default, rename = "totalCount")]
    pub total_count: i64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Response {
    #[serde(default)]
    pub query_response: QueryResponse,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct WebAddr {}

impl std::fmt::Display for Addr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{street_addr}, {city}, {country_sub_division_code}, {country} {postal_code}",
            street_addr = self.line1,
            city = self.city,
            country_sub_division_code = self.country_sub_division_code,
            country = self.country,
            postal_code = self.postal_code
        )
    }
}
