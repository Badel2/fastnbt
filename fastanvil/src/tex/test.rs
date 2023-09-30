use super::*;
use serde_json;

fn cube_model() -> Model {
    serde_json::from_str(
        r##"
        {
            "parent": "block/block",
            "elements": [
                {   "from": [ 0, 0, 0 ],
                    "to": [ 16, 16, 16 ],
                    "faces": {
                        "down":  { "texture": "#down", "cullface": "down" },
                        "up":    { "texture": "#up", "cullface": "up" },
                        "north": { "texture": "#north", "cullface": "north" },
                        "south": { "texture": "#south", "cullface": "south" },
                        "west":  { "texture": "#west", "cullface": "west" },
                        "east":  { "texture": "#east", "cullface": "east" }
                    }
                }
            ]
        }
        "##,
    )
    .unwrap()
}

fn acacia_stairs_model() -> Model {
    serde_json::from_str(
        r#"
    {
        "parent": "minecraft:block/stairs",
        "textures": {
          "bottom": "minecraft:block/acacia_planks",
          "top": "minecraft:block/acacia_planks",
          "side": "minecraft:block/acacia_planks"
        }
      }"#,
    )
    .unwrap()
}

fn stairs_model() -> Model {
    serde_json::from_str(
        r##"
        {   "parent": "block/block",
            "display": {
                "gui": {
                    "rotation": [ 30, 135, 0 ],
                    "translation": [ 0, 0, 0],
                    "scale":[ 0.625, 0.625, 0.625 ]
                },
                "head": {
                    "rotation": [ 0, -90, 0 ],
                    "translation": [ 0, 0, 0 ],
                    "scale": [ 1, 1, 1 ]
                },
                "thirdperson_lefthand": {
                    "rotation": [ 75, -135, 0 ],
                    "translation": [ 0, 2.5, 0],
                    "scale": [ 0.375, 0.375, 0.375 ]
                }
            },
            "textures": {
                "particle": "#side"
            },
            "elements": [
                {   "from": [ 0, 0, 0 ],
                    "to": [ 16, 8, 16 ],
                    "faces": {
                        "down":  { "uv": [ 0, 0, 16, 16 ], "texture": "#bottom", "cullface": "down" },
                        "up":    { "uv": [ 0, 0, 16, 16 ], "texture": "#top" },
                        "north": { "uv": [ 0, 8, 16, 16 ], "texture": "#side", "cullface": "north" },
                        "south": { "uv": [ 0, 8, 16, 16 ], "texture": "#side", "cullface": "south" },
                        "west":  { "uv": [ 0, 8, 16, 16 ], "texture": "#side", "cullface": "west" },
                        "east":  { "uv": [ 0, 8, 16, 16 ], "texture": "#side", "cullface": "east" }
                    }
                },
                {   "from": [ 8, 8, 0 ],
                    "to": [ 16, 16, 16 ],
                    "faces": {
                        "up":    { "uv": [ 8, 0, 16, 16 ], "texture": "#top", "cullface": "up" },
                        "north": { "uv": [ 0, 0,  8,  8 ], "texture": "#side", "cullface": "north" },
                        "south": { "uv": [ 8, 0, 16,  8 ], "texture": "#side", "cullface": "south" },
                        "west":  { "uv": [ 0, 0, 16,  8 ], "texture": "#side" },
                        "east":  { "uv": [ 0, 0, 16,  8 ], "texture": "#side", "cullface": "east" }
                    }
                }
            ]
        }
        "##,
    )
    .unwrap()
}

fn block_model() -> Model {
    serde_json::from_str(
        r#"
        {
            "gui_light": "side",
            "display": {
                "gui": {
                    "rotation": [ 30, 225, 0 ],
                    "translation": [ 0, 0, 0],
                    "scale":[ 0.625, 0.625, 0.625 ]
                },
                "ground": {
                    "rotation": [ 0, 0, 0 ],
                    "translation": [ 0, 3, 0],
                    "scale":[ 0.25, 0.25, 0.25 ]
                },
                "fixed": {
                    "rotation": [ 0, 0, 0 ],
                    "translation": [ 0, 0, 0],
                    "scale":[ 0.5, 0.5, 0.5 ]
                },
                "thirdperson_righthand": {
                    "rotation": [ 75, 45, 0 ],
                    "translation": [ 0, 2.5, 0],
                    "scale": [ 0.375, 0.375, 0.375 ]
                },
                "firstperson_righthand": {
                    "rotation": [ 0, 45, 0 ],
                    "translation": [ 0, 0, 0 ],
                    "scale": [ 0.40, 0.40, 0.40 ]
                },
                "firstperson_lefthand": {
                    "rotation": [ 0, 225, 0 ],
                    "translation": [ 0, 0, 0 ],
                    "scale": [ 0.40, 0.40, 0.40 ]
                }
            }
        }        
        "#,
    )
    .unwrap()
}

