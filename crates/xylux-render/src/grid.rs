use crate::vertex::Vertex;
use glam::Vec3;

pub fn generate_grid(size: f32, step: f32) -> Vec<Vertex> {
    let mut vertices = Vec::new();
    let half_size = size / 2.0;
    
    // Color grid: Gris oscuro
    let grid_color = Vec3::new(0.4, 0.4, 0.4);
    // Color axis X: Rojo
    let x_axis_color = Vec3::new(0.8, 0.2, 0.2);
    // Color axis Z: Verde
    let z_axis_color = Vec3::new(0.2, 0.8, 0.2);
    
    let line_width = 0.02;

    let num_lines = (size / step) as i32;

    for i in -num_lines..=num_lines {
        let pos = i as f32 * step;
        
        // Determinar color (Axis highlight)
        let color_x = if i == 0 { z_axis_color } else { grid_color };
        let color_z = if i == 0 { x_axis_color } else { grid_color };

        // Líneas paralelas al eje X (variable Z constante)
        // Creamos un quad delgado para simular la línea
        add_line(&mut vertices, 
            Vec3::new(-half_size, 0.0, pos), 
            Vec3::new(half_size, 0.0, pos), 
            line_width, 
            color_z
        );

        // Líneas paralelas al eje Z (variable X constante)
        add_line(&mut vertices, 
            Vec3::new(pos, 0.0, -half_size), 
            Vec3::new(pos, 0.0, half_size), 
            line_width, 
            color_x
        );
    }

    vertices
}

fn add_line(vertices: &mut Vec<Vertex>, start: Vec3, end: Vec3, width: f32, color: Vec3) {
    let dir = (end - start).normalize();
    // Perpendicular en el plano XZ
    let perp = Vec3::new(-dir.z, 0.0, dir.x) * width * 0.5;
    let normal = Vec3::Y; // Grid plano apunta arriba

    let v1 = Vertex { pos: start - perp, color, normal };
    let v2 = Vertex { pos: start + perp, color, normal };
    let v3 = Vertex { pos: end - perp, color, normal };
    let v4 = Vertex { pos: end + perp, color, normal };

    // Triangulo 1
    vertices.push(v1);
    vertices.push(v2);
    vertices.push(v3);

    // Triangulo 2
    vertices.push(v2);
    vertices.push(v4);
    vertices.push(v3);
}
