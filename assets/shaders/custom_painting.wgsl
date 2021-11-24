[[block]]
struct View {
    view_proj: mat4x4<f32>;
    projection: mat4x4<f32>;
    world_position: vec3<f32>;
};
[[group(0), binding(0)]]
var<uniform> view: View;

[[block]]
struct Mesh {
    transform: mat4x4<f32>;
};
[[group(1), binding(0)]]
var<uniform> mesh: Mesh;

struct Vertex {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] p : vec2<f32>;
};

[[stage(vertex)]]
fn vertex(vertex: Vertex) -> VertexOutput {
    let world_position = mesh.transform * vec4<f32>(vertex.position, 1.0);

    var out: VertexOutput;
    out.clip_position = view.view_proj * world_position;
    out.p = vertex.uv;
    return out;
}

[[block]]
struct CustomPaintingMaterial {
    color1: vec4<f32>;
    color2: vec4<f32>;
};
[[group(2), binding(0)]]
var<uniform> material: CustomPaintingMaterial;

[[stage(fragment)]]
fn fragment(in: VertexOutput) -> [[location(0)]] vec4<f32> {
	return material.color1;

	// vec3 col = mix( vec3(1.0,0.3,0.0), vec3(1.0,0.8,0.3), sqrt(p.y) );
	
	// float r = 0.2 + 0.1*cos( atan(q.y,q.x)*10.0 + 20.0*q.x + 1.0);
	// col *= smoothstep( r, r+0.01, length( q ) );

	// r = 0.015;
	// r += 0.002*sin(120.0*q.y);
	// r += exp(-40.0*p.y);
    // col *= 1.0 - (1.0-smoothstep(r,r+0.002, abs(q.x-0.25*sin(2.0*q.y))))*(1.0-smoothstep(0.0,0.1,q.y));
	
	// fragColor = vec4(col,1.0);
}
