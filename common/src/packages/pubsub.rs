use anyhow::anyhow;
use anyhow::Result;
use xml::EventReader;
// File auto-generated using xsd-parser-rs & IEEE 2030.5 sep-ordered-dep.xsd
use xsd_parser::generator::validator::Validate;
use yaserde_derive::{YaDeserialize, YaSerialize};

// TODO Ethan: Temporary import all
use crate::packages::primitives::*;

use super::traits::*;

// Indicates a condition that must be satisfied for the Notification to be
// triggered.
#[derive(Default, PartialEq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Condition {
    // 0 = Reading value
    // 1-255 = Reserved
    #[yaserde(rename = "attributeIdentifier")]
    pub attribute_identifier: Uint8,

    // The value of the lower threshold
    #[yaserde(rename = "lowerThreshold")]
    pub lower_threshold: Int48,

    // The value of the upper threshold
    #[yaserde(rename = "upperThreshold")]
    pub upper_threshold: Int48,
}

impl Validate for Condition {}

#[derive(Default, PartialEq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct SubscriptionBase {
    // The resource for which the subscription applies. Query string parameters
    // SHALL NOT be specified when subscribing to list resources. Should a query
    // string parameter be specified, servers SHALL ignore them.
    #[yaserde(rename = "subscribedResource")]
    pub subscribed_resource: String,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl SEResource for SubscriptionBase {}
impl Validate for SubscriptionBase {}

#[derive(Default, PartialEq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Subscription {
    #[yaserde(rename = "Condition")]
    pub condition: Option<Condition>,

    // 0 - application/sep+xml
    // 1 - application/sep-exi
    // 2-255 - reserved
    #[yaserde(rename = "encoding")]
    pub encoding: Uint8,

    // Contains the preferred schema and extensibility level indication such as
    // "+S1"
    #[yaserde(rename = "level")]
    pub level: String16,

    // This element is used to indicate the maximum number of list items that
    // should be included in a notification when the subscribed resource
    // changes. This limit is meant to be functionally equivalent to the
    // ‘limit’ query string parameter, but applies to both list resources as
    // well as other resources. For list resources, if a limit of ‘0’ is
    // specified, then notifications SHALL contain a list resource with
    // results=’0’ (equivalent to a simple change notification). For list
    // resources, if a limit greater than ‘0’ is specified, then
    // notifications SHALL contain a list resource with results equal to the
    // limit specified (or less, should the list contain fewer items than the
    // limit specified or should the server be unable to provide the requested
    // number of items for any reason) and follow the same rules for list
    // resources (e.g., ordering). For non-list resources, if a limit of ‘0’
    // is specified, then notifications SHALL NOT contain a resource
    // representation (equivalent to a simple change notification). For non-list
    // resources, if a limit greater than ‘0’ is specified, then
    // notifications SHALL contain the representation of the changed resource.
    #[yaserde(rename = "limit")]
    pub limit: Uint32,

    // The resource to which to post the notifications about the requested
    // subscribed resource. Because this URI will exist on a server other than
    // the one being POSTed to, this attribute SHALL be a fully-qualified
    // absolute URI, not a relative reference.
    #[yaserde(rename = "notificationURI")]
    pub notification_uri: String,

    // The resource for which the subscription applies. Query string parameters
    // SHALL NOT be specified when subscribing to list resources. Should a query
    // string parameter be specified, servers SHALL ignore them.
    #[yaserde(rename = "subscribedResource")]
    pub subscribed_resource: String,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl SESubscriptionBase for Subscription {}
impl SEResource for Subscription {}
impl Validate for Subscription {}

#[derive(Default, PartialEq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct SubscriptionList {
    #[yaserde(rename = "Subscription")]
    pub subscription: Vec<Subscription>,

    // The default polling rate for this function set (this resource and all
    // resources below), in seconds. If not specified, a default of 900 seconds
    // (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    // this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

    // The number specifying "all" of the items in the list. Required on a
    // response to a GET, ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl SEList for SubscriptionList {}
impl SEResource for SubscriptionList {}
impl Validate for SubscriptionList {}

#[derive(Default, PartialEq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[yaserde(namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance")]
pub struct Notification<T: SEResource> {
    // The new location of the resource, if moved. This attribute SHALL be a
    // fully-qualified absolute URI, not a relative reference.
    #[yaserde(rename = "newResourceURI")]
    pub new_resource_uri: Option<String>,

    #[yaserde(rename = "Resource")]
    #[yaserde(generic)]
    pub resource: Option<T>,

    // 0 = Default Status
    // 1 = Subscription canceled, no additional information
    // 2 = Subscription canceled, resource moved
    // 3 = Subscription canceled, resource definition changed (e.g., a new
    // version of IEEE 2030.5)
    // 4 = Subscription canceled, resource deleted
    // All other values reserved.
    #[yaserde(rename = "status")]
    pub status: Uint8,

    // The subscription from which this notification was triggered. This
    // attribute SHALL be a fully-qualified absolute URI, not a relative
    // reference.
    #[yaserde(rename = "subscriptionURI")]
    pub subscription_uri: String,

    // The resource for which the subscription applies. Query string parameters
    // SHALL NOT be specified when subscribing to list resources. Should a query
    // string parameter be specified, servers SHALL ignore them.
    #[yaserde(rename = "subscribedResource")]
    pub subscribed_resource: String,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

pub fn get_notif_type(notif_xml: &str) -> Result<String> {
    let parser = EventReader::new(notif_xml.as_bytes());
    for event in parser {
        match event? {
            xml::reader::XmlEvent::StartElement {
                name, attributes, ..
            } if name.local_name == "Resource" => {
                return Ok(attributes
                    .iter()
                    .find(|a| a.name.local_name == "type")
                    .ok_or(anyhow!("Notification did not include a type annotation"))?
                    .value
                    .clone())
            }
            _ => (),
        }
    }
    Err(anyhow!("Notification did not contain an inner resource"))
}

impl<T: SEResource> SESubscriptionBase for Notification<T> {}
impl<T: SEResource> SEResource for Notification<T> {}
impl<T: SEResource> Validate for Notification<T> {}

#[derive(Default, PartialEq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[yaserde(namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance")]
pub struct NotificationList<T: SEResource> {
    #[yaserde(rename = "Notification")]
    pub notification: Vec<Notification<T>>,

    // The number specifying "all" of the items in the list. Required on a
    // response to a GET, ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl<T: SEResource> SEList for NotificationList<T> {}
impl<T: SEResource> SEResource for NotificationList<T> {}
impl<T: SEResource> Validate for NotificationList<T> {}
