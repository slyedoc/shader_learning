#import bevy_pbr::mesh_types
#import bevy_pbr::mesh_view_bindings

let PI: f32 = 3.14159265359;
let colorA = vec3<f32>(0.149,0.141,0.912);
let colorB = vec3<f32>(0.912,0.149,0.141);

[[group(1), binding(0)]]
var<uniform> mesh: Mesh;

struct Time {    
    time_since_startup: f32;
};

[[group(2), binding(0)]]
var<uniform> time: Time;

struct Vertex {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] uv: vec2<f32>;
};

[[stage(vertex)]]
fn vertex(vertex: Vertex) -> VertexOutput {
    let world_position = mesh.model * vec4<f32>(vertex.position, 1.0);
    var out: VertexOutput;
    out.clip_position = view.view_proj * world_position;
    out.uv = vertex.uv;
    return out;
}

// Plot a line on Y using a value between 0.0-1.0
fn plot(st: vec2<f32>, pct: f32) -> f32 {    
      return smoothStep( pct - 0.01, pct, st.y) -
          smoothStep( pct, pct + 0.01, st.y);
}

fn circle(st: vec2<f32>, radius: f32) -> f32 {
    var dist = st - vec2<f32>(0.5);
	return  1.0 - smoothStep(radius - (radius * 0.01),
                         radius + (radius * 0.01),
                         dot(dist, dist) * 4.0);
}


fn box(st: vec2<f32>, size_in: vec2<f32>, smooth_edges: f32) -> f32 {
    
     var size = vec2<f32>(0.5) - size_in * 0.5;
     let aa = vec2<f32>(smooth_edges * 0.5);
     var uv = smoothStep(size,
                         size + aa,
                         st);
     uv = uv * smoothStep(size,
                     size + aa,
                     vec2<f32>(1.0) - st);
    return uv.x * uv.y;
}

fn cross_hair(st: vec2<f32>, size: f32) -> f32 {
    return   box(st, vec2<f32>(size, size / 4.0), 0.1) +
            box(st, vec2<f32>(size / 4.0, size), 0.1);
}

fn rotate2d(st_in: vec2<f32>, angle: f32) -> vec2<f32> {
    var st = st_in;
    st = st - 0.5;
    st = st * mat2x2<f32>(cos(angle),-sin(angle),
                sin(angle),cos(angle));
    st = st + 0.5;
    return st;
}

fn scale(scale: vec2<f32>)  -> mat2x2<f32>{
    return  mat2x2<f32>(scale.x,0.0,
                0.0,scale.y);
}


fn tile(st_in: vec2<f32>, zoom: f32) -> vec2<f32> {
    var st = st_in * zoom;
    return fract(st);
}


fn brick_tile(st_in: vec2<f32>, zoom: f32) -> vec2<f32> {
    var st =  st_in * zoom;

    // Here is where the offset is happening
    st.x = st.x + step(1.0, st.y % 2.0) * 0.5;

    return fract(st);
}

fn random( st: vec2<f32>) -> f32 {
    return fract(sin(dot(st.xy,
                         vec2<f32>(12.9898,78.233)))*
        43758.5453123);
}

[[stage(fragment)]]
fn fragment(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    var st = in.uv.xy;    
    var t = time.time_since_startup;
 
    // bottom-left
    //pct = distance(st,vec2<f32>(0.4)) + distance(st,vec2<f32>(0.6));
    //pct = distance(st,vec2<f32>(0.4)) * distance(st,vec2<f32>(0.6));
    //pct = min(distance(st,vec2<f32>(0.4)),distance(st,vec2<f32>(0.6)));
    //pct = max(distance(st,vec2<f32>(0.4)),distance(st,vec2<f32>(0.6)));
    //pct = pow(distance(st,vec2<f32>(0.4)),distance(st,vec2<f32>(0.6)));
    // The multiplication of left*bottom will be similar to the logical AND.
    
    // Plot a line
    // let color = vec3<f32>(y);
    // let pct = plot(st, y);    
    // var color = vec3<f32>(circle(st, 0.5));
    // var c = box(st, vec2<f32>(0.5));


  // move space from the center to the vec2(0.0)
    //st = st - vec2<f32>(0.5);
    // rotate the space
    //st = st * rotate2d( sin(t)*PI ) ;

    
    // move it back to the original place
    //st = st + vec2<f32>(0.5);

    // Divide the space in 4
    //ast = rotate2d( st, sin(t)*PI ) ;
    //st = tile(st,3.);

    // Use a matrix to rotate the space 45 degrees
    //st = rotate2d(st,PI*0.25);

    var color = vec3<f32>(0.0);
    //var y = st.x % 2.0;
    st = brick_tile(st,5.0);
    
    color = vec3<f32>(box(st,vec2<f32>(0.9), 0.1));
    

    // Draw a square
    //color = color + vec3<f32>(box(st,vec2<f32>(0.7),0.01));
    // st = st + vec2<f32>(0.4);
    // color = color + vec3<f32>(circle(st,0.3));
    // st = st - vec2<f32>(0.8);    
    // color = color + vec3<f32>(circle(st,0.3));
    return vec4<f32>(color, 1.0);
}
