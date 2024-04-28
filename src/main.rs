use base64::prelude::{Engine as _, BASE64_STANDARD as b64};
use std::io;
use std::io::prelude::*;

use aws_config::BehaviorVersion;
use aws_sdk_ec2 as ec2;
use ec2::types::InstanceType;

#[::tokio::main]
async fn main() -> Result<(), ec2::Error> {
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let ec2_client = aws_sdk_ec2::Client::new(&config);

    //TODO make configurable
    let image_id = "ami-019a292cfb114a776";

    //TODO make configurable
    let instance_type = InstanceType::T3Nano;

    //TODO make configurable
    let docker_image = "nginx:latest";

    //TODO implement
    let _key_name = "";

    //TODO security groups
    let security_group = ec2_client.create_security_group()
        .description("Temporary group allowing traffic on ssh, whatever docker port is used, and all outbound")
        .group_name("showers-rs")
        .send().await?;
    let security_group_id = security_group.group_id.unwrap();
    ec2_client
        .authorize_security_group_ingress()
        .group_id(&security_group_id)
        .ip_permissions(
            ec2::types::IpPermission::builder()
                .ip_protocol("-1")
                .ip_ranges(ec2::types::IpRange::builder().cidr_ip("0.0.0.0/0").build())
                .from_port(3000)
                .to_port(3000)
                .build(),
        )
        .ip_permissions(
            ec2::types::IpPermission::builder()
                .ip_protocol("-1")
                .ip_ranges(ec2::types::IpRange::builder().cidr_ip("0.0.0.0/0").build())
                .from_port(22)
                .to_port(22)
                .build(),
        )
        .send()
        .await?;

    let res = ec2_client
        .run_instances()
        .image_id(image_id)
        .min_count(1)
        .max_count(1)
        .instance_type(instance_type)
        .user_data(b64.encode(format!(
            "#!/bin/bash\ndocker run -p 3000:80 {}",
            docker_image
        )))
        .security_group_ids(&security_group_id)
        .send()
        .await?;
    let instances = res.instances;
    let instance = instances.unwrap()[0].clone();
    let instance_id = instance.instance_id.unwrap();
    println!("Successfully created Instance.");
    pause("");

    ec2_client
        .terminate_instances()
        .instance_ids(&instance_id)
        .send()
        .await?;
    ec2_client
        .delete_security_group()
        .group_id(security_group_id)
        .send()
        .await?;
    Ok(())
}

fn pause(text: &str) {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "{}\nPress any key to continue", text).unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}