fn cube_all_model() -> Model {
    serde_json::from_str(
        r##"
        {
            "parent": "block/cube",
            "textures": {
                "particle": "#all",
                "down": "#all",
                "up": "#all",
                "north": "#all",
                "east": "#all",
                "south": "#all",
                "west": "#all"
            }
        }   
        "##,
    )
    .unwrap()
}

fn cobblestone_model() -> Model {
    serde_json::from_str(
        r#"
        {
            "parent": "minecraft:block/cube_all",
            "textures": {
                "all": "minecraft:block/cobblestone"
            }
        }
        "#,
    )
    .unwrap()
}

fn cobblestone_blockstate() -> Blockstate {
    serde_json::from_str(
        r#"
        {
            "variants": {
            "": {
                "model": "minecraft:block/cobblestone"
            }
            }
        }
        "#,
    )
    .unwrap()
}

fn cobblestone_texture() -> Texture {
    vec![1, 2, 3]
}

fn acacia_planks_texture() -> Texture {
    vec![1, 2, 3]
}

fn cobblestone_renderer() -> Renderer {
    let blockstates = vec![("minecraft:cobblestone".to_owned(), cobblestone_blockstate())]
        .into_iter()
        .collect();

    let models = vec![
        (
            "minecraft:block/cobblestone".to_owned(),
            cobblestone_model(),
        ),
        ("block/cube".to_owned(), cube_model()),
        ("block/block".to_owned(), block_model()),
        ("minecraft:block/cube_all".to_owned(), cube_all_model()),
    ]
    .into_iter()
    .collect();

    let textures = vec![(
        "minecraft:block/cobblestone".to_owned(),
        cobblestone_texture(),
    )]
    .into_iter()
    .collect();

    Renderer::new(blockstates, models, textures)
}

fn acacia_stairs_blockstate() -> Blockstate {
    serde_json::from_str(include_str!(
        "../../resources/assets/blockstates/acacia_stairs.json"
    ))
    .unwrap()
}

fn acacia_stairs_renderer() -> Renderer {
    let blockstates = vec![(
        "minecraft:acacia_stairs".to_owned(),
        acacia_stairs_blockstate(),
    )]
    .into_iter()
    .collect();

    let models = vec![
        (
            "minecraft:block/cobblestone".to_owned(),
            cobblestone_model(),
        ),
        (
            "minecraft:block/acacia_stairs".to_owned(),
            acacia_stairs_model(),
        ),
        ("minecraft:block/stairs".to_owned(), stairs_model()),
        ("block/cube".to_owned(), cube_model()),
        ("block/block".to_owned(), block_model()),
        ("minecraft:block/cube_all".to_owned(), cube_all_model()),
    ]
    .into_iter()
    .collect();

    let textures = vec![(
        "minecraft:block/acacia_planks".to_owned(),
        cobblestone_texture(),
    )]
    .into_iter()
    .collect();

    Renderer::new(blockstates, models, textures)
}

#[test]
fn cobblestone() {
    let mut renderer = cobblestone_renderer();
    let tex = renderer.get_top("minecraft:cobblestone", "").unwrap();

    assert_eq!(tex, cobblestone_texture());
}
#[test]
fn flatten_cobblestone_model_to_cube_generic() {
    let renderer = cobblestone_renderer();
    let model = renderer
        .flatten_model("minecraft:block/cobblestone")
        .unwrap();

    let textures = model.textures.unwrap();
    assert_eq!("minecraft:block/cobblestone", textures["up"]);
    assert_eq!("minecraft:block/cobblestone", textures["down"]);
    assert_eq!("minecraft:block/cobblestone", textures["north"]);
    assert_eq!("minecraft:block/cobblestone", textures["south"]);
    assert_eq!("minecraft:block/cobblestone", textures["west"]);
    assert_eq!("minecraft:block/cobblestone", textures["east"]);
    assert_eq!("#up", model.elements.unwrap()[0].faces["up"].texture)
}

#[test]
fn stairs() {
    let mut renderer = acacia_stairs_renderer();

    let tex = renderer
        .get_top(
            "minecraft:acacia_stairs",
            "facing=east,half=top,shape=straight",
        )
        .unwrap();

    assert_eq!(tex, acacia_planks_texture());
}
