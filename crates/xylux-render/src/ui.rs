use crate::vertex::Vertex;
use glam::Vec3;

pub fn generate_ui_panel(items: &[String]) -> Vec<Vertex> {
    let mut vertices = Vec::new();
    let items_count = items.len();
    
    // Panel izquierdo: covers 20% of screen
    // NDC: -1.0 to 1.0 (width 2.0)
    
    let x_min = -1.0;
    let x_max = -0.6; // 20% of width
    let y_min = -1.0;
    let y_max = 1.0;
    
    let z = 0.1; // In front (0.0 to 1.0 range for Identity MVP). 0.1 allows layering in front.
    let color = Vec3::new(0.2, 0.2, 0.2); // Dark Gray
    let normal = Vec3::Z;

    // Background Panel
    let v1 = Vertex { pos: Vec3::new(x_min, y_min, z), color, normal };
    let v2 = Vertex { pos: Vec3::new(x_max, y_min, z), color, normal };
    let v3 = Vertex { pos: Vec3::new(x_max, y_max, z), color, normal };
    let v4 = Vertex { pos: Vec3::new(x_min, y_max, z), color, normal };

    // Quad (2 triangles)
    vertices.push(v1); vertices.push(v2); vertices.push(v3);
    vertices.push(v1); vertices.push(v3); vertices.push(v4);
    
    // Draw List Items
    let item_height = 0.1;
    let item_margin = 0.02;
    let item_color = Vec3::new(0.4, 0.4, 0.4); // Lighter Gray
    let item_color_highlight = Vec3::new(0.5, 0.5, 0.3); // Yellowish
    
    // Slightly closer than panel (smaller Z)
    let item_z = 0.05; 
    
    for i in 0..items_count {
        let i_f = i as f32;
        // Start from TOP (y_min = -1.0)
        // Y increases downwards
        let item_y_min = y_min + item_margin + (i_f * (item_height + item_margin));
        let item_y_max = item_y_min + item_height;
        
        let item_x_min = x_min + item_margin;
        let item_x_max = x_max - item_margin;
        
        // Simple "button"
        let c = if i == 0 { item_color_highlight } else { item_color }; // Highlight first item
        
        // v1 = Top-Left, v2 = Top-Right, v3 = Bottom-Right, v4 = Bottom-Left
        // Note: Y coordinates for pos:
        // Top is item_y_min? Wait. -1 is top.
        // item_y_min is e.g. -0.98.
        // item_y_max is e.g. -0.88.
        // So item_y_min is "Top" visually (smaller Y value).
        // item_y_max is "Bottom" visually (larger Y value).
        
        let iv1 = Vertex { pos: Vec3::new(item_x_min, item_y_min, item_z), color: c, normal };
        let iv2 = Vertex { pos: Vec3::new(item_x_max, item_y_min, item_z), color: c, normal };
        let iv3 = Vertex { pos: Vec3::new(item_x_max, item_y_max, item_z), color: c, normal };
        let iv4 = Vertex { pos: Vec3::new(item_x_min, item_y_max, item_z), color: c, normal };
        
        vertices.push(iv1); vertices.push(iv2); vertices.push(iv3);
        vertices.push(iv1); vertices.push(iv3); vertices.push(iv4);
        
        // Add Object Name Text
        // Center text in buttonish area
        let text_scale = 0.05; // 5% of whatever scale
        let text_x = item_x_min + 0.02;
        // Text origin is Top-Left of text.
        // Place it slightly down from top of button.
        let text_y = item_y_min + 0.02; 
        // Even closer
        let text_z = 0.01; 
        
        // Real name from slice
        let name = &items[i];
        let text_color = Vec3::ONE; // White text
        
        let text_verts = crate::text::generate_text_mesh(name, Vec3::new(text_x, text_y, text_z), text_scale, text_color);
        vertices.extend(text_verts);
    }

    vertices
}
