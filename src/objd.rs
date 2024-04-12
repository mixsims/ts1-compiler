use crate::iff;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ObjectDefinition {
    #[serde(rename = "@name")]
    pub chunk_label: String,
    #[serde(rename = "@id")]
    pub chunk_id: iff::ChunkId,
    #[serde(rename = "@version")]
    pub version: i32,
    #[serde(rename = "@initialstacksize")]
    pub initialstacksize: i16,
    #[serde(rename = "@basegraphic")]
    pub basegraphic: i16,
    #[serde(rename = "@numgraphics")]
    pub numgraphics: i16,
    #[serde(rename = "@maintreeid")]
    pub maintreeid: i16,
    #[serde(rename = "@gardeningtreeid")]
    pub gardeningtreeid: i16,
    #[serde(rename = "@treetableid")]
    pub treetableid: i16,
    #[serde(rename = "@interactiongroup")]
    pub interactiongroup: i16,
    #[serde(rename = "@type")]
    pub object_type: i16,
    #[serde(rename = "@masterid")]
    pub masterid: i16,
    #[serde(rename = "@subindex")]
    pub subindex: i16,
    #[serde(rename = "@washhandstreeid")]
    pub washhandstreeid: i16,
    #[serde(rename = "@animtableid")]
    pub animtableid: i16,
    #[serde(rename = "@guid")]
    pub guid: i32,
    #[serde(rename = "@disabled")]
    pub disabled: i16,
    #[serde(rename = "@portaltreeid")]
    pub portaltreeid: i16,
    #[serde(rename = "@price")]
    pub price: i16,
    #[serde(rename = "@bodystringsid")]
    pub bodystringsid: i16,
    #[serde(rename = "@slotsid")]
    pub slotsid: i16,
    #[serde(rename = "@allowintersectiontreeid")]
    pub allowintersectiontreeid: i16,
    #[serde(rename = "@usesfntable")]
    pub usesfntable: i16,
    #[serde(rename = "@unused4")]
    pub unused4: i16,
    #[serde(rename = "@preptreeid")]
    pub preptreeid: i16,
    #[serde(rename = "@cooktreeid")]
    pub cooktreeid: i16,
    #[serde(rename = "@surfacetreeid")]
    pub surfacetreeid: i16,
    #[serde(rename = "@disposetreeid")]
    pub disposetreeid: i16,
    #[serde(rename = "@foodtreeid")]
    pub foodtreeid: i16,
    #[serde(rename = "@pickupfromslottreeid")]
    pub pickupfromslottreeid: i16,
    #[serde(rename = "@washdishtreeid")]
    pub washdishtreeid: i16,
    #[serde(rename = "@eatingsurfacetreeid")]
    pub eatingsurfacetreeid: i16,
    #[serde(rename = "@sittreeid")]
    pub sittreeid: i16,
    #[serde(rename = "@standtreeid")]
    pub standtreeid: i16,
    #[serde(rename = "@saleprice")]
    pub saleprice: i16,
    #[serde(rename = "@initialdepreciation")]
    pub initialdepreciation: i16,
    #[serde(rename = "@dailydepreciation")]
    pub dailydepreciation: i16,
    #[serde(rename = "@selfdepreciating")]
    pub selfdepreciating: i16,
    #[serde(rename = "@depreciationlimit")]
    pub depreciationlimit: i16,
    #[serde(rename = "@roomflags")]
    pub roomflags: i16,
    #[serde(rename = "@functionflags")]
    pub functionflags: i16,
    #[serde(rename = "@catalogid")]
    pub catalogid: i16,
    #[serde(rename = "@globalsimulationobject")]
    pub globalsimulationobject: i16,
    #[serde(rename = "@inittreeid")]
    pub inittreeid: i16,
    #[serde(rename = "@placementtreeid")]
    pub placementtreeid: i16,
    #[serde(rename = "@userpickuptreeid")]
    pub userpickuptreeid: i16,
    #[serde(rename = "@wallstyle")]
    pub wallstyle: i16,
    #[serde(rename = "@loadtreeid")]
    pub loadtreeid: i16,
    #[serde(rename = "@userplacementtreeid")]
    pub userplacementtreeid: i16,
    #[serde(rename = "@objectversion")]
    pub objectversion: i16,
    #[serde(rename = "@roomchangedtreeid")]
    pub roomchangedtreeid: i16,
    #[serde(rename = "@motiveeffectsid")]
    pub motiveeffectsid: i16,
    #[serde(rename = "@cleanuptreeid")]
    pub cleanuptreeid: i16,
    #[serde(rename = "@levelinforequesttreeid")]
    pub levelinforequesttreeid: i16,
    #[serde(rename = "@catalogpopupid")]
    pub catalogpopupid: i16,
    #[serde(rename = "@servingsurfacetreeid")]
    pub servingsurfacetreeid: i16,
    #[serde(rename = "@leveloffset")]
    pub leveloffset: i16,
    #[serde(rename = "@shadow")]
    pub shadow: i16,
    #[serde(rename = "@numattributes")]
    pub numattributes: i16,
    #[serde(rename = "@cleantreeid")]
    pub cleantreeid: i16,
    #[serde(rename = "@queueskippedtreeid")]
    pub queueskippedtreeid: i16,
    #[serde(rename = "@frontfacedirection")]
    pub frontfacedirection: i16,
    #[serde(rename = "@walladjacencychangedtreeid")]
    pub walladjacencychangedtreeid: i16,
    #[serde(rename = "@leadobject")]
    pub leadobject: i16,
    #[serde(rename = "@dynspritebaseid")]
    pub dynspritebaseid: i16,
    #[serde(rename = "@numdynsprites")]
    pub numdynsprites: i16,
    #[serde(rename = "@chairentryflags")]
    pub chairentryflags: i16,
    #[serde(rename = "@tilewidth")]
    pub tilewidth: i16,
    #[serde(rename = "@suitnotcopyable")]
    pub suitnotcopyable: i16,
    #[serde(rename = "@buildmodetype")]
    pub buildmodetype: i16,
    #[serde(rename = "@originalguid")]
    pub originalguid: i32,
    #[serde(rename = "@originalsuitguid")]
    pub originalsuitguid: i32,
    #[serde(rename = "@pickuptreeid")]
    pub pickuptreeid: i16,
    #[serde(rename = "@thumbnailgraphicindex")]
    pub thumbnailgraphicindex: i16,
    #[serde(rename = "@shadowflags")]
    pub shadowflags: i16,
    #[serde(rename = "@footprintinsetmask")]
    pub footprintinsetmask: i16,
    #[serde(rename = "@mtadjupdatetreeid")]
    pub mtadjupdatetreeid: i16,
    #[serde(rename = "@shadowbrightness")]
    pub shadowbrightness: i16,
    #[serde(rename = "@repairtreeid")]
    pub repairtreeid: i16,
    #[serde(rename = "@customwallstyleid")]
    pub customwallstyleid: i16,
    #[serde(rename = "@ratinghunger")]
    pub ratinghunger: i16,
    #[serde(rename = "@ratingcomfort")]
    pub ratingcomfort: i16,
    #[serde(rename = "@ratinghygiene")]
    pub ratinghygiene: i16,
    #[serde(rename = "@ratingbladder")]
    pub ratingbladder: i16,
    #[serde(rename = "@ratingenergy")]
    pub ratingenergy: i16,
    #[serde(rename = "@ratingfun")]
    pub ratingfun: i16,
    #[serde(rename = "@ratingroom")]
    pub ratingroom: i16,
    #[serde(rename = "@ratingskillflags")]
    pub ratingskillflags: i16,
    #[serde(rename = "@numtypeattributes")]
    pub numtypeattributes: i16,
    #[serde(rename = "@miscflags")]
    pub miscflags: i16,
    #[serde(rename = "@typeattrguid")]
    pub typeattrguid: i32,
    #[serde(rename = "@functionsubsort")]
    pub functionsubsort: i16,
    #[serde(rename = "@downtownsort")]
    pub downtownsort: i16,
    #[serde(rename = "@keepbuying")]
    pub keepbuying: i16,
    #[serde(rename = "@vacationsort")]
    pub vacationsort: i16,
    #[serde(rename = "@resetlotaction")]
    pub resetlotaction: i16,
    #[serde(rename = "@communitysort")]
    pub communitysort: i16,
    #[serde(rename = "@dreamflags")]
    pub dreamflags: i16,
    #[serde(rename = "@renderflags")]
    pub renderflags: i16,
    #[serde(rename = "@unused8")]
    pub unused8: i16,
    #[serde(rename = "@unused9")]
    pub unused9: i16,
    #[serde(rename = "@unused10")]
    pub unused10: i16,
    #[serde(rename = "@unused11")]
    pub unused11: i16,
    #[serde(rename = "@unused12")]
    pub unused12: i16,
    #[serde(rename = "@unused13")]
    pub unused13: i16,
}
