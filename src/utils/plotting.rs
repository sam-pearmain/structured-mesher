#![allow(dead_code)]

use plotters::prelude::*;
use crate::geometry::prelude::*;
use crate::mesh::nodes::*;

pub fn plot_vertices_2d(vertices: &Vertices<Point2D>, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    // create drawing area
    let root = BitMapBackend::new(filename, (2560, 1440)).into_drawing_area();
    root.fill(&WHITE)?;

    // find the bounds of the vertices
    let mut min_x = f64::MAX;
    let mut max_x = f64::MIN;
    let mut min_y = f64::MAX;
    let mut max_y = f64::MIN;

    for vertex in vertices.vertices() {
        min_x = min_x.min(vertex.get_x());
        max_x = max_x.max(vertex.get_x());
        min_y = min_y.min(vertex.get_y());
        max_y = max_y.max(vertex.get_y());
    }

    let padding = 0.1 * ((max_x - min_x).max(max_y - min_y));
    min_x -= padding;
    max_x += padding;
    min_y -= padding;
    max_y += padding;

    // create the chart
    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(min_x..max_x, min_y..max_y)?;

    chart.configure_mesh().draw()?;

    // plot vertices as points
    for vertex in vertices.vertices() {
        chart.draw_series(PointSeries::of_element(
            vec![(vertex.get_x(), vertex.get_y())],
            1,
            &BLACK,
            &|coord, size, style| {
                EmptyElement::at(coord)
                    + Circle::new((0, 0), size, style.filled())
            },
        ))?;
    }

    root.present()?;
    Ok(())
}

pub fn plot_vertices_3d(vertices: &Vertices<Point3D>, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    // create drawing area
    let root = BitMapBackend::new(filename, (2560, 1440)).into_drawing_area();
    root.fill(&WHITE)?;

    // find the bounds of the vertices
    let mut min_x = f64::MAX;
    let mut max_x = f64::MIN;
    let mut min_y = f64::MAX;
    let mut max_y = f64::MIN;
    let mut min_z = f64::MAX;
    let mut max_z = f64::MIN;

    for vertex in vertices.vertices() {
        min_x = min_x.min(vertex.get_x());
        max_x = max_x.max(vertex.get_x());
        min_y = min_y.min(vertex.get_y());
        max_y = max_y.max(vertex.get_y());
        min_z = min_z.min(vertex.get_z());
        max_z = max_z.max(vertex.get_z());
    }

    let padding = 0.1 * ((max_x - min_x).max(max_y - min_y).max(max_z - min_z));
    min_x -= padding;
    max_x += padding;
    min_y -= padding;
    max_y += padding;
    min_z -= padding;
    max_z += padding;

    // create the chart with 3D coordinate system
    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .caption("3D Vertices", ("sans-serif", 30))
        .build_cartesian_3d(min_x..max_x, min_y..max_y, min_z..max_z)?;

    // Configure the 3D axes
    chart.configure_axes()
        .light_grid_style(BLACK.mix(0.15))
        .max_light_lines(3)
        .draw()?;

    // plot vertices as points with labels
    for vertex in vertices.vertices() {
        chart.draw_series(PointSeries::of_element(
            vec![(vertex.get_x(), vertex.get_y(), vertex.get_z())],
            5,
            &BLACK,
            &|coord, size, style| {
                EmptyElement::at(coord)
                    + Circle::new((0, 0), size, style.filled())
                    + Text::new(
                        format!("{}", vertex.get_id()),
                        (10, 0),
                        ("sans-serif", 15),
                    )
            },
        ))?;
    }

    root.present()?;
    Ok(())
}

pub fn plot_nodes_2d(nodes: &Nodes<'_, Point2D>, filename: &str, draw_numbers: bool) -> Result<(), Box<dyn std::error::Error>> {
    // find bounds first
    let mut min_x = f64::MAX;
    let mut max_x = f64::MIN;
    let mut min_y = f64::MAX;
    let mut max_y = f64::MIN;

    for node in &nodes.nodes {
        for vertex in [
            node.north_face.start, node.north_face.end,
            node.south_face.start, node.south_face.end,
            node.east_face.start, node.east_face.end,
            node.west_face.start, node.west_face.end,
        ] {
            min_x = min_x.min(vertex.get_x());
            max_x = max_x.max(vertex.get_x());
            min_y = min_y.min(vertex.get_y());
            max_y = max_y.max(vertex.get_y());
        }
    }

    // calculate padding
    let padding = 0.01 * ((max_x - min_x).max(max_y - min_y));
    min_x -= padding;
    max_x += padding;
    min_y -= padding;
    max_y += padding;

    // calculate aspect ratio and image dimensions
    let mesh_width = max_x - min_x;
    let mesh_height = max_y - min_y;
    let aspect_ratio = mesh_width / mesh_height;
    
    // base image size on width = 2560, adjust height to match aspect ratio
    let width = 2560u32;
    let height = (width as f64 / aspect_ratio) as u32;

    // create drawing area with calculated dimensions
    let root = BitMapBackend::new(filename, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    // create chart with minimal decorations
    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(min_x..max_x, min_y..max_y)?;

    // configure mesh with minimal decorations
    chart.configure_mesh()
        .disable_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .axis_style(ShapeStyle::from(&WHITE.mix(0.0))) // Hide axes
        .draw()?;

    for node in &nodes.nodes {
        // get a slice of the four lines comprising the current node
        let lines: [(&Vertex<Point2D>, &Vertex<Point2D>); 4] = [
            (node.north_face.start, node.north_face.end),
            (node.south_face.start, node.south_face.end),
            (node.east_face.start, node.east_face.end),
            (node.west_face.start, node.west_face.end),
        ];

        for (start, end) in lines {
            // iterate over the slice of lines and draw them
            chart.draw_series(LineSeries::new(
                vec![
                    (start.get_x(), start.get_y()),
                    (end.get_x(), end.get_y()),
                ],
                &BLACK,
            ))?;
        }

        if draw_numbers {
            let center_x = (node.north_face.start.get_x() + node.south_face.end.get_x()) / 2.0;
            let center_y = (node.north_face.start.get_y() + node.south_face.end.get_y()) / 2.0;
            
            chart.draw_series(PointSeries::of_element(
                vec![(center_x, center_y)],
                1,
                &RED,
                &|coord, _size, _style| {
                    Text::new(
                        format!("{}", node.id),
                        coord,
                        ("sans-serif", 15).into_font().color(&BLACK),
                    )
                },
            ))?;
        }
    }

    root.present()?;
    Ok(())
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2d_vertex_plot() {
        let mut vertices = Vertices::new_2d(10, 10);
        vertices.populate_uniform();
        plot_vertices_2d(&vertices, "2d-uniform.png").expect("didn't work");
    }

    #[test]
    fn test_3d_vertex_plot() {
        let mut vertices = Vertices::new_3d(5, 5, 5);
        vertices.populate_uniform();
        plot_vertices_3d(&vertices, "3d-uniform.png").expect("didn't work");
    }

    #[test]
    fn test_2d_node_plot() {
        let mut vertices = Vertices::new_2d(100, 100);
        vertices.populate_uniform();
        
        let mut nodes = Nodes::new_2d();
        nodes.populate(&vertices).expect("failed to populate nodes");
        
        plot_nodes_2d(&nodes, "2d-nodes.png", false).expect("failed to plot nodes");
    }
}