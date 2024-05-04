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
    parameters: Option<HashMap<String, Parameter>>,
    resources: HashMap<String, Resource>,
    outputs: Option<HashMap<String, Output>>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
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

#[derive(Deserialize, Debug, PartialEq, Eq)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(untagged)]
pub enum Value {
    String(String),
    Number(i64),
    #[serde(rename_all = "PascalCase")]
    Ref {
        r#ref: String,
    },
    GetAtt {
        #[serde(rename = "Fn::GetAtt")]
        get_att: Vec<String>,
    },
    Join {
        #[serde(rename = "Fn::Join")]
        join: (String, Vec<Value>),
    },
    Sub {
        #[serde(rename = "Fn::Sub")]
        sub: String,
    },
}
