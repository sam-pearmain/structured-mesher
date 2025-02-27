mod geometry;
mod mesh;
mod utils;

use geometry::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create uniform mesh
    let mut vertices_uniform = Vertices::new_2d(200, 100);
    create_busemann_mesh_2d(
        &mut vertices_uniform,
        2.0,  // length in x direction
        WallDistribution::Uniform,
        inlet_contour,
        None,
    );
    utils::plotting::plot_vertices_2d(&vertices_uniform, "busemann_uniform.png")?;

    // create hyperbolic tangent mesh
    let mut vertices_tanh = Vertices::new_2d(200, 100);
    create_busemann_mesh_2d(
        &mut vertices_tanh,
        2.0,
        WallDistribution::HyperbolicTangent,
        inlet_contour,
        Some(2.5),  // beta parameter for clustering
    );
    utils::plotting::plot_vertices_2d(&vertices_tanh, "busemann_tanh.png")?;

    // create top-clustered mesh
    let mut vertices_top = Vertices::new_2d(200, 100);
    create_busemann_mesh_2d(
        &mut vertices_top,
        2.0,
        WallDistribution::TopClusteredTangent,
        inlet_contour,
        Some(3.0),  // stronger clustering near top
    );
    utils::plotting::plot_vertices_2d(&vertices_top, "busemann_top.png")?;

    Ok(())
}

enum WallDistribution {
    Uniform,
    HyperbolicTangent,
    TopClusteredTangent,
}

fn inlet_contour(x: f64) -> f64 {
    1.0 - (1.0 / 10.0 * x.powi(2))
}

fn create_busemann_mesh_2d(
    vertices: &mut Vertices<Point2D>,
    lenx: f64,
    wall_distribution: WallDistribution,
    inlet_contour: impl Fn(f64) -> f64,
    beta: Option<f64>, // controls point clustering (higher = more clustering)
) {
    let (nx, ny) = vertices.nx_ny();
    // calculate delta x (delta y will vary depending on the inlet contour function and the wall distribution)
    let dx: f64 = lenx / (nx as f64 - 1.0);

    // create starting coordinates
    let mut x = 0.0;
    let mut y = 0.0;

    // create empty vector to store vertices
    let mut vertex_id: usize = 0;

    match wall_distribution {
        WallDistribution::Uniform => {
            for _ in 0..nx {
                // calculate domain height at given x and find corresponding dy
                let leny = inlet_contour(x);
                let dy = leny / (ny as f64 - 1.0);
                
                for _ in 0..ny {
                    // create vertex at current point (x, y) and push to vertices
                    let vertex  = Vertex::new_2d(vertex_id, x, y);
                    vertices.add_vertex(vertex);

                    // increment vertex id and step by dy
                    vertex_id += 1;
                    y = y + dy;
                }
                // reset y and step by dx
                y = 0.0;
                x = x + dx;
            }
        }
        WallDistribution::HyperbolicTangent => {
            let beta = beta.unwrap_or(2.0);

            for _ in 0..nx {
                // calculate domain height at current x position
                let leny = inlet_contour(x);

                for j in 0..ny {
                    // calculate normalised coordinate eta between 0 and 1
                    let eta = j as f64 / (ny - 1) as f64;

                    // apply hyperbolic tangent distribution
                    // y(eta) = leny * (1 + tanh(beta * (eta - 0.5)) / tanh(beta / 2))
                    let tanh_term = 
                        (beta * (eta - 0.5)). tanh() / 
                        (beta * 0.5).tanh();
                    let y = leny * (1.0 + tanh_term);

                    // create vertex at current point (x, y)
                    let vertex = Vertex::new_2d(vertex_id, x, y);
                    vertices.add_vertex(vertex);

                    // increment vertex id
                    vertex_id += 1;
                }
                // step in x direction
                x = x + dx;
            }
        } 
        WallDistribution::TopClusteredTangent => {
            let beta = beta.unwrap_or(2.0);

            for _ in 0..nx {
                // calculate domain height at current x position
                let leny = inlet_contour(x);

                for j in 0..ny {
                    // calculate normalised coordinate eta between 0 and 1
                    let eta = j as f64 / (ny - 1) as f64;
                    
                    // top-boundary-clustered distribution
                    let tanh_term = (beta * eta).tanh() / beta.tanh();
                    y = leny * tanh_term;

                    // create vertex at current (x, y) and push to vertices
                    let vertex = Vertex::new_2d(vertex_id, x, y);
                    vertices.add_vertex(vertex);

                    // increment vertex id
                    vertex_id += 1;
                }
                // step in x direction
                x = x + dx;
            }
        }
    }
}
