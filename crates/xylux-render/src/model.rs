// Imports cleaned up
use crate::vertex::Vertex;
use glam::Vec3;
use std::fs::File;
use std::io::{BufRead, BufReader, Read}; // Consolidated Read
use std::path::Path;
use blend::Blend;

pub fn load_obj<P: AsRef<Path>>(path: P) -> Result<Vec<(String, Vec<Vertex>)>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut positions = Vec::new();
    let mut normals = Vec::new(); 
    
    // Result: List of (Name, Vertices)
    let mut named_meshes: Vec<(String, Vec<Vertex>)> = Vec::new();
    
    let mut current_name = "Object".to_string();
    let mut current_vertices = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split_whitespace();
        
        match parts.next() {
            Some("o") | Some("g") => {
                // New object/group found. 
                // If previous had vertices, save it.
                if !current_vertices.is_empty() {
                    named_meshes.push((current_name.clone(), current_vertices));
                    current_vertices = Vec::new();
                }
                
                if let Some(name) = parts.next() {
                    current_name = name.to_string();
                } else {
                    current_name = "Unnamed".to_string();
                }
            }
            Some("v") => {
                let x: f32 = parts.next().unwrap().parse().unwrap();
                let y: f32 = parts.next().unwrap().parse().unwrap();
                let z: f32 = parts.next().unwrap().parse().unwrap();
                positions.push(Vec3::new(x, y, z));
            }
            Some("vn") => {
                let x: f32 = parts.next().unwrap().parse().unwrap();
                let y: f32 = parts.next().unwrap().parse().unwrap();
                let z: f32 = parts.next().unwrap().parse().unwrap();
                normals.push(Vec3::new(x, y, z));
            }
            Some("f") => {
                // Soportar Triángulos (3 índices) y Quads (4 índices) triangulando
                let face_parts: Vec<&str> = parts.collect();
                let triangle_indices = if face_parts.len() == 4 {
                    vec![0, 1, 2, 0, 2, 3] // Quad -> 2 Triángulos
                } else {
                    vec![0, 1, 2] // Triángulo
                };

                for &i in &triangle_indices {
                    let face_str = face_parts[i];
                    let mut indices = face_str.split('/');
                    
                    let v_idx: usize = indices.next().unwrap().parse().unwrap();
                    let _vt_idx = indices.next(); // Ignorar textura por ahora
                    let vn_idx_opt = indices.next();

                    let pos = positions[v_idx - 1]; // 0-based
                    
                    let normal = if let Some(vn_str) = vn_idx_opt {
                        if !vn_str.is_empty() {
                            let vn_idx: usize = vn_str.parse().unwrap();
                            normals[vn_idx - 1]
                        } else {
                            Vec3::Y // Default normal
                        }
                    } else {
                        Vec3::Y // Default normal
                    };
                    
                    // Color generado basado en la posición para debug
                    let color = Vec3::new(
                        (pos.x + 1.0) * 0.5,
                        (pos.y + 1.0) * 0.5,
                        (pos.z + 1.0) * 0.5,
                    ).clamp(Vec3::ZERO, Vec3::ONE);

                    current_vertices.push(Vertex { pos, color, normal });
                }
            }
            _ => {}
        }
    }

    // Push last object
    if !current_vertices.is_empty() {
        named_meshes.push((current_name, current_vertices));
    }
    
    // If no named objects were found (e.g. format without 'o'), but vertices exist, return them as "Object"
    // This logic is handled by initializing current_name="Object" and pushing at the end.
    
    Ok(named_meshes)
}

pub fn load_blend<P: AsRef<Path>>(path: P) -> Result<Vec<(String, Vec<Vertex>)>, Box<dyn std::error::Error>> {
    let path = path.as_ref();
    
    // 1. Check for ZSTD compression header
    let mut file = File::open(path)?;
    let mut header = [0u8; 4];
    if file.read(&mut header).unwrap_or(0) >= 4 {
        if header == [0x28, 0xB5, 0x2F, 0xFD] {
            println!("Detected compressed Blend file. Decompressing...");
            
            // Re-open file to read from start
            let file = File::open(path)?;
            let mut decoder = zstd::stream::Decoder::new(file)?;
            
            // Create temp file
            let mut temp_file = tempfile::NamedTempFile::new()?;
            std::io::copy(&mut decoder, &mut temp_file)?;
            
            // IMPORTANT: Flush to ensure other file handles can see the data
            use std::io::Write;
            temp_file.flush()?;
            
            // Re-open as Blend from temp path
            let temp_path = temp_file.path();
            
            match load_blend_internal(temp_path) {
                Ok(v) => return Ok(v),
                Err(e) => {
                    println!("Direct parsing of decompressed stream failed: {}. Attempting Blender CLI conversion on original file...", e);
                    // Fallback to using the original path with Blender CLI
                    return convert_blend_to_obj_and_load(path);
                }
            }
        }
    }

    // 4. Return result or fallback for uncompressed files
    match load_blend_internal(path) {
        Ok(v) => Ok(v),
        Err(e) => {
            println!("Direct parsing failed: {}. Attempting Blender CLI conversion...", e);
            convert_blend_to_obj_and_load(path)
        }
    }
}

fn load_blend_internal(path: &Path) -> Result<Vec<(String, Vec<Vertex>)>, Box<dyn std::error::Error>> {
    let _blend_wrapper = Blend::from_path(path).map_err(|e| format!("{:?}", e))?;
    println!("Blend file parsed structurally (native).");
    Err("Native geometry extraction not implemented. Fallback to CLI required.".into())
}

use std::process::Command;
use std::io::Write;

fn convert_blend_to_obj_and_load(path: &Path) -> Result<Vec<(String, Vec<Vertex>)>, Box<dyn std::error::Error>> {
    // 1. Create a temp file for the output OBJ
    let temp_dir = tempfile::tempdir()?;
    let output_obj_path = temp_dir.path().join("exported.obj");
    
    // 2. Create a Python script for Blender
    let script_content = r#"
import bpy
import sys
import os

# Get args
argv = sys.argv
if "--" in argv:
    argv = argv[argv.index("--") + 1:]
else:
    argv = []

if len(argv) < 1:
    print("Error: No output path provided")
    sys.exit(1)

output_path = argv[0]

# Export
# For Blender 4.0+ / 5.0, use the new OBJ exporter if available, otherwise fallback
try:
    bpy.ops.wm.obj_export(filepath=output_path, export_selected_objects=False)
except AttributeError:
    # Fallback for older versions
    bpy.ops.export_scene.obj(filepath=output_path, use_selection=False)
"#;
    
    let script_path = temp_dir.path().join("export_script.py");
    let mut script_file = File::create(&script_path)?;
    script_file.write_all(script_content.as_bytes())?;
    
    // 3. Run Blender
    // blender -b <blend_file> -P <script> -- <output_obj>
    println!("Executing Blender CLI...");
    let status = Command::new("blender")
        .arg("-b")
        .arg(path)
        .arg("-P")
        .arg(&script_path)
        .arg("--")
        .arg(&output_obj_path)
        .output()?;
        
    if !status.status.success() {
        let stdout = String::from_utf8_lossy(&status.stdout);
        let stderr = String::from_utf8_lossy(&status.stderr);
        return Err(format!("Blender CLI failed.\nStdout: {}\nStderr: {}", stdout, stderr).into());
    }
    
    // 4. Load the resulting OBJ
    if output_obj_path.exists() {
        println!("Blender conversion successful. Loading OBJ...");
        load_obj(&output_obj_path).map_err(|e| e.into())
    } else {
        Err("Blender finished but output OBJ was not created.".into())
    }
}

