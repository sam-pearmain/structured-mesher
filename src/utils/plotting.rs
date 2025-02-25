#![allow(dead_code)]

use plotters::prelude::*;
use crate::geometry::prelude::*;

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
            3,
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
}