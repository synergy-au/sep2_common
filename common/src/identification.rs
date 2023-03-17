// File auto-generated using xsd-parser-rs & IEEE 2030.5 sep-ordered-dep.xsd
// Types should eventually be put in a module corresponding to their package
use std::str::FromStr;
use xsd_macro_utils::{UtilsDefaultSerde, UtilsTupleIo};
use xsd_parser::generator::validator::Validate;
use yaserde_derive::{YaDeserialize, YaSerialize};

// TODO Ethan: Temporary import all
use crate::primitives::*;
use crate::xsd::*;

// A resource is an addressable unit of information, either a collection (List)
// or instance of an object (identifiedObject, or simply, Resource)
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Resource {
    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for Resource {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Response {
    // The createdDateTime field contains the date and time when the
    // acknowledgement/status occurred in the client. The client will provide
    // the timestamp to ensure the proper time is captured in case the response
    // is delayed in reaching the server (server receipt time would not be the
    // same as the actual confirmation time). The time reported from the client
    // should be relative to the time server indicated by the
    // FunctionSetAssignment that also indicated the event resource; if no
    // FunctionSetAssignment exists, the time of the server where the event
    // resource was hosted.
    #[yaserde(rename = "createdDateTime")]
    pub created_date_time: Option<TimeType>,

    // Contains the LFDI of the device providing the response.
    #[yaserde(rename = "endDeviceLFDI")]
    pub end_device_lfdi: HexBinary160,

    // The status field contains the acknowledgement or status. Each event type
    // (DRLC, DER, Price, or Text) can return different status information (e.g.
    // an Acknowledge will be returned for a Price event where a DRLC event can
    // return Event Received, Event Started, and Event Completed). The Status
    // field value definitions are defined in Table 27: Response Types by
    // Function Set.
    #[yaserde(rename = "status")]
    pub status: Option<Uint8>,

    // The subject field provides a method to match the response with the
    // originating event. It is populated with the mRID of the original object.
    #[yaserde(rename = "subject")]
    pub subject: Mridtype,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for Response {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct List {
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

impl Validate for List {}

// Links provide a reference, via URI, to another resource.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Link {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for Link {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct IdentifiedObject {
    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for IdentifiedObject {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct RespondableResource {
    // A reference to the response resource address (URI). Required on a
    // response to a GET if responseRequired is "true".
    #[yaserde(attribute, rename = "replyTo")]
    pub reply_to: Option<String>,

    // Indicates whether or not a response is required upon receipt, creation or
    // update of this resource. Responses shall be posted to the collection
    // specified in "replyTo".
    // If the resource has a deviceCategory field, devices that match one or
    // more of the device types indicated in deviceCategory SHALL respond
    // according to the rules listed below. If the category does not match, the
    // device SHALL NOT respond. If the resource does not have a deviceCategory
    // field, a device receiving the resource SHALL respond according to the
    // rules listed below.
    // Value encoded as hex according to the following bit assignments, any
    // combination is possible.
    // See Table 27 for the list of appropriate Response status codes to be sent
    // for these purposes.
    // 0 - End device shall indicate that message was received
    // 1 - End device shall indicate specific response.
    // 2 - End user / customer response is required.
    // All other values reserved.
    #[yaserde(attribute, rename = "responseRequired")]
    pub response_required: Option<HexBinary8>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for RespondableResource {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct RespondableIdentifiedObject {
    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // A reference to the response resource address (URI). Required on a
    // response to a GET if responseRequired is "true".
    #[yaserde(attribute, rename = "replyTo")]
    pub reply_to: Option<String>,

    // Indicates whether or not a response is required upon receipt, creation or
    // update of this resource. Responses shall be posted to the collection
    // specified in "replyTo".
    // If the resource has a deviceCategory field, devices that match one or
    // more of the device types indicated in deviceCategory SHALL respond
    // according to the rules listed below. If the category does not match, the
    // device SHALL NOT respond. If the resource does not have a deviceCategory
    // field, a device receiving the resource SHALL respond according to the
    // rules listed below.
    // Value encoded as hex according to the following bit assignments, any
    // combination is possible.
    // See Table 27 for the list of appropriate Response status codes to be sent
    // for these purposes.
    // 0 - End device shall indicate that message was received
    // 1 - End device shall indicate specific response.
    // 2 - End user / customer response is required.
    // All other values reserved.
    #[yaserde(attribute, rename = "responseRequired")]
    pub response_required: Option<HexBinary8>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for RespondableIdentifiedObject {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct RespondableSubscribableIdentifiedObject {
    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

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

    // A reference to the response resource address (URI). Required on a
    // response to a GET if responseRequired is "true".
    #[yaserde(attribute, rename = "replyTo")]
    pub reply_to: Option<String>,

    // Indicates whether or not a response is required upon receipt, creation or
    // update of this resource. Responses shall be posted to the collection
    // specified in "replyTo".
    // If the resource has a deviceCategory field, devices that match one or
    // more of the device types indicated in deviceCategory SHALL respond
    // according to the rules listed below. If the category does not match, the
    // device SHALL NOT respond. If the resource does not have a deviceCategory
    // field, a device receiving the resource SHALL respond according to the
    // rules listed below.
    // Value encoded as hex according to the following bit assignments, any
    // combination is possible.
    // See Table 27 for the list of appropriate Response status codes to be sent
    // for these purposes.
    // 0 - End device shall indicate that message was received
    // 1 - End device shall indicate specific response.
    // 2 - End user / customer response is required.
    // All other values reserved.
    #[yaserde(attribute, rename = "responseRequired")]
    pub response_required: Option<HexBinary8>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for RespondableSubscribableIdentifiedObject {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct SubscribableResource {
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

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct SubscribableList {
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

impl Validate for SubscribableList {}
