import bevy_sprite_render::mesh2d::{vertex_output::VertexOutput, view_bindings::globals};

struct FractalMaterial {
    mouse_pos: vec2f,
    zoom: f32,
    center: vec2f,
}

@group(constants::MATERIAL_BIND_GROUP) @binding(0)
var<uniform> material: FractalMaterial;

fn add_complex(z1: vec2f, z2: vec2f) -> vec2f {
    return vec2f(z1.x + z2.x, z1.y + z2.y);
}

fn square_complex(z: vec2f) -> vec2f {
    return vec2f(z.x * z.x - z.y * z.y, 2 * z.x * z.y);
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4f {
    const MAX_ITERATIONS: i32 = 200;

    var iterations = 0;
    var color = vec4f(0.0, 0.0, 0.0, 1.0);
    var z = vec2f(
            input.uv.x * material.zoom - material.center.x,
            input.uv.y * material.zoom - material.center.y,
        );
    // let c = vec2f(
    //         material.mouse_pos.x * material.zoom - material.center.x,
    //         material.mouse_pos.y * material.zoom - material.center.y,
    //     );
    let c = vec2f(
            input.uv.x * material.zoom - material.center.x,
            input.uv.y * material.zoom - material.center.y,
        );

    while iterations <= MAX_ITERATIONS {
        z = add_complex(square_complex(z), c);

        if dot(z, z) < 10 {
            iterations += 1;
        } else {
            break;
        }
    }

    let quotient = f32(iterations) / f32(MAX_ITERATIONS);
    let col = clamp(quotient, 0.0, 1.0);
    if quotient > 0.5 {
        color = vec4f(1.0, 1.0, 1.0, 0.0);
    } else {
        color = vec4f(0.0, col, col, 0.0);
    }

    return color;
}
