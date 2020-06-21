extern crate shaderc;

use shaderc::Compiler;
use shaderc::ShaderKind;
use std::fs;
use std::path::MAIN_SEPARATOR;

fn main() {
    build_shaders();
}

fn build_shaders() {
    let mut compiler = Compiler::new().unwrap();
    build_shader(
        &mut compiler,
        &pt("src/shaders/shader.vert"),
        ShaderKind::Vertex,
    );
    build_shader(
        &mut compiler,
        &pt("src/shaders/shader.frag"),
        ShaderKind::Fragment,
    );
}

fn build_shader(compiler: &mut Compiler, path: &str, kind: ShaderKind) {
    println!("cargo:rerun-if-changed={}", path);
    let src = fs::read_to_string(path).unwrap();
    let name = path.rsplit(MAIN_SEPARATOR).next().unwrap_or(path);
    let spirv = compiler
        .compile_into_spirv(&src, kind, name, "main", None)
        .unwrap();
    let data = spirv.as_binary_u8();
    fs::write(format!("{}{}", path, ".spirv"), &data).unwrap();

    // For debugging if needed
    // println!("cargo:warning={}", format!("{}{}", path, ".spirv"));
    // println!("cargo:warning={}", format!("name = {}", name));
    // println!("cargo:warning={}", format!("path = {}", path));
}

fn pt(s: &str) -> String {
    s.replace("/", &format!("{}", MAIN_SEPARATOR))
}
