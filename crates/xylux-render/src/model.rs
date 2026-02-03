use crate::vertex::Vertex;
use glam::Vec3;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn load_obj<P: AsRef<Path>>(path: P) -> Result<Vec<Vertex>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut positions = Vec::new();
    let mut normals = Vec::new(); // Almacenar normales del archivo
    let mut vertices = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split_whitespace();
        
        match parts.next() {
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

                    vertices.push(Vertex { pos, color, normal });
                }
            }
            _ => {}
        }
    }

    Ok(vertices)
}
