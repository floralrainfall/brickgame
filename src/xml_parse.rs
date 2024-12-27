use crate::instance::*;
use bevy::prelude::*;

use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::reader::Reader;
use std::str;

pub fn create_instances(mut world: &mut World) {
    let reader_file = Reader::from_file("place.rbxl");

    let mut reader = reader_file.unwrap();
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    let mut txt: String = String::from("");
    let mut instance = world.spawn(Instance::new("DataModel", None)).id(); // datamodel
    let mut tmp_vec3: Vec3 = Vec3::ZERO;
    let mut tmp_mat3: Mat3 = Mat3::IDENTITY;
    let mut target_name: String = String::from("");
    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => match e.name().as_ref() {
                // parse stuff
                b"Item" => {
                    println!("New on {:?} parent {:?}", e, instance);

                    if let Some(attrib) = e
                        .attributes()
                        .find(|v| v.as_ref().unwrap().key == QName(b"class"))
                    {
                        let unwrap = attrib.unwrap();
                        let str = String::from_utf8_lossy(&unwrap.value);

                        let inst = Instance::new(str.as_str(), Some(instance));
                        let new_instance = match str.as_str() {
                            "Part" => {
                                world.spawn((inst, Part::new(), Transform::from_xyz(0.0, 0.0, 0.0)))
                            }
                            _ => world.spawn(inst),
                        };
                        instance = new_instance.id();

                        info!("Class {}", str);
                    } else {
                        panic!("No class name");
                    }
                }
                b"Vector3" => {
                    if let Some(attrib) = e
                        .attributes()
                        .find(|v| v.as_ref().unwrap().key == QName(b"name"))
                    {
                        let unwrap = attrib.unwrap();
                        let str = String::from_utf8_lossy(&unwrap.value);

                        target_name = str.to_string();
                    } else {
                        panic!("No class name");
                    }

                    tmp_vec3 = Vec3::ZERO;
                }
                b"CoordinateFrame" => {
                    tmp_vec3 = Vec3::ZERO;
                    tmp_mat3 = Mat3::IDENTITY;

                    if let Some(attrib) = e
                        .attributes()
                        .find(|v| v.as_ref().unwrap().key == QName(b"name"))
                    {
                        let unwrap = attrib.unwrap();
                        let str = String::from_utf8_lossy(&unwrap.value);

                        target_name = str.to_string();
                    } else {
                        panic!("No class name");
                    }
                }
                b"int" => {
                    if let Some(attrib) = e
                        .attributes()
                        .find(|v| v.as_ref().unwrap().key == QName(b"name"))
                    {
                        let unwrap = attrib.unwrap();
                        let str = String::from_utf8_lossy(&unwrap.value);

                        target_name = str.to_string();
                    } else {
                        panic!("No class name");
                    }
                }
                _ => (),
            },
            Ok(Event::Text(e)) => txt = e.unescape().unwrap().into_owned(),
            Ok(Event::End(e)) => match e.name().as_ref() {
                b"X" => {
                    tmp_vec3.x = txt.parse().unwrap();
                }
                b"Y" => {
                    tmp_vec3.y = txt.parse().unwrap();
                }
                b"Z" => {
                    tmp_vec3.z = txt.parse().unwrap();
                }
                b"R00" => {
                    tmp_mat3.x_axis.x = txt.parse().unwrap();
                }
                b"R01" => {
                    tmp_mat3.x_axis.y = txt.parse().unwrap();
                }
                b"R02" => {
                    tmp_mat3.x_axis.z = txt.parse().unwrap();
                }
                b"R10" => {
                    tmp_mat3.y_axis.x = txt.parse().unwrap();
                }
                b"R11" => {
                    tmp_mat3.y_axis.y = txt.parse().unwrap();
                }
                b"R12" => {
                    tmp_mat3.y_axis.z = txt.parse().unwrap();
                }
                b"R20" => {
                    tmp_mat3.z_axis.x = txt.parse().unwrap();
                }
                b"R21" => {
                    tmp_mat3.z_axis.y = txt.parse().unwrap();
                }
                b"R22" => {
                    tmp_mat3.z_axis.z = txt.parse().unwrap();
                }
                b"Vector3" => {
                    match target_name.as_str() {
                        "size" => {
                            if let Ok(mut part) =
                                world.query::<&mut Part>().get_mut(world, instance)
                            {
                                part.size = tmp_vec3;
                            } else {
                                warn!("Vector3 size on non Part");
                            }
                        }
                        _ => warn!("Unknown name {}", target_name),
                    };
                }
                b"CoordinateFrame" => match target_name.as_str() {
                    "CFrame" => {
                        if let Ok(mut transform) = world
                            .query_filtered::<&mut Transform, With<Part>>()
                            .get_mut(world, instance)
                        {
                            transform.translation = tmp_vec3;
                            transform.rotation = Quat::from_mat3(&tmp_mat3);
                            info!("Vec: {}", transform.translation);
                        } else {
                            warn!("CFrame size on non Part");
                        }
                    }
                    _ => warn!("Unknown name {}", target_name),
                },
                b"int" => match target_name.as_str() {
                    "BrickColor" => {
                        if let Ok(mut part) = world.query::<&mut Part>().get_mut(world, instance) {
                            part.brickColor = txt.parse().unwrap();
                        }
                    }
                    _ => warn!("Unknown name {}", target_name),
                },

                b"Item" => {
                    println!("End on {:?}", e);
                    let mut instances = world.query::<&mut Instance>();
                    if let Ok(component) = instances.get(world, instance) {
                        info!("Parenting {}", component.class);
                        if let Some(parent) = component.parent {
                            if let Ok(mut parent_component) = instances.get_mut(world, parent) {
                                parent_component.children.push(instance);
                                println!("Parent");
                                instance = parent;
                            }
                        } else {
                            warn!("Instance has no parent");
                            break;
                        }
                    } else {
                        warn!("Could not get instance");
                    }
                }
                _ => (),
            },
            _ => (),
        }
        buf.clear();
    }
}
