// shader.vert
#version 450

layout(location=0) in vec2 src_ul;
layout(location=1) in vec2 src_lr;
layout(location=2) in vec2 dst_ul;
layout(location=3) in vec2 dst_lr;
layout(location=4) in float rotate_theta;

layout(location=0) out vec2 v_tex_coords;

const vec2 positions[4] = vec2[4](
    vec2(0.0, 0.0),
    vec2(1.0, 0.0),
    vec2(1.0, 1.0),
    vec2(0.0, 1.0)
);

const uint indices[6] = uint[6](
    0, 3, 2,
    0, 2, 1
);

// matrix to multiply to get wgpu coordinates
const mat3 to_wgpu = mat3(
    2.0, 0.0, 0.0,
    0.0, -2.0, 0.0,
    -1.0, 1.0, 1.0
);

mat3 translation_matrix(vec2 dxdy) {
    // NOTE: the first row actually is the first column
    return mat3(
        1.0, 0.0, 0.0,
        0.0, 1.0, 0.0,
        dxdy, 1.0
    );
}

// rotates theta radians clockwise around origin
mat3 rotation_matrix_around_origin(float theta) {
    return mat3(
        cos(theta), sin(theta), 0.0,
        -sin(theta), cos(theta), 0.0,
        0.0, 0.0, 1.0
    );
}

const mat3 normalized_basis = mat3(
    0.0, 0.0, 1.0,
    1.0, 1.0, 1.0,
    1.0, 0.0, 1.0
);

void main() {
    // ---------------
    // Define some useful matrices for the
    // requested transformation
    // ---------------

    mat3 src_basis = mat3(
        vec3(src_ul, 1.0),
        vec3(src_lr, 1.0),
        vec3(src_lr[0], src_ul[1], 1.0)
    );

    mat3 dst_basis = mat3(
        vec3(dst_ul, 1.0),
        vec3(dst_lr, 1.0),
        vec3(dst_lr[0], dst_ul[1], 1.0)
    );

    // get matrix to turn normalized coordinates to cropped location
    // on the texture
    mat3 normalized_to_src = src_basis * inverse(normalized_basis);

    // matrix that converts cropped source coordinates to destination rect coordinates
    mat3 normalized_to_dst = dst_basis * inverse(normalized_basis);

    vec2 dst_center = (dst_ul + dst_lr) / 2.0;
    mat3 dst_center_to_origin = translation_matrix(-dst_center);
    mat3 origin_to_dst_center = translation_matrix(dst_center);
    mat3 rotate_around_origin = rotation_matrix_around_origin(rotate_theta);
    mat3 rotate_around_dst_center =
        origin_to_dst_center *
        rotate_around_origin *
        dst_center_to_origin;

    // ---------------
    // now compute actual coordinates
    // ---------------
    vec2 normalized_pos2 = positions[indices[gl_VertexIndex]];
    vec3 normalized_pos3 = vec3(normalized_pos2, 1.0);

    vec3 src_pos3 = normalized_to_src * normalized_pos3;
    vec3 dst_pos3 = normalized_to_dst * normalized_pos3;
    vec3 rot_pos3 = rotate_around_dst_center * dst_pos3;

    v_tex_coords = vec2(src_pos3);
    gl_Position = vec4(vec2(to_wgpu * rot_pos3), 0.0, 1.0);
}
