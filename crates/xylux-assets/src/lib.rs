use xylux_core::Vertex;
use glam::Vec3;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct AssetManager {
    base_path: PathBuf,
}

impl AssetManager {
    pub fn new() -> Self {
        // Automatically find assets directory by searching upwards
        let mut path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let mut base_path = PathBuf::from("assets");

        for _ in 0..5 {
            if path.join("assets").exists() {
                base_path = path.join("assets");
                break;
            }
            if let Some(parent) = path.parent() {
                path = parent.to_path_buf();
            } else {
                break;
            }
        }

        Self { base_path }
    }

    pub fn resolve_path(&self, relative_path: &str) -> PathBuf {
        // If the relative_path already contains "assets/", strip it for better resolution
        let clean_path = if relative_path.starts_with("assets/") {
            &relative_path[7..]
        } else {
            relative_path
        };
        self.base_path.join(clean_path)
    }

    pub fn load_model(&self, relative_path: &str) -> Result<Vec<(String, Vec<Vertex>)>, Box<dyn std::error::Error>> {
        let full_path = self.resolve_path(relative_path);
        if !full_path.exists() {
             return Err(format!("Asset not found at: {:?}", full_path).into());
        }

        let extension = full_path.extension().and_then(|e| e.to_str()).unwrap_or("");
        match extension {
            "blend" => self.load_blend(&full_path),
            "obj" => self.load_obj(&full_path).map_err(|e| e.into()),
            _ => Err(format!("Unsupported model format: {}", extension).into()),
        }
    }

    fn load_obj(&self, path: &Path) -> Result<Vec<(String, Vec<Vertex>)>, std::io::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut positions = Vec::new();
        let mut normals = Vec::new(); 
        let mut named_meshes: Vec<(String, Vec<Vertex>)> = Vec::new();
        
        let mut current_name = "Object".to_string();
        let mut current_vertices = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let mut parts = line.split_whitespace();
            
            match parts.next() {
                Some("o") | Some("g") => {
                    if !current_vertices.is_empty() {
                        named_meshes.push((current_name.clone(), current_vertices));
                        current_vertices = Vec::new();
                    }
                    if let Some(name) = parts.next() {
                        current_name = name.to_string();
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
                    let face_parts: Vec<&str> = parts.collect();
                    let triangle_indices = if face_parts.len() == 4 {
                        vec![0, 1, 2, 0, 2, 3] 
                    } else {
                        vec![0, 1, 2]
                    };

                    for &i in &triangle_indices {
                        let face_str = face_parts[i];
                        let mut indices = face_str.split('/');
                        
                        let v_idx: usize = indices.next().unwrap().parse().unwrap();
                        let _vt_idx = indices.next(); 
                        let vn_idx_opt = indices.next();

                        let pos = positions[v_idx - 1];
                        let normal = if let Some(vn_str) = vn_idx_opt {
                            if !vn_str.is_empty() {
                                let vn_idx: usize = vn_str.parse().unwrap();
                                normals[vn_idx - 1]
                            } else { Vec3::Y }
                        } else { Vec3::Y };
                        
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
        if !current_vertices.is_empty() {
            named_meshes.push((current_name, current_vertices));
        }
        Ok(named_meshes)
    }

    fn load_blend(&self, path: &Path) -> Result<Vec<(String, Vec<Vertex>)>, Box<dyn std::error::Error>> {
        let mut file = File::open(path)?;
        let mut header = [0u8; 4];
        if file.read(&mut header).unwrap_or(0) >= 4 {
            if header == [0x28, 0xB5, 0x2F, 0xFD] {
                let file = File::open(path)?;
                let mut decoder = zstd::stream::Decoder::new(file)?;
                let mut temp_file = tempfile::NamedTempFile::new()?;
                std::io::copy(&mut decoder, &mut temp_file)?;
                temp_file.flush()?;
                
                // For now, native extraction is hard, fallback to CLI always for blend
            }
        }
        self.convert_blend_to_obj_and_load(path)
    }

    fn convert_blend_to_obj_and_load(&self, path: &Path) -> Result<Vec<(String, Vec<Vertex>)>, Box<dyn std::error::Error>> {
        let temp_dir = tempfile::tempdir()?;
        let output_obj_path = temp_dir.path().join("exported.obj");
        let script_content = r#"
import bpy
import sys
argv = sys.argv
if "--" in argv: argv = argv[argv.index("--") + 1:]
else: argv = []
output_path = argv[0]
try: bpy.ops.wm.obj_export(filepath=output_path, export_selected_objects=False)
except AttributeError: bpy.ops.export_scene.obj(filepath=output_path, use_selection=False)
"#;
        let script_path = temp_dir.path().join("export_script.py");
        let mut script_file = File::create(&script_path)?;
        script_file.write_all(script_content.as_bytes())?;
        
        let status = Command::new("blender")
            .arg("-b")
            .arg(path)
            .arg("-P")
            .arg(&script_path)
            .arg("--")
            .arg(&output_obj_path)
            .output()?;
            
        if !status.status.success() {
            return Err("Blender CLI failed.".into());
        }
        self.load_obj(&output_obj_path).map_err(|e| e.into())
    }
}
