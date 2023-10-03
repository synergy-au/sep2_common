use crate::traits::{
    SEFunctionSetAssignmentsBase, SEList, SEResource, SESubscribableList, SESubscribableResource,
    Validate,
};
use sep2_common_derive::{
    SEFunctionSetAssignmentsBase, SEList, SEResource, SESubscribableList, SESubscribableResource,
};

use yaserde::{YaDeserialize, YaSerialize};

use super::{
    identification::{Link, ListLink},
    primitives::{String32, Uint32},
    types::{MRIDType, SubscribableType, VersionType},
};

#[derive(
    Default,
    PartialEq,
    Eq,
    Debug,
    Clone,
    YaSerialize,
    YaDeserialize,
    SEResource,
    SEFunctionSetAssignmentsBase,
)]
#[yaserde(rename = "FunctionSetAssignmentsBase")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct FunctionSetAssignmentsBase {
    #[yaserde(rename = "CustomerAccountListLink")]
    pub customer_account_list_link: Option<ListLink>,

    #[yaserde(rename = "DemandResponseProgramListLink")]
    pub demand_response_program_list_link: Option<ListLink>,

    #[yaserde(rename = "DERProgramListLink")]
    pub der_program_list_link: Option<ListLink>,

    #[yaserde(rename = "FileListLink")]
    pub file_list_link: Option<ListLink>,

    #[yaserde(rename = "MessagingProgramListLink")]
    pub messaging_program_list_link: Option<ListLink>,

    #[yaserde(rename = "PrepaymentListLink")]
    pub prepayment_list_link: Option<ListLink>,

    #[yaserde(rename = "ResponseSetListLink")]
    pub response_set_list_link: Option<ListLink>,

    #[yaserde(rename = "TariffProfileListLink")]
    pub tariff_profile_list_link: Option<ListLink>,

    #[yaserde(rename = "TimeLink")]
    pub time_link: Option<Link>,

    #[yaserde(rename = "UsagePointListLink")]
    pub usage_point_list_link: Option<ListLink>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for FunctionSetAssignmentsBase {}

#[derive(
    Default,
    PartialEq,
    Eq,
    Debug,
    Clone,
    YaSerialize,
    YaDeserialize,
    SEFunctionSetAssignmentsBase,
    SEResource,
)]
#[yaserde(rename = "FunctionSetAssignments")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct FunctionSetAssignments {
    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub mrid: MRIDType,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    #[yaserde(rename = "CustomerAccountListLink")]
    pub customer_account_list_link: Option<ListLink>,

    #[yaserde(rename = "DemandResponseProgramListLink")]
    pub demand_response_program_list_link: Option<ListLink>,

    #[yaserde(rename = "DERProgramListLink")]
    pub der_program_list_link: Option<ListLink>,

    #[yaserde(rename = "FileListLink")]
    pub file_list_link: Option<ListLink>,

    #[yaserde(rename = "MessagingProgramListLink")]
    pub messaging_program_list_link: Option<ListLink>,

    #[yaserde(rename = "PrepaymentListLink")]
    pub prepayment_list_link: Option<ListLink>,

    #[yaserde(rename = "ResponseSetListLink")]
    pub response_set_list_link: Option<ListLink>,

    #[yaserde(rename = "TariffProfileListLink")]
    pub tariff_profile_list_link: Option<ListLink>,

    #[yaserde(rename = "TimeLink")]
    pub time_link: Option<Link>,

    #[yaserde(rename = "UsagePointListLink")]
    pub usage_point_list_link: Option<ListLink>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl PartialOrd for FunctionSetAssignments {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FunctionSetAssignments {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary Key - mRID (descending)
        self.mrid.cmp(&other.mrid).reverse()
    }
}

impl Validate for FunctionSetAssignments {}

#[derive(
    Default,
    PartialEq,
    Eq,
    Debug,
    Clone,
    YaSerialize,
    YaDeserialize,
    SESubscribableList,
    SEList,
    SESubscribableResource,
    SEResource,
)]
#[yaserde(rename = "FunctionSetAssignmentsList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct FunctionSetAssignmentsList {
    #[yaserde(rename = "FunctionSetAssignments")]
    pub function_set_assignments: Vec<FunctionSetAssignments>,

    // The default polling rate for this function set (this resource and all
    // resources below), in seconds. If not specified, a default of 900 seconds
    // (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    // this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

    // The number specifying "all" of the items in the list. Required on GET,
    // ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for FunctionSetAssignmentsList {}
