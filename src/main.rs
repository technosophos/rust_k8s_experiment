#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged, rename_all="camelCase")]
enum Object {
    #[serde(rename_all="camelCase")]
    Pod{
        api_version: String,
        kind: String,
        metadata: ObjectMeta,
        spec: Spec,
    },
    #[serde(rename_all="camelCase")]
    Secret{
        api_version: String,
        kind: String,
        metadata: ObjectMeta,
        data: HashMap<String, String>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct ObjectMeta {
    name: String,
    labels: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
struct Spec {
 restart_policy: String,
}

fn main() {
    let pod_reader = std::fs::File::open("pod.yaml").unwrap();
    let object_one: Object= serde_yaml::from_reader(pod_reader).unwrap();
    inspect_object(&object_one);

    let secret_reader = std::fs::File::open("secret.yaml").unwrap();
    let object_two: Object = serde_yaml::from_reader(secret_reader).unwrap();
    inspect_object(&object_two);
    println!("{:?}", object_two);
}

fn inspect_object(obj: &Object) -> () {
    let _ = match obj {
        Object::Pod{ metadata, .. } => {
            println!("I am a pod named {}!", metadata.name);
        }
        Object::Secret{metadata, ..} => {
            println!("I am a secret named {}", metadata.name);
        }
    };
}
