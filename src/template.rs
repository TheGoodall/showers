use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Template {
    #[serde(rename = "AWSTemplateFormatVersion")]
    aws_template_format_version: Option<String>,
    metadata: Option<HashMap<String, String>>,
    description: Option<String>,
    mappings: Option<Mapping>,
    parameters: Option<HashMap<String, Parameter<Box<dyn Value>>>>,
    resources: HashMap<String, Resource>,
    outputs: Option<HashMap<String, Output>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
#[serde(tag = "Type")]
pub enum Resource {
    #[serde(rename = "AWS::EC2::Instance")]
    Ec2(ResourceContainer<Ec2>),
    #[serde(rename = "AWS::EC2::VPC")]
    Vpc(ResourceContainer<Vpc>),
    #[serde(rename = "AWS::SNS::Topic")]
    Topic,
    #[serde(rename = "AWS::AutoScaling::AutoScalingGroup")]
    AutoScalingGroup,
    #[serde(rename = "AWS::AutoScaling::LaunchConfiguration")]
    LaunchConfiguration,
    #[serde(rename = "AWS::AutoScaling::ScalingPolicy")]
    ScalingPolicy,
    #[serde(rename = "AWS::CloudWatch::Alarm")]
    Alarm,
    #[serde(rename = "AWS::ElasticLoadBalancing::LoadBalancer")]
    LoadBalancer,
    #[serde(rename = "AWS::EC2::SecurityGroup")]
    SecurityGroup(ResourceContainer<SecurityGroup>),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct ResourceContainer<T> {
    properties: T,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Mapping {
    #[serde(flatten)]
    entries: HashMap<String, MappingEntry>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(untagged)]
pub enum MappingEntry {
    String(String),
    List(Vec<String>),
    Mapping(Mapping),
}
#[typetag::serde(tag = "Type")]
trait Value: core::fmt::Debug + std::cmp::Eq {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
struct Ref {
    r#ref: String,
}
#[typetag::serde]
impl Value for Ref {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
struct GetAtt {
    get_att: Vec<String>,
}
#[typetag::serde]
impl Value for GetAtt {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct Join {
    #[serde(rename = "Fn::Join")]
    join: (String, Vec<Box<dyn Value>>),
}
#[typetag::serde]
impl Value for Join {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct Sub {
    #[serde(rename = "Fn::Sub")]
    sub: String,
}
#[typetag::serde]
impl Value for Sub {}

#[typetag::serde]
impl Value for String {}
#[typetag::serde]
impl Value for i64 {}

// #[typetag::serde]
// impl Value for Box<dyn Value + '_> {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Parameter<T: Value> {
    allowed_values: Vec<Box<T>>,
    default: Box<T>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Output {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Ec2 {}

pub struct Vpc {}

pub struct SecurityGroup {}
