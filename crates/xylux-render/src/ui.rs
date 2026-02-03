use crate::vertex::Vertex;
use glam::Vec3;

pub fn generate_ui_panel() -> Vec<Vertex> {
    let mut vertices = Vec::new();
    
    // Panel izquierdo: cubre 20% de la pantalla
    // Coordenadas normalizadas (NDC): -1.0 a 1.0
    
    let x_min = -1.0;
    let x_max = -0.6; // 20% del ancho (de -1 a -0.6 es 0.4 width en rango 2.0 = 20%)
    let y_min = -1.0;
    let y_max = 1.0;
    
    let z = 0.0; // En frente
    let color = Vec3::new(0.2, 0.2, 0.2); // Gris oscuro
    let normal = Vec3::Z; // Apuntando al usuario

    let v1 = Vertex { pos: Vec3::new(x_min, y_min, z), color, normal };
    let v2 = Vertex { pos: Vec3::new(x_max, y_min, z), color, normal };
    let v3 = Vertex { pos: Vec3::new(x_max, y_max, z), color, normal };
    let v4 = Vertex { pos: Vec3::new(x_min, y_max, z), color, normal };

    // Quad (2 triangulos)
    vertices.push(v1);
    vertices.push(v2);
    vertices.push(v3);

    vertices.push(v1);
    vertices.push(v3);
    vertices.push(v4);

    vertices
}
